use crate::sync::Locked;
use alloc::collections::linked_list::LinkedList;
use core::{
	alloc::{GlobalAlloc, Layout},
	ptr::NonNull,
};

enum SlabState {
	Empty,
	Partial,
	Full,
}

struct Slab {
	base: usize,
	size: usize,
	free_bitmap: u64,
	free_count: usize,
	object_size: usize,
}

pub struct SlabAllocator {
	slabs_full: LinkedList<Slab>,
	slabs_partial: LinkedList<Slab>,
	slabs_free: LinkedList<Slab>,
}

unsafe impl Send for SlabAllocator {}
unsafe impl Sync for SlabAllocator {}

unsafe impl GlobalAlloc for Locked<SlabAllocator> {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let mut allocator = self.lock();

		unsafe { allocator.alloc(layout) }
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		let mut allocator = self.lock();

		unsafe { allocator.dealloc(ptr, layout) }
	}
}

// Public interface
impl SlabAllocator {
	pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
		return core::ptr::without_provenance_mut(0x0);
	}

	pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {}
}

// Private interface
impl SlabAllocator {}
