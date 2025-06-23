//! Provides a fixed-size pool allocator (`NodePoolAllocator`) for linked list
//! nodes and a safe wrapper (`NodeAllocatorWrapper`) implementing the
//! `Allocator` trait.

use super::{allocator::NODE_POOL_ALLOCATOR, PhysAddr, VirtAddr};
use crate::{
	collections::linked_list::Node,
	log_error,
	memory::{allocator::EARLY_PHYSICAL_ALLOCATOR, PAGE_SIZE},
	println_serial,
	sync::Locked,
};
use alloc::slice;
use core::{
	alloc::{AllocError, Allocator, Layout},
	mem::{align_of, size_of},
	ptr::{self, NonNull},
};

// --- Node Allocator Wrapper (for GlobalAlloc trait) ---

/// A zero-sized type that implements `core::alloc::Allocator`.
///
/// This acts as a safe wrapper around the global static `NODE_POOL_ALLOCATOR`,
/// handling locking internally for each allocation/deallocation request.
#[derive(Debug, Clone, Copy)]
pub struct NodeAllocatorWrapper;

unsafe impl Allocator for NodeAllocatorWrapper {
	/// Allocates memory suitable for one `Node<T>` using the global node pool.
	///
	/// Returns `Err(AllocError)` if the pool is uninitialized or allocation
	/// fails.
	///
	/// # Safety
	/// Relies on the underlying `NodePoolAllocator::alloc` being sound.
	fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		let mut guard = NODE_POOL_ALLOCATOR.lock();
		let pool_allocator = guard.get_mut().ok_or(AllocError)?;

		let ptr = unsafe { pool_allocator.alloc(layout) };

		if ptr.is_null() {
			return Err(AllocError);
		}

		let non_null_ptr = NonNull::new(ptr).ok_or(AllocError)?;
		let slice_ptr =
			NonNull::slice_from_raw_parts(non_null_ptr, layout.size());
		return Ok(slice_ptr);
	}

	/// Deallocates memory previously allocated by this allocator.
	///
	/// # Safety
	/// - `ptr` must have been previously allocated by this allocator (via
	///   `allocate`).
	/// - `layout` must match the layout used for allocation.
	/// - Relies on the underlying `NodePoolAllocator::dealloc` being sound.
	#[allow(clippy::expect_used)]
	unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
		let mut guard = NODE_POOL_ALLOCATOR.lock();
		let pool_allocator = guard.get_mut().expect(
			"NODE_POOL_ALLOCATOR accessed before deallocation/initialization",
		);

		unsafe { pool_allocator.dealloc(ptr.as_ptr(), layout) };
	}
}

// --- Node Pool Allocator (Actual Implementation) ---

/// Manages a fixed-size pool of memory suitable for `Node<T>` allocations.
///
/// Uses a bitmap (`map`) to track used/free slots within a contiguous
/// memory region starting at `base`. Designed primarily for allocating
/// `Node<T>` instances for linked lists.
#[derive(Debug)]
pub struct NodePoolAllocator {
	base: VirtAddr,
	map: &'static mut [usize],
	capacity: usize,
	// NOTE: Consider storing node_size and node_align here too.
}

impl NodePoolAllocator {
	/// Creates a new `NodePoolAllocator`.
	///
	/// Allocates the necessary bitmap from the `EARLY_PHYSICAL_ALLOCATOR`.
	/// Panics if bitmap allocation fails or base alignment is incorrect.
	///
	/// # Arguments
	/// * `base`: The starting physical address of the node storage pool. Must
	///   be aligned for `Node<usize>`.
	/// * `capacity`: The total number of `Node<usize>`-sized slots the pool
	///   should manage.
	#[allow(clippy::expect_used)]
	pub fn new(base: VirtAddr, capacity: usize) -> Self {
		use core::ptr::with_exposed_provenance_mut;

		assert!(
			base.as_usize() % align_of::<Node<usize>>() == 0,
			"Node pool base address not aligned"
		);
		assert!(capacity > 0, "Node pool capacity must be > 0");

		let bitmap_words_needed = capacity.div_ceil(usize::BITS as usize);
		let bitmap_layout = Layout::array::<usize>(bitmap_words_needed)
			.expect("Failed to create layout for bitmap");

		let bitmap_ptr = {
			let mut memblock_guard = EARLY_PHYSICAL_ALLOCATOR.lock();
			let allocator = memblock_guard.get_mut().expect(
				"EARLY_PHYSICAL_ALLOCATOR not available for NodePool bitmap",
			);

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

		println_serial!(
            "NodePoolAllocator initialized: base={:#x}, capacity={}, bitmap={:#x} ({} words)",
            base.as_usize(),
            capacity,
            bitmap_base_addr,
            bitmap_words_needed
        );

		return Self {
			base,
			map: map_slice,
			capacity,
		};
	}

	/// Allocates a single node slot from the pool. (Internal Method)
	///
	/// Checks if the requested layout matches the expected `Node<usize>`
	/// size/alignment. Finds a free slot using the bitmap, marks it allocated,
	/// and returns its raw pointer. Returns `null_mut` if the layout is
	/// incorrect or the pool is full.
	///
	/// # Safety
	/// The caller must ensure the returned pointer is used correctly according
	/// to the provided `layout` (which must match `Node<usize>`). The memory
	/// is not zeroed. Requires `&mut self` for bitmap modification.
	pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
		const NODE_SIZE: usize = size_of::<Node<usize>>();
		const NODE_ALIGN: usize = align_of::<Node<usize>>();

		if layout.size() != NODE_SIZE {
			log_error!(
                "NodePoolAllocator::alloc: Incorrect size (expected {}, got {})",
                NODE_SIZE, layout.size()
            );
			return ptr::null_mut();
		}
		if layout.align() > NODE_ALIGN {
			log_error!(
                "NodePoolAllocator::alloc: Incorrect alignment (max {}, requested {})",
                NODE_ALIGN, layout.align()
            );
			return ptr::null_mut();
		}

		match self.find_block() {
			Some(index) => {
				self.mark_allocated(index);
				let addr = self.base + (index * NODE_SIZE);

				println_serial!(
					"NodePoolAllocator::alloc: Allocated block {}, Addr: {:#x}",
					index,
					addr.as_usize()
				);

				ptr::with_exposed_provenance_mut(addr.as_usize())
			}
			None => {
				log_error!("Allocation failed - pool is full.");
				ptr::null_mut()
			}
		}
	}

	/// Deallocates a single node slot back to the pool. (Internal Method)
	///
	/// Validates the pointer and layout, calculates the slot index,
	/// and marks the corresponding bit as free in the bitmap.
	///
	/// # Safety
	/// - `ptr` must point to the start of a node slot previously allocated from
	///   *this* pool.
	/// - `layout` must match the layout used for allocation (`Node<usize>`).
	/// - Requires `&mut self` for bitmap modification.
	pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
		const NODE_SIZE: usize = size_of::<Node<usize>>();
		const NODE_ALIGN: usize = align_of::<Node<usize>>();

		if layout.size() != NODE_SIZE || layout.align() > NODE_ALIGN {
			println_serial!(
                "NodePoolAllocator::dealloc: Incorrect layout provided. Ptr={:p}", ptr
            );
			// Considering panicking on invalid deallocation layout?
			return;
		}

		let addr: VirtAddr = (ptr as usize).into();
		let base_usize = self.base.as_usize();
		let addr_usize = addr.as_usize();
		let pool_end = base_usize.saturating_add(self.capacity * NODE_SIZE);

		if addr < self.base || addr_usize >= pool_end {
			println_serial!(
                "NodePoolAllocator::dealloc: Pointer {:#x} out of pool bounds [{:#x} - {:#x}).",
                addr_usize, base_usize, pool_end
            );
			// Considering panicking on invalid pointer
			return;
		}

		let offset = addr_usize.saturating_sub(base_usize);
		if offset % NODE_SIZE != 0 {
			println_serial!(
                "NodePoolAllocator::dealloc: Pointer {:#x} not aligned to a node start within pool.",
                addr_usize
            );
			// Consider panicking
			return;
		}

		let index = offset / NODE_SIZE;
		self.mark_deallocated(index);

		println_serial!(
			"NodePoolAllocator::dealloc: Deallocated block {}, Addr: {:#x}",
			index,
			addr_usize
		);
	}

	/// (Internal) Marks the bit corresponding to `index` as allocated (1).
	/// Panics if index is out of bounds or already allocated.
	fn mark_allocated(&mut self, index: usize) {
		assert!(index < self.capacity, "mark_allocated: Index out of bounds");

		let word_index = index / (usize::BITS as usize);
		let bit_index = index % (usize::BITS as usize);
		let mask = 1 << bit_index;

		if (self.map[word_index] & mask) != 0 {
			panic!(
				"NodePoolAllocator: Double allocation detected at index {}!",
				index
			);
		}

		self.map[word_index] |= mask;
	}

	/// (Internal) Marks the bit corresponding to `index` as free (0).
	/// Panics if index is out of bounds or already free (double free).
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
                "NodePoolAllocator: Double free or freeing unallocated block detected at index {}!",
                index
            );
		}

		self.map[word_index] &= !mask;
	}

	/// (Internal) Finds the index of the first free slot (0-bit) in the bitmap.
	/// Returns `Some(index)` if found, `None` if the pool is full.
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
