use stack::KernelStack;

extern crate alloc;

/* -------------------------------------- */

pub mod allocator;
pub mod buddy;
pub mod region;
pub mod stack;

/* -------------------------------------- */

pub const PAGE_SIZE: usize = 4096;

// pub static STACK = KernelStack::new();

#[repr(u32)]
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RegionType {
	Available = 1,
	Reserved = 2,
	AcpiReclaimable = 3,
	AcpiNvs = 4,
	BadMemory = 5,
}

/// Represents a segment of memory in the system's memory map.
/// Each segment has a start address, size, and type that indicates its usage.
#[derive(Copy, Clone)]
pub struct MemorySegment {
	start_addr: u64,
	len: u64,
	segment_type: RegionType,
}

impl MemorySegment {
	/// Creates a new MemorySegment with the specified parameters
	///
	/// # Arguments
	/// * `start_addr` - The physical start address of the memory segment
	/// * `size` - The size of the memory segment in bytes
	/// * `segment_type` - The type of memory segment (Available, Reserved, etc)
	fn new(start_addr: u64, len: u64, segment_type: RegionType) -> Self {
		return Self {
			start_addr,
			len,
			segment_type,
		};
	}

	/// Returns the physical start address of this memory segment
	pub const fn start_addr(&self) -> u64 {
		return self.start_addr;
	}

	/// Returns the size of this memory segment in bytes
	pub const fn size(&self) -> u64 {
		return self.len;
	}

	/// Returns the type of this memory segment (Available, Reserved, etc)
	pub const fn segment_type(&self) -> RegionType {
		return self.segment_type;
	}
}
