use super::allocator::NODE_POOL_ALLOCATOR;
use crate::{
	collections::linked_list::Node,
	memory::{allocator::EARLY_PHYSICAL_ALLOCATOR, PAGE_SIZE},
	println_serial,
	sync::Locked,
};
use alloc::slice;
use core::{
	alloc::{AllocError, Allocator, Layout},
	ptr::NonNull,
};

pub struct NodeAllocatorWrapper;

unsafe impl Allocator for NodeAllocatorWrapper {
	#[allow(clippy::expect_used)]
	fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		let mut guard = NODE_POOL_ALLOCATOR.lock();

		let pool_allocator = guard
			.get_mut()
			.expect("NODE_POOL_ALLOCATOR accessed before initialization");

		let ptr = unsafe { pool_allocator.alloc(layout) };

		if ptr.is_null() {
			return Err(AllocError);
		}

		let non_null_ptr = NonNull::new(ptr).ok_or(AllocError)?;
		let slice_ptr =
			NonNull::slice_from_raw_parts(non_null_ptr, layout.size());
		return Ok(slice_ptr);
	}

	#[allow(clippy::expect_used)]
	unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
		let mut guard = NODE_POOL_ALLOCATOR.lock();
		let pool_allocator = guard.get_mut().expect(
			"NODE_POOL_ALLOCATOR accessed before deallocation/initialization",
		);

		unsafe { pool_allocator.dealloc(ptr.as_ptr(), layout) };
	}
}

#[derive(Debug)]
pub struct NodePoolAllocator {
	base: usize,
	map: &'static mut [usize],
	capacity: usize,
}

impl NodePoolAllocator {
	pub fn new(base: usize, capacity: usize) -> Self {
		use core::ptr::with_exposed_provenance_mut;

		assert!(
			base % align_of::<Node<usize>>() == 0,
			"Node pool base address not aligned"
		);

		let bitmap_words_needed =
			(capacity + (usize::BITS as usize) - 1) / (usize::BITS as usize);
		let bitmap_layout = Layout::array::<usize>(bitmap_words_needed)
			.expect("Failed to create layout for bitmap");

		let bitmap_ptr = {
			let mut memblock_guard = EARLY_PHYSICAL_ALLOCATOR.lock();
			let allocator =
				memblock_guard.get_mut().expect("MemBlock not available");

			unsafe { allocator.alloc(bitmap_layout) }
		};

		if bitmap_ptr.is_null() {
			panic!("NodePoolAllocator::new failed to allocate bitmap memory from MemBlock");
		}

		let bitmap_base_addr = bitmap_ptr as usize;

		let map_slice: &'static mut [usize] = unsafe {
			let slice = slice::from_raw_parts_mut(
				with_exposed_provenance_mut(bitmap_base_addr),
				bitmap_words_needed,
			);
			slice.fill(0);
			slice
		};

		return Self {
			base,
			map: map_slice,
			capacity,
		};
	}

	#[allow(clippy::expect_used)]
	pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
		use core::ptr;

		if layout.size() != size_of::<Node<usize>>() {
			println_serial!(
				"NodePoolAllocator::alloc: Incorrect size requested."
			);
			return ptr::null_mut();
		}

		if layout.align() > align_of::<Node<usize>>() {
			println_serial!(
				"NodePoolAllocator::alloc: Incorrect alignment requested."
			);
			return ptr::null_mut();
		}

		let nodes = layout.size() / size_of::<Node<usize>>();
		if nodes > self.capacity {
			return ptr::null_mut();
		}

		match self.find_block() {
			Some(i) => {
				let addr = self
					.base
					.checked_add(i * size_of::<Node<usize>>())
					.expect("NodePoolAllocator::alloc: Address overflow");

				self.mark_allocated(i);

				println_serial!(
					"NodePoolAllocator::alloc: Allocated block {}, Addr: {:#x}",
					i,
					addr
				);

				return ptr::with_exposed_provenance_mut(addr);
			}
			None => {
				println_serial!("NodePoolAllocator::alloc: Allocation failed - pool is full.");

				return ptr::null_mut();
			}
		};
	}

	pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
		if layout.size() != size_of::<Node<usize>>() {
			println_serial!(
				"NodePoolAllocator::dealloc: Incorrect size requested."
			);
			return;
		}

		if layout.align() > align_of::<Node<usize>>() {
			println_serial!(
				"NodePoolAllocator::dealloc: Incorrect alignment requested."
			);
			return;
		}

		let addr = ptr as usize;

		if self.base > addr
			|| self.base + (self.capacity * size_of::<Node<usize>>()) < addr
		{
			println_serial!(
				"NodePoolAllocator::dealloc: Pointer {:#x} out of pool bounds.",
				addr
			);

			return;
		}

		if (addr - self.base) % size_of::<Node<usize>>() != 0 {
			println_serial!("NodePoolAllocator::dealloc: Pointer {:#x} not aligned to a node start.", addr);
			return;
		}

		let offset = addr - self.base;
		let index = offset / size_of::<Node<usize>>();

		self.mark_deallocated(index);
		println_serial!(
			"NodePoolAllocator::dealloc: Deallocated block {}, Addr: {:#x}",
			index,
			addr
		);
	}

	fn mark_allocated(&mut self, index: usize) {
		assert!(
			index < self.capacity,
			"mark_allocation: Index out of bounds"
		);

		let word = index / (usize::BITS as usize);
		let bit_pos = index % (usize::BITS as usize);
		let mask = 1 << bit_pos;

		if self.map[word] & mask != 0 {
			panic!("Double alloc");
		}

		self.map[word] |= mask;
	}

	fn mark_deallocated(&mut self, index: usize) {
		assert!(
			index < self.capacity,
			"mark_deallocated: Index out of bounds"
		);

		let word_index = index / (usize::BITS as usize);
		let bit_index = index % (usize::BITS as usize);
		let mask = 1 << bit_index;

		if (self.map[word_index] & mask) == 0 {
			panic!(
            "NodePoolAllocator::mark_deallocated: Double free or freeing unallocated block detected at index {}!",
            index
        );
		}

		self.map[word_index] &= !mask;
	}

	fn find_block(&self) -> Option<usize> {
		for (word_index, &word) in self.map.iter().enumerate() {
			if word != usize::MAX {
				let bit_index = (!word).trailing_zeros() as usize;
				let block_index =
					word_index * (usize::BITS as usize) + bit_index;
				if block_index < self.capacity {
					return Some(block_index);
				}
			}
		}

		return None;
	}
}
