//! An early physical memory allocator, similar in concept to Linux's memblock.
//!
//! Manages physical memory based on regions reported by the bootloader.
//! It tracks available and reserved memory regions and provides basic
//! allocation. Typically used during boot before the main page allocator is
//! initialized.

use super::{MemorySegment, PhysAddr, RegionType};
use crate::{
	arch::x86::multiboot::{get_memory_region, MultibootInfo, G_SEGMENTS},
	memory::PAGE_SIZE,
	println, println_serial,
	sync::{mutex::MutexGuard, Locked},
};
use core::{
	alloc::{GlobalAlloc, Layout},
	fmt::Debug,
	ptr,
};

const MAX_REGION: usize = 64;

/// Represents a memory region
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MemRegion {
	base: PhysAddr,
	size: usize,
}

impl MemRegion {
	/// Creates a new memory region with the specified base address and size.
	pub const fn new(base: PhysAddr, size: usize) -> Self {
		Self {
			base,
			size,
		}
	}

	/// Checks if this memory region is empty (has zero size).
	pub fn is_empty(&self) -> bool {
		*self == MemRegion::empty()
	}

	/// Creates an empty memory region with zero base address and size.
	pub const fn empty() -> Self {
		MemRegion {
			base: PhysAddr::new(0x0),
			size: 0,
		}
	}

	/// Returns the base of region
	pub const fn base(&self) -> PhysAddr {
		self.base
	}

	/// Returns size of region
	pub const fn size(&self) -> usize {
		self.size
	}
}

/// `memblock` allocator metadata
#[derive(Debug)]
pub struct MemBlockAllocator {
	memory_region: [MemRegion; MAX_REGION],
	reserved_region: [MemRegion; MAX_REGION],
	memory_count: usize,
	reserved_count: usize,
}

unsafe impl Send for MemBlockAllocator {}
unsafe impl Sync for MemBlockAllocator {}

impl MemBlockAllocator {
	/// Creates a new memory block allocator with empty region arrays.
	///
	/// Initializes the allocator with zero available and zero reserved memory
	/// regions. This is typically called very early in the boot process.
	#[allow(clippy::new_without_default)]
	pub const fn new() -> Self {
		const EMPTY: MemRegion = MemRegion::empty();

		Self {
			memory_region: [EMPTY; MAX_REGION],
			reserved_region: [EMPTY; MAX_REGION],
			memory_count: 0,
			reserved_count: 0,
		}
	}

	/// Creates a new memory block allocator with empty region arrays.
	///
	/// Initializes the allocator with zero available and zero reserved memory
	/// regions. This is typically called very early in the boot process.
	pub fn init(&mut self) {
		let segments = G_SEGMENTS.lock();

		for segment in segments.iter() {
			// TODO: Might add other RegionTypes
			#[allow(clippy::single_match)]
			match segment.segment_type() {
				RegionType::Available => {
					if !self.add(segment.start_addr(), segment.size()) {
						panic!("memblock: MAX_COUNT is full in memory segment");
					}
				}
				_ => {}
			}
		}
	}

	/// Returns a reference of the current length of `mem_region`
	pub const fn mem_count(&self) -> usize {
		self.memory_count
	}

	/// Returns a reference to the current `memregion`
	pub const fn mem_region(&self) -> &[MemRegion; MAX_REGION] {
		&self.memory_region
	}

	/// Returns a reference of the current length of `reserved_region`
	pub const fn reserved_count(&self) -> usize {
		self.reserved_count
	}

	/// Returns a reference to the current `reserved_region`
	pub const fn reserved_region(&self) -> &[MemRegion; MAX_REGION] {
		&self.reserved_region
	}

	/// Allocates memory with the specified layout requirements.
	///
	/// Attempts to find a region of memory that satisfies the size and
	/// alignment requirements specified in the layout. If successful, returns
	/// a pointer to the allocated memory; otherwise, returns a null pointer.
	///
	/// # Safety
	/// This function is unsafe because improper use may lead to memory
	/// unsafety.
	///
	/// # Parameters
	/// * `layout` - The layout requirements for the allocation
	///
	/// # Returns
	/// A pointer to the allocated memory or null if allocation fails
	pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
		match self.find_free_region(layout.size(), layout.align()) {
			Some(addr) => addr.as_mut_ptr(),
			None => ptr::null_mut(),
		}
	}

	/// Deallocates previously allocated memory.
	///
	/// This function is not implemented for MemBlockAllocator and will panic if
	/// called. Memory allocated by this allocator is only meant to be freed
	/// when transitioning to a more sophisticated memory allocator.
	///
	/// # Safety
	/// This function is unsafe and will panic if called.
	///
	/// # Panics
	/// This function always panics if called
	pub unsafe fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
		panic!("memblock::dealloc: Should never been called");
	}

	fn remove(&mut self, region_type: RegionType, index: usize) {
		match region_type {
			RegionType::Available => {
				for i in index..self.memory_count - 1 {
					self.memory_region[i] = self.memory_region[i + 1];
				}

				self.memory_region[self.memory_count - 1] = MemRegion::empty();
				self.memory_count -= 1;
			}
			RegionType::Reserved => {
				for i in index..self.reserved_count - 1 {
					self.reserved_region[i] = self.reserved_region[i + 1];
				}

				self.reserved_region[self.reserved_count - 1] =
					MemRegion::empty();
				self.reserved_count -= 1;
			}
			_ => {}
		}
	}

	#[must_use]
	fn add(&mut self, base: PhysAddr, size: usize) -> bool {
		if self.memory_count >= MAX_REGION {
			return false;
		}

		self.memory_region[self.memory_count] = MemRegion::new(base, size);
		self.memory_count += 1;

		return true;
	}

	#[must_use]
	fn reserved(&mut self, base: PhysAddr, size: usize) -> bool {
		if self.reserved_count >= MAX_REGION {
			return false;
		}

		self.reserved_region[self.reserved_count] = MemRegion::new(base, size);
		self.reserved_count += 1;

		return true;
	}

	/// Finds a free memory region that satisfies the given size and alignment
	/// requirements.
	///
	/// This function searches for an available memory region large enough to
	/// accommodate the requested size with the specified alignment. If found,
	/// it reserves the region, handles any alignment padding, and returns the
	/// aligned physical address.
	///
	/// # Parameters
	/// * `size` - The requested size in bytes
	/// * `align` - The required alignment in bytes
	///
	/// # Returns
	/// Some(physical_address) if a suitable region was found, None otherwise
	pub fn find_free_region(
		&mut self,
		size: usize,
		align: usize,
	) -> Option<PhysAddr> {
		if self.memory_count == 0 || size == 0 {
			return None;
		}

		let alloc_size =
			core::cmp::max(size.next_multiple_of(PAGE_SIZE), PAGE_SIZE);
		let required_align = core::cmp::max(align, PAGE_SIZE);
		let mut found_index = None;

		for (i, region) in self.memory_region.iter().enumerate() {
			if region.is_empty() || region.size < alloc_size {
				continue;
			}

			let base_addr = region.base;
			let aligned_addr = if base_addr.as_usize() % required_align == 0 {
				base_addr
			} else {
				let align_mask = required_align - 1;
				((base_addr.as_usize() + align_mask) & !align_mask).into()
			};

			let alignment_offset = aligned_addr - base_addr;
			if region.size >= alignment_offset + alloc_size {
				println_serial!(
					"Found suitable region at index {}: {:?}",
					i,
					region
				);
				found_index = Some(i);
				break;
			}
		}

		if let Some(i) = found_index {
			let region = self.memory_region[i];
			let original_base = region.base;
			let original_size = region.size;

			self.remove(RegionType::Available, i);

			let aligned_addr = if original_base.as_usize() % required_align == 0
			{
				original_base
			} else {
				let align_mask = required_align - 1;
				((original_base.as_usize() + align_mask) & !align_mask).into()
			};

			if !self.reserved(aligned_addr, alloc_size) {
				println!("Max Count in reserved_region array");
			}

			let alignment_gap = aligned_addr - original_base;
			if alignment_gap > 0 && !self.add(original_base, alignment_gap) {
				println!("Max Count in memory_region array");
			}

			let remaining_size = original_size - alignment_gap - alloc_size;
			if remaining_size > 0
				&& !self.add(aligned_addr + alloc_size, remaining_size)
			{
				println!("Max Count in memory_region array");
			}

			println_serial!(
				"Allocated at: 0x{:x}, size: {}",
				aligned_addr.as_usize(),
				alloc_size
			);
			return Some(aligned_addr);
		}

		None
	}
}
