use super::region::get_primary_memory_region;
use crate::{
	arch::x86::multiboot::{MultibootInfo, MultibootMmapEntry},
	memory::{stack::KernelStack, MemorySegment, RegionType},
	println_serial,
	sync::locked::Locked,
};
use core::{
	alloc::{GlobalAlloc, Layout},
	mem,
	ptr::{self, null_mut, NonNull},
};
use kernel_sync::{mutex::MutexGuard, Mutex};

#[derive(Debug, Default)]
pub struct ListNode {
	next: Option<NonNull<ListNode>>,
	prev: Option<NonNull<ListNode>>,
}

#[derive(Debug, Default)]
pub struct LinkedList {
	head: Option<NonNull<ListNode>>,
}

impl LinkedList {
	pub const fn new() -> Self {
		return Self {
			head: None,
		};
	}
}

#[derive(Debug, Default)]
pub struct BuddyAllocator {
	start_addr: usize,
	len: usize,
	free_list: LinkedList,
	map: &'static mut [usize],
}

unsafe impl Send for BuddyAllocator {}
unsafe impl Sync for BuddyAllocator {}

#[global_allocator]
pub static ALLOCATOR: Locked<BuddyAllocator> =
	Locked::new(BuddyAllocator::new());

unsafe impl GlobalAlloc for Locked<BuddyAllocator> {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		unsafe {
			return self.lock().alloc(layout);
		}
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		unsafe { self.lock().dealloc(ptr, layout) }
	}
}

unsafe impl GlobalAlloc for BuddyAllocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let size = layout.size();
		let align = layout.align();

		match self.find_free_block(size, align) {
			Some(block_addr) => {
				return ptr::with_exposed_provenance_mut(block_addr);
			}
			None => {
				return ptr::null_mut();
			}
		}
	}

	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
		panic!("dealloc should be never called")
	}
}

impl BuddyAllocator {
	/// Creates a new uninitialized BuddyAllocator.
	/// Must be initialized with `init` before use.
	pub const fn new() -> Self {
		return Self {
			start_addr: 0,
			len: 0,
			free_list: LinkedList::new(),
			map: &mut [],
		};
	}

	/// Initializes the allocator with memory information from the bootloader.
	pub fn init(&mut self, boot_info: &MultibootInfo) {
		let memory_segment = get_primary_memory_region(boot_info);
		self.start_addr = memory_segment.start_addr() as usize;
		self.len = memory_segment.size() as usize;
	}

	/// Finds a free block of memory of the requested size.
	/// Returns Some(address) if found, None if no suitable block available.
	pub fn find_free_block(&self, size: usize, align: usize) -> Option<usize> {
		if size > self.len {
			return None;
		}

		let addr = self.start_addr;
		let aligned_addr = (addr + align - 1) & !(align - 1);

		if aligned_addr + size <= self.start_addr + self.len {
			return Some(aligned_addr);
		}

		return None;
	}
}
