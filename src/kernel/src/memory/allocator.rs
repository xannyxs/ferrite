//! Defines the kernel's global memory allocator instance.

use super::{
	buddy::BuddyAllocator, memblock::MemBlockAllocator, slab::SlabAllocator,
	NodePoolAllocator,
};
use crate::{memory::allocator, sync::Locked};
use core::{
	alloc::{GlobalAlloc, Layout},
	cell::OnceCell,
	ptr,
};

// 1. Define static for the EARLY allocator (MemBlock) NO #[global_allocator]
//    attribute here!
#[allow(missing_docs)]
pub static EARLY_PHYSICAL_ALLOCATOR: Locked<OnceCell<MemBlockAllocator>> =
	Locked::new(OnceCell::new());

// 2. Define another static which is in charge to reserve space for the Buddy
//    Allocator meant for the `free_list`.
#[allow(missing_docs)]
pub static NODE_POOL_ALLOCATOR: Locked<OnceCell<NodePoolAllocator>> =
	Locked::new(OnceCell::new());

// 2. Define statics for the LATER allocators (Buddy + Slab) These need
//    initialization logic. Using OnceCell is one way.
#[allow(missing_docs)]
pub static BUDDY_PAGE_ALLOCATOR: Locked<OnceCell<BuddyAllocator>> =
	Locked::new(OnceCell::new());
static KERNEL_HEAP_ALLOCATOR: Locked<OnceCell<SlabAllocator>> =
	Locked::new(OnceCell::new());

// 3. Define the actual GLOBAL ALLOCATOR static. This will WRAP access to the
//    KERNEL_HEAP_ALLOCATOR once initialized.
struct KernelAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: Locked<KernelAllocator> = Locked::new(KernelAllocator);

unsafe impl GlobalAlloc for Locked<KernelAllocator> {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let mut allocator = KERNEL_HEAP_ALLOCATOR.lock();

		match allocator.get_mut() {
			Some(allocator) => unsafe { allocator.alloc(layout) },
			None => ptr::null_mut(),
		}
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		let mut allocator = KERNEL_HEAP_ALLOCATOR.lock();

		match allocator.get_mut() {
			Some(allocator) => unsafe { allocator.dealloc(ptr, layout) },
			None => {
				panic!("Heap allocator not initialized yet! Cannot deallocate.")
			}
		};
	}
}
