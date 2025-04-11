use super::{
	allocator::EARLY_PHYSICAL_ALLOCATOR, memblock::MemRegion, MemorySegment,
	PhysAddr, PAGE_SIZE,
};
use crate::{arch::x86::multiboot::G_SEGMENTS, println_serial, sync::Locked};
use alloc::collections::linked_list::LinkedList;
use core::{alloc::Layout, error::Error, ptr};

const MAX_ORDERS: usize = 32;

pub struct BuddyAllocator {
	base: usize,
	size: usize,
	min_block_size: usize,
	max_order: usize,
	free_lists: [LinkedList<usize>; MAX_ORDERS],
	map: &'static mut [usize],
}

unsafe impl Send for BuddyAllocator {}
unsafe impl Sync for BuddyAllocator {}

impl BuddyAllocator {
	/// Creates a new BuddyAllocator.
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		use core::mem::{align_of, size_of};

		let memory_segments = G_SEGMENTS.lock();

		let mut size = 0;
		for segment in memory_segments.iter() {
			size += segment.size();
		}

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

		#[allow(clippy::expect_used)]
		let bitmap_layout = Layout::from_size_align(bitmap_size, align_of::<usize>())
			.expect("Error while creating the Buddy Allocation Layout");

		let bitmap_ptr;
		{
			let mut early_alloc = EARLY_PHYSICAL_ALLOCATOR.lock();
			bitmap_ptr = unsafe {
				match early_alloc.get_mut() {
					Some(allocator) => allocator.alloc(bitmap_layout),
					None => panic!("Could not access early physical allocator"),
				}
			};
		}

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

		let base_addr = memory_segments[0].start_addr();
		const EMPTY_LIST: LinkedList<usize> = LinkedList::new();
		let mut free_lists = [EMPTY_LIST; MAX_ORDERS];

		if blocks_count > 0 {
			free_lists[max_order].push_back(base_addr);
		}

		return Self {
			base: memory_segments[0].start_addr(),
			size,
			min_block_size,
			max_order,
			free_lists,
			map,
		};
	}

	pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
		match self.find_free_block(layout) {
			Some(block_addr) => {
				return ptr::with_exposed_provenance_mut(block_addr);
			}
			None => {
				return ptr::null_mut();
			}
		}
	}

	pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
		let addr = ptr as usize;

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
	fn remove_from_free_list(&mut self, addr: usize, order: usize) {
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
        addr, order
    );
	}

	#[inline(always)]
	fn find_buddy_addr(&self, addr: usize, order: usize) -> usize {
		let block_size = self.min_block_size * (1 << order);
		let buddy_relative_addr = (addr - self.base) ^ block_size;

		return self.base + buddy_relative_addr;
	}

	/// Finds a free block of memory of the requested size.
	/// Returns Some(address) if found, None if no suitable block available.
	fn find_free_block(&mut self, layout: Layout) -> Option<usize> {
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

		return Some(block_addr);
	}

	#[inline(always)]
	fn get_block_index(&self, addr: usize) -> usize {
		return (addr - self.base) / self.min_block_size;
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

		return true;
	}
}
