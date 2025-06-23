//! Defines the kernel's global memory allocator instance.

use super::{
	buddy::BuddyAllocator, memblock::MemBlockAllocator, slab::SlabCache,
	NodePoolAllocator,
};
use crate::{
	arch::x86::multiboot::{
		get_biggest_available_segment_index, get_memory_region, MultibootInfo,
		G_SEGMENTS,
	},
	collections::linked_list::Node,
	log_debug, log_info,
	memory::{
		allocator,
		frame::FRAME_ALLOCATOR,
		get_kernel_virtual_end,
		paging::{flags, map_page},
		FrameAllocator, PhysAddr, VirtAddr, NODE_POOL_VIRT_START, PAGE_SIZE,
	},
	print_serial, println_serial,
	sync::Locked,
};
use core::{
	alloc::{GlobalAlloc, Layout},
	cell::OnceCell,
	ptr,
};

const SLAB_CACHE_COUNT: usize = 9;
const CACHE_SIZES: [usize; SLAB_CACHE_COUNT] =
	[4, 8, 16, 32, 64, 128, 256, 512, 1024];

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

#[allow(clippy::implicit_return)]
#[allow(clippy::expect_used)]
unsafe impl GlobalAlloc for Locked<KernelAllocator> {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		if layout.size() == 0 {
			return ptr::null_mut();
		}

		// TODO: If there is no cache Buddy Allocator should take over
		let index = CACHE_SIZES
			.iter()
			.position(|&cache_size| cache_size >= layout.size())
			.expect("dealloc: No suitable cache found");

		match SLAB_CACHES.lock().get_mut() {
			Some(caches) => {
				let cache = caches
					.get_mut(index)
					.expect("FATAL: Slab cache out of bounds during dealloc!");

				unsafe { cache.alloc(layout) }
			}
			None => ptr::null_mut(),
		}
	}

	#[allow(clippy::implicit_return)]
	#[allow(clippy::expect_used)]
	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		// TODO: If there is no cache Buddy Allocator should take over
		let index = CACHE_SIZES
			.iter()
			.position(|&cache_size| cache_size >= layout.size())
			.expect("dealloc: No suitable cache found for size {}");

		match SLAB_CACHES.lock().get_mut() {
			Some(alloc_array) => {
				let alloc = alloc_array
					.get_mut(index)
					.expect("FATAL: Slab cache out of bounds during dealloc!");

				unsafe { alloc.dealloc(ptr, layout) };
			}
			None => {
				panic!("Heap allocator not initialized yet! Cannot deallocate.")
			}
		};
	}
}

/// Initializes the kernel's memory management system.
///
/// Sets up the early physical allocator (`MemBlockAllocator`), reserves memory
/// for and initializes the `NodePoolAllocator`, initializes the
/// `BuddyAllocator` and `SlabCache` array, and finally decommissions the early
/// allocator.
///
/// # Panics
/// Panics if memory regions cannot be found, essential allocations fail, or if
/// the early allocator fails to decommission.
#[allow(clippy::implicit_return)]
#[allow(clippy::expect_used)]
pub fn memory_init(boot_info: &MultibootInfo) {
	log_info!("Initializing Memory Allocators");

	get_memory_region(boot_info);

	{
		let mut memblock = EARLY_PHYSICAL_ALLOCATOR.lock();
		memblock.get_or_init(MemBlockAllocator::new);
		memblock
			.get_mut()
			.expect("Failed to initialize memory block allocator.")
			.init();
	}
	log_debug!("Initialized Memblock",);

	FRAME_ALLOCATOR.lock().get_or_init(FrameAllocator::new);
	FRAME_ALLOCATOR
		.lock()
		.get_mut()
		.expect("Frame Allocator not created yet")
		.init();

	log_debug!("Initialized Frame Allocator",);

	let index =
		get_biggest_available_segment_index().expect("No segment available");

	let needed_nodes = G_SEGMENTS.lock()[index].size() / PAGE_SIZE;
	let pool_layout = Layout::from_size_align(
		needed_nodes * size_of::<Node<usize>>(),
		align_of::<Node<usize>>(),
	)
	.expect("Error while creating a layout");

	let ptr = {
		let mut memblock_guard = EARLY_PHYSICAL_ALLOCATOR.lock();
		unsafe {
			memblock_guard
				.get_mut()
				.expect("MemBlock not available")
				.alloc(pool_layout)
		}
	};

	if ptr.is_null() {
		panic!("Failed to allocate node pool from MemBlock");
	}

	let kernel_end_addr = get_kernel_virtual_end();
	let node_pool_virt_start = VirtAddr::new(NODE_POOL_VIRT_START);

	log_debug!("Kernel virtual end: {:#x}", kernel_end_addr.as_usize());
	log_debug!(
		"Node Pool virtual start: {:#x}",
		node_pool_virt_start.as_usize()
	);

	log_info!(
		"Mapping Node Pool: VA {:#x} -> {:#x} (Size: {} bytes)",
		node_pool_virt_start.as_usize(),
		node_pool_virt_start.as_usize() + pool_layout.size(),
		pool_layout.size()
	);

	let mut current_vaddr = node_pool_virt_start;
	let end_vaddr = VirtAddr::new(NODE_POOL_VIRT_START + pool_layout.size());

	while current_vaddr < end_vaddr {
		let frame = FRAME_ALLOCATOR
			.lock()
			.get()
			.expect("Frame Allocator does not exist")
			.allocate_frame()
			.expect("Failed to allocate frame for node pool");

		map_page(frame, current_vaddr, flags::PRESENT | flags::WRITABLE);
		current_vaddr = VirtAddr::new(current_vaddr.as_usize() + PAGE_SIZE);
	}

	let pool_base_addr: PhysAddr = (ptr as usize).into();
	NODE_POOL_ALLOCATOR.lock().get_or_init(|| {
		NodePoolAllocator::new(node_pool_virt_start, needed_nodes)
	});

	log_debug!(
		"Initializing NodePoolAllocator at {:#x}",
		pool_base_addr.as_usize()
	);

	let base: PhysAddr = {
		let guard = EARLY_PHYSICAL_ALLOCATOR.lock();
		let memblock = guard
			.get()
			.expect("Failed to get memblock from early allocator");

		memblock
			.mem_region()
			.iter()
			.find(|&region| !region.is_empty())
			.map(|region| region.base())
			.expect("No non-empty memory regions available")
	};

	BUDDY_PAGE_ALLOCATOR
		.lock()
		.get_or_init(|| BuddyAllocator::new(base));

	log_debug!("Initialized Buddy Page Allocator",);

	SLAB_CACHES
		.lock()
		.get_or_init(|| CACHE_SIZES.map(|size| SlabCache::new(size, 0)));

	log_debug!("Initialized Slab Caches",);

	EARLY_PHYSICAL_ALLOCATOR.lock().take();
	if EARLY_PHYSICAL_ALLOCATOR.lock().get().is_some() {
		panic!(
			"EARLY_PHYSICAL_ALLOCATOR (memblock) has not been decommissioned."
		);
	}

	log_debug!("Decommissioned memblock");
	log_info!("Initialized Memory Allocators succesfully");
}
