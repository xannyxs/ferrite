//! Implements a physical memory allocator using the buddy system algorithm.

use super::{
	allocator::EARLY_PHYSICAL_ALLOCATOR, memblock::MemRegion,
	node_pool::NodeAllocatorWrapper, MemorySegment, PhysAddr, PAGE_SIZE,
};
use crate::{
	arch::x86::multiboot::G_SEGMENTS, collections::linked_list::LinkedList,
	memory::NodePoolAllocator, println_serial,
};
use core::{alloc::Layout, ptr};

const MAX_ORDERS: usize = 32;

/// Manages physical memory allocation using a buddy system with power-of-two
/// block sizes.
///
/// Tracks free blocks using linked lists for each size order and a bitmap
/// (`map`) to mark allocated/free status of the smallest block size
/// (`min_block_size`).
pub struct BuddyAllocator {
	base: PhysAddr,
	size: usize,
	min_block_size: usize,
	max_order: usize,
	free_lists: [LinkedList<PhysAddr, NodeAllocatorWrapper>; MAX_ORDERS],
	map: &'static mut [usize],
}

unsafe impl Send for BuddyAllocator {}
unsafe impl Sync for BuddyAllocator {}

impl BuddyAllocator {
	/// Creates and initializes a new `BuddyAllocator`.
	///
	/// Calculates the required size based on `G_SEGMENTS`, determines the
	/// necessary orders, allocates memory for the internal tracking bitmap
	/// using the `EARLY_PHYSICAL_ALLOCATOR`, and initializes the free lists
	/// with the largest initial block(s).
	///
	/// # Arguments
	///
	/// * `base`: The starting physical address of the memory region to manage.
	///
	/// # Panics
	///
	/// Panics if the early physical allocator is unavailable, fails to allocate
	/// memory for the bitmap, or if layout calculation fails. It also panics
	/// if `G_SEGMENTS` is not properly initialized or accessible.
	// NOTE: Keeping expect_used allow as panicking on init failure is common.
	#[allow(clippy::expect_used)]
	pub fn new(base: PhysAddr) -> Self {
		use core::mem::{align_of, size_of};

		let mut size = 0;
		for segment in G_SEGMENTS.lock().iter() {
			size += segment.size();
		}

		size -= base.as_usize();

		let min_block_size = PAGE_SIZE;
		let blocks_count = size / min_block_size;
		let mut max_order = 0;
		let mut temp = blocks_count;
		while temp > 1 {
			temp >>= 1;
			max_order += 1;
		}

		let bitmap_bits = blocks_count;
		let bitmap_bytes = bitmap_bits.div_ceil(8);
		let bitmap_words = bitmap_bytes.div_ceil(size_of::<usize>());
		let bitmap_size = bitmap_words * size_of::<usize>();

		let bitmap_layout =
			Layout::from_size_align(bitmap_size, align_of::<usize>())
				.expect("Error while creating the Buddy Allocation Layout");

		let bitmap_ptr: *mut u8 = unsafe {
			EARLY_PHYSICAL_ALLOCATOR
				.lock()
				.get_mut()
				.expect("Could not access early physical allocator")
				.alloc(bitmap_layout)
		};

		if bitmap_ptr.is_null() {
			panic!("Failed to allocate memory for buddy allocator bitmap");
		}

		let map = unsafe {
			core::slice::from_raw_parts_mut(
				bitmap_ptr as *mut usize,
				bitmap_words,
			)
		};
		for word in map.iter_mut() {
			*word = 0;
		}

		const EMPTY_LIST: LinkedList<PhysAddr, NodeAllocatorWrapper> =
			LinkedList::new_in(NodeAllocatorWrapper);
		let mut free_lists = [EMPTY_LIST; MAX_ORDERS];

		if blocks_count > 0 {
			free_lists[max_order].push_back(base);
		}

		Self {
			base,
			size,
			min_block_size,
			max_order,
			free_lists,
			map,
		}
	}

	/// Allocates a block of physical memory satisfying the given `layout`.
	///
	/// Finds the smallest suitable free block using the buddy system, splits
	/// larger blocks if necessary, marks the block as allocated in the bitmap,
	/// and returns a pointer to the start of the allocated block.
	///
	/// # Safety
	///
	/// The caller receives a raw pointer to physical memory which is not
	/// automatically zeroed. The caller must ensure correct usage and
	/// alignment handling if needed beyond what the `layout` specifies (though
	/// this allocator respects layout alignment).
	pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
		match self.find_free_block(layout) {
			Some(block_addr) => {
				ptr::with_exposed_provenance_mut(block_addr.as_usize())
			}
			None => ptr::null_mut(),
		}
	}

	/// Deallocates a previously allocated block of physical memory.
	///
	/// Marks the block associated with `ptr` and `layout` as free in the
	/// bitmap. Attempts to merge the freed block with its buddy if the buddy
	/// is also free, repeating the merge process for larger blocks if
	/// possible. The resulting free block (original or merged) is added to the
	/// appropriate free list.
	///
	/// # Safety
	///
	/// The caller *must* ensure that `ptr` was previously returned by a call to
	/// `alloc` on *this* allocator instance with the *exact same* `layout`.
	/// Deallocating with an incorrect `layout`, freeing the same block twice,
	/// or freeing a pointer not allocated by this allocator results in
	/// undefined behavior.
	pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
		let addr = (ptr as usize).into();

		let i = self.get_block_index(addr);
		let required_size = layout.size().max(layout.align());

		let mut order = 0;
		while self.min_block_size * (1 << order) < required_size {
			order += 1;
			if order >= MAX_ORDERS {
				panic!("Layout implies an order larger than MAX_ORDERS during dealloc");
			}
		}

		self.mark_free(i, order);

		let mut current_addr = addr;
		let mut current_order = order;

		while current_order < MAX_ORDERS - 1 {
			let buddy_addr = self.find_buddy_addr(current_addr, current_order);
			let buddy_index = self.get_block_index(buddy_addr);

			if !self.is_free(buddy_index, current_order) {
				break;
			}

			self.remove_from_free_list(buddy_addr, current_order);

			current_addr = current_addr.min(buddy_addr);
			current_order += 1;
		}
		self.free_lists[current_order].push_back(current_addr);
	}

	/// Panics on error
	fn remove_from_free_list(&mut self, addr: PhysAddr, order: usize) {
		if order >= MAX_ORDERS {
			panic!("Invalid order {} provided to remove_from_free_list", order);
		}

		let list = &mut self.free_lists[order];

		let mut cursor = list.cursor_front_mut();

		while cursor.current().is_some() {
			if let Some(value_ref) = cursor.current() {
				if *value_ref == addr {
					cursor.remove_current();
					return;
				}
			}
			cursor.move_next();
		}

		panic!(
        "Failed to remove address {:#x} from free_list[{}]: address not found in list!",
        addr.as_usize(), order
    );
	}

	#[inline(always)]
	fn find_buddy_addr(&self, addr: PhysAddr, order: usize) -> PhysAddr {
		let block_size = self.min_block_size * (1 << order);
		let buddy_relative_addr = (addr - self.base) ^ block_size;

		self.base + buddy_relative_addr
	}

	/// Finds a free block of memory of the requested size.
	/// Returns Some(address) if found, None if no suitable block available.
	fn find_free_block(&mut self, layout: Layout) -> Option<PhysAddr> {
		let mut k = 0;

		let required_size = layout.size().max(layout.align());
		while self.min_block_size * (1 << k) < required_size {
			k += 1;
		}

		if k >= MAX_ORDERS {
			return None;
		}

		let required_order = k;

		while k < MAX_ORDERS && self.free_lists[k].is_empty() {
			k += 1;
		}

		if k == MAX_ORDERS {
			return None;
		}

		let block_addr = self.free_lists[k].pop_back()?;

		while k > required_order {
			let buddy_offset = self.min_block_size * (1 << (k - 1));
			let buddy_addr = block_addr + buddy_offset;

			self.free_lists[k - 1].push_back(buddy_addr);

			k -= 1;
		}

		self.mark_allocated(block_addr, required_order);

		Some(block_addr)
	}

	#[inline(always)]
	fn get_block_index(&self, addr: PhysAddr) -> usize {
		(addr - self.base) / self.min_block_size
	}

	fn mark_allocated(&mut self, addr: PhysAddr, order: usize) {
		let i = (addr - self.base) / self.min_block_size;

		let blocks_to_mark = 1 << order;

		for j in 0..blocks_to_mark {
			let current = i + j;
			let word = current / (usize::BITS as usize);
			let bit_pos = current % (usize::BITS as usize);
			let mask = 1 << bit_pos;

			if word >= self.map.len() {
				panic!("Allocation address out of map bounds");
			}

			self.map[word] |= mask;
		}
	}

	fn mark_free(&mut self, i: usize, order: usize) {
		let blocks_to_mark = 1 << order;

		for j in 0..blocks_to_mark {
			let current = i + j;
			let word = current / (usize::BITS as usize);
			let bit_pos = current % (usize::BITS as usize);
			let mask = 1 << bit_pos;

			if word >= self.map.len() {
				panic!("Allocation address out of map bounds");
			}

			self.map[word] &= !mask;
		}
	}

	fn is_free(&self, i: usize, order: usize) -> bool {
		let blocks_to_check = 1 << order;

		for j in 0..blocks_to_check {
			let current = i + j;
			let word = current / (usize::BITS as usize);
			let bit_pos = current % (usize::BITS as usize);
			let mask = 1 << bit_pos;

			if word >= self.map.len() || self.map[word] & mask != 0 {
				return false;
			}
		}

		true
	}
}
