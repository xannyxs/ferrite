//! Defines the kernel's global memory allocator instance.

use super::{
	buddy::BuddyAllocator, memblock::MemBlockAllocator, slab::SlabCache,
	NodePoolAllocator,
};
use crate::{memory::allocator, println_serial, sync::Locked};
use core::{
	alloc::{GlobalAlloc, Layout},
	cell::OnceCell,
	ptr,
};

const SLAB_CACHE_COUNT: usize = 8;
const CACHE_SIZES: [usize; SLAB_CACHE_COUNT] =
	[8, 16, 32, 64, 128, 256, 512, 1024];

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

static SLAB_CACHES: Locked<OnceCell<[SlabCache; SLAB_CACHE_COUNT]>> =
	Locked::new(OnceCell::new());

// 3. Define the actual GLOBAL ALLOCATOR static. This will WRAP access to the
//    KERNEL_HEAP_ALLOCATOR once initialized.
struct KernelAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: Locked<KernelAllocator> = Locked::new(KernelAllocator);

unsafe impl GlobalAlloc for Locked<KernelAllocator> {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		if layout.size() == 0 {
			return ptr::null_mut();
		}

		// TODO: If there is no cache Buddy Allocator should take over
		let index = CACHE_SIZES.iter().position(|&cache_size| {
			return cache_size >= layout.size();
		});

		match index {
			Some(index) => {
				let mut allocator = SLAB_CACHES.lock();

				match allocator.get_mut() {
					Some(caches) => {
						return unsafe { caches[index].alloc(layout) };
					}
					None => return ptr::null_mut(),
				}
			}
			None => {
				println_serial!(
					"No suitable cache found for size {}",
					layout.size()
				);
				return ptr::null_mut();
			}
		}
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		if layout.size() == 0 {
			return;
		}

		// TODO: If there is no cache Buddy Allocator should take over
		let index = CACHE_SIZES.iter().position(|&cache_size| {
			return cache_size >= layout.size();
		});

		match index {
			Some(index) => {
				let mut allocator = SLAB_CACHES.lock();

				match allocator.get_mut() {
					Some(allocator) => unsafe {
						allocator[index].dealloc(ptr, layout)
					},
					None => {
						panic!("Heap allocator not initialized yet! Cannot deallocate.")
					}
				};
			}
			None => {
				println_serial!(
					"dealloc: No suitable cache found for size {}",
					layout.size()
				);
				return;
			}
		}
	}
}

/// Initiate the Slab Caches
pub fn slab_cache_init() {
	let guard = SLAB_CACHES.lock();

	if guard.get().is_some() {
		println_serial!("Slab Cache already allocated");
		return;
	}

	guard.get_or_init(|| CACHE_SIZES.map(|size| SlabCache::new(size, 0)));
}
