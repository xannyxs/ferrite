use super::{
	stack::{KernelStack, STACK},
	PAGE_SIZE,
};
use crate::{
	arch::x86::multiboot::{get_memory_region, MultibootInfo},
	println_serial,
	sync::locked::Locked,
};
use core::{
	alloc::{GlobalAlloc, Layout},
	ptr,
};

#[derive(Debug, Default)]
pub struct BuddyAllocator {
	start_addr: usize,
	len: usize,
	map: &'static mut [usize],
}

unsafe impl Send for BuddyAllocator {}
unsafe impl Sync for BuddyAllocator {}

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
			map: &mut [],
		};
	}

	/// Initializes the allocator with memory information from the bootloader.
	pub fn init(&mut self, boot_info: &MultibootInfo) {
		let memory_segment = get_memory_region(boot_info);
		self.start_addr = memory_segment.start_addr() as usize;
		self.len = memory_segment.size() as usize;

		let mut guarded = STACK.lock();
		let kernel_stack: &mut KernelStack = match guarded.get_mut() {
			Some(ks) => ks,
			None => {
				panic!("Attempted to use STACK before it was initialized!");
			}
		};

		let levels = (self.len / PAGE_SIZE).ilog2() as usize;
		let total_bits: usize = (1 << (levels + 1)) - 1;
		let bits_per_usize = core::mem::size_of::<usize>() * 8;
		let map_size = total_bits.div_ceil(bits_per_usize);

		self.map = unsafe {
			match kernel_stack.create_usize_array(map_size) {
				Ok(slice) => slice,
				Err(e) => {
					println_serial!("Notice: {}", e);

					let available_elements = kernel_stack.size();

					#[allow(clippy::expect_used)]
					kernel_stack
						.create_usize_array(available_elements)
						.expect("Critical: Could not allocate even minimal map")
				}
			}
		};
	}

	// fn find_buddy(size: usize) -> NonNull<ListNode> {}

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
