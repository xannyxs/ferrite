//! Kernel memory management core types and allocator modules.
//!
//! This module defines fundamental types for memory regions (`MemorySegment`,
//! `RegionType`), physical addresses (`PhysAddr`), page constants
//! (`PAGE_SIZE`), and organizes the different memory allocator implementations.

extern crate alloc;

/* -------------------------------------- */

pub mod allocator;
pub mod buddy;
pub mod memblock;
pub mod node_pool;
pub mod slab;
pub mod transition;

pub use node_pool::NodePoolAllocator;

/* -------------------------------------- */

/// Type alias for representing physical memory addresses.
pub type PhysAddr = usize;
/// Defines the system's page size (typically 4 KiB on x86_64).
pub const PAGE_SIZE: usize = 4096;

#[repr(u32)]
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RegionType {
	Unknown = 0,
	Available = 1,
	Reserved = 2,
	AcpiReclaimable = 3,
	AcpiNvs = 4,
	BadMemory = 5,
}

/// Represents a segment of memory in the system's memory map.
/// Each segment has a start address, size, and type that indicates its usage.
#[derive(Debug, Copy, Clone)]
pub struct MemorySegment {
	start_addr: usize,
	len: usize,
	segment_type: RegionType,
}

impl MemorySegment {
	/// Creates a new MemorySegment with the specified parameters
	///
	/// # Arguments
	/// * `start_addr` - The physical start address of the memory segment
	/// * `size` - The size of the memory segment in bytes
	/// * `segment_type` - The type of memory segment (Available, Reserved, etc)
	pub fn new(
		start_addr: usize,
		len: usize,
		segment_type: RegionType,
	) -> Self {
		return Self {
			start_addr,
			len,
			segment_type,
		};
	}

	/// Creates an empty memory segment with zero base address, size, and
	/// `Unknown` type. Useful for initializing arrays or representing
	/// invalid/unused segments.
	pub const fn empty() -> Self {
		return Self {
			start_addr: 0,
			len: 0,
			segment_type: RegionType::Unknown,
		};
	}

	/// Returns the physical start address of this memory segment
	pub const fn start_addr(&self) -> usize {
		return self.start_addr;
	}

	/// Returns the size of this memory segment in bytes
	pub const fn size(&self) -> usize {
		return self.len;
	}

	/// Returns the type of this memory segment (Available, Reserved, etc)
	pub const fn segment_type(&self) -> RegionType {
		return self.segment_type;
	}
}
