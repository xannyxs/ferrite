use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

extern crate alloc;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
	unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
		return null_mut();
	}

	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
		panic!("dealloc should be never called")
	}
}
