use super::{MemorySegment, PhysAddr, RegionType};
use crate::{
	arch::x86::multiboot::{get_memory_region, MultibootInfo},
	memory::PAGE_SIZE,
	println_serial,
	sync::locked::Locked,
};
use core::{
	alloc::{GlobalAlloc, Layout},
	fmt::Debug,
	ptr,
};
use kernel_sync::mutex::MutexGuard;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MemRegion {
	base: PhysAddr,
	size: usize,
}

impl MemRegion {
	pub const fn new(base: PhysAddr, size: usize) -> Self {
		return Self {
			base,
			size,
		};
	}

	pub fn is_empty(&self) -> bool {
		*self == MemRegion::empty()
	}

	pub const fn empty() -> Self {
		return MemRegion {
			base: 0,
			size: 0,
		};
	}
}

const MAX_REGION: usize = 64;

pub struct MemBlockAllocator {
	memory_region: [MemRegion; MAX_REGION],
	reserved_region: [MemRegion; MAX_REGION],
	memory_count: usize,
	reserved_count: usize,
}

unsafe impl Send for MemBlockAllocator {}
unsafe impl Sync for MemBlockAllocator {}

unsafe impl GlobalAlloc for Locked<MemBlockAllocator> {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let mut allocator = self.lock();

		unsafe {
			return allocator.alloc(layout);
		}
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		let mut allocator = self.lock();

		unsafe {
			return allocator.dealloc(ptr, layout);
		}
	}
}

impl MemBlockAllocator {
	pub const fn new() -> Self {
		const EMPTY: MemRegion = MemRegion::empty();

		return Self {
			memory_region: [EMPTY; MAX_REGION],
			reserved_region: [EMPTY; MAX_REGION],
			memory_count: 0,
			reserved_count: 0,
		};
	}

	pub fn init(&mut self, segments: &mut [MemorySegment; 16]) {
		for segment in segments.iter() {
			if segment.segment_type() == RegionType::Available {
				self.add(segment.start_addr(), segment.size());
			} else if segment.segment_type() == RegionType::Reserved {
				self.reserved(segment.start_addr(), segment.size());
			}
		}
	}

	pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
		match self.find_free_region(layout.size(), layout.align()) {
			Some(addr) => return ptr::with_exposed_provenance_mut(addr),
			None => return ptr::null_mut(),
		}
	}

	pub unsafe fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
		panic!("dealloc should be never called for MemBlockAllocator");
	}

	fn add(&mut self, base: PhysAddr, size: usize) -> bool {
		if self.memory_count >= MAX_REGION {
			return false;
		}

		self.memory_region[self.memory_count] = MemRegion::new(base, size);
		self.memory_count += 1;

		return true;
	}

	fn reserved(&mut self, base: PhysAddr, size: usize) -> bool {
		if self.reserved_count >= MAX_REGION {
			return false;
		}

		self.reserved_region[self.reserved_count] = MemRegion::new(base, size);
		self.reserved_count += 1;

		return true;
	}

	pub fn find_free_region(
		&mut self,
		size: usize,
		align: usize,
	) -> Option<usize> {
		if self.memory_region.is_empty() || size == 0 {
			return None;
		}

		let alloc_size =
			core::cmp::max(size.next_multiple_of(PAGE_SIZE), PAGE_SIZE);
		let required_align = core::cmp::max(align, PAGE_SIZE);

		let mut found_region: Option<&MemRegion> = None;

		for region in self.memory_region.iter() {
			if region.is_empty() || region.size < alloc_size {
				continue;
			}

			println_serial!("We found a region! {:?}", region);
			found_region = Some(region);
			break;
		}

		if let Some(region) = found_region {
			println_serial!("Returning base: 0x{:x}", region.base);
			let base = region.base;
			return Some(base);
		}

		return None;
	}
}
