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
	size: PhysAddr,
}

impl MemRegion {
	/// Creates a new memory region with the specified base address and size.
	///
	/// # Parameters
	/// * `base` - The starting physical address of the memory region
	/// * `size` - The size of the memory region in bytes
	///
	/// # Returns
	/// A new `MemRegion` instance representing the specified memory area
	pub const fn new(base: PhysAddr, size: PhysAddr) -> Self {
		return Self {
			base,
			size,
		};
	}

	/// Checks if this memory region is empty (has zero size).
	///
	/// An empty region represents an unused slot in the memory region arrays.
	///
	/// # Returns
	/// `true` if the region is empty, `false` otherwise
	pub fn is_empty(&self) -> bool {
		return *self == MemRegion::empty();
	}

	/// Creates an empty memory region with zero base address and size.
	///
	/// Used to initialize memory region arrays and to represent unused slots.
	///
	/// # Returns
	/// An empty `MemRegion` instance
	pub const fn empty() -> Self {
		return MemRegion {
			base: 0x0,
			size: 0x0,
		};
	}
}

/// `memblock` allocator metadata
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
	/// Creates a new memory block allocator with empty region arrays.
	///
	/// Initializes the allocator with zero available and zero reserved memory
	/// regions. This is typically called very early in the boot process.
	///
	/// # Returns
	/// A new empty `MemBlockAllocator` instance
	#[allow(clippy::new_without_default)]
	pub const fn new() -> Self {
		const EMPTY: MemRegion = MemRegion::empty();

		return Self {
			memory_region: [EMPTY; MAX_REGION],
			reserved_region: [EMPTY; MAX_REGION],
			memory_count: 0,
			reserved_count: 0,
		};
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
			Some(addr) => return ptr::with_exposed_provenance_mut(addr),
			None => return ptr::null_mut(),
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
	/// # Parameters
	/// * `ptr` - Pointer to the memory to deallocate
	/// * `layout` - The layout that was used for allocation
	///
	/// # Panics
	/// This function always panics if called
	pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
		let addr = ptr as usize;
		let size = layout.size().next_multiple_of(PAGE_SIZE);

		let mut found = false;
		let mut reserved_index = 0;

		for (i, region) in self.reserved_region.iter().enumerate() {
			if region.base == addr && region.size >= size {
				reserved_index = i;
				found = true;
				break;
			}
		}

		if !found {
			panic!(
				"Attempted to deallocate memory not managed by this allocator"
			);
		}

		let reserved = self.reserved_region[reserved_index];
		self.remove(RegionType::Reserved, reserved_index);

		if !self.add(reserved.base, reserved.size) {
			println!("Max Count in reserved_region array");
		}

		self.coalesce_free_regions();

		println_serial!("Dealloc has been called in memblock()");
	}

	fn sort_regions(&mut self, region_type: RegionType) {
		let (regions, count) = match region_type {
			RegionType::Available => {
				(&mut self.memory_region, self.memory_count)
			}
			RegionType::Reserved => {
				(&mut self.reserved_region, self.reserved_count)
			}
			_ => (&mut self.memory_region, self.memory_count),
		};

		for i in 1..count {
			let key = regions[i];
			let mut j = i;

			while j > 0 && regions[j - 1].base > key.base {
				regions[j] = regions[j - 1];
				j -= 1;
			}

			regions[j] = key;
		}
	}

	fn coalesce_free_regions(&mut self) {
		self.sort_regions(RegionType::Available);

		let mut i = 0;

		while i < self.memory_count - 1 {
			let current = self.memory_region[i];
			let next = self.memory_region[i + 1];

			if current.base + current.size == next.base {
				self.memory_region[i].size += next.size;
				self.remove(RegionType::Available, i + 1);
			} else {
				i += 1;
			}
		}
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
	) -> Option<usize> {
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
			let aligned_addr = if base_addr % required_align == 0 {
				base_addr
			} else {
				let align_mask = required_align - 1;
				(base_addr + align_mask) & !align_mask
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

			let aligned_addr = if original_base % required_align == 0 {
				original_base
			} else {
				let align_mask = required_align - 1;
				(original_base + align_mask) & !align_mask
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
				aligned_addr,
				alloc_size
			);
			return Some(aligned_addr);
		}

		return None;
	}
}
