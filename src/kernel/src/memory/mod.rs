//! Kernel memory management core types and allocator modules.
//!
//! This module defines fundamental types for memory regions (`MemorySegment`,
//! `RegionType`), physical addresses (`PhysAddr`), page constants
//! (`PAGE_SIZE`), and organizes the different memory allocator implementations.

extern crate alloc;

/* -------------------------------------- */

pub mod addr;
pub mod allocator;
pub mod buddy;
pub mod frame;
pub mod memblock;
pub mod node_pool;
pub mod paging;
pub mod slab;

pub use addr::{PhysAddr, VirtAddr};
pub use buddy::BuddyAllocator;
use core::sync::atomic::{AtomicUsize, Ordering};
pub use frame::FrameAllocator;
pub use memblock::MemBlockAllocator;
pub use node_pool::NodePoolAllocator;
pub use slab::SlabCache;

/* -------------------------------------- */

extern "C" {
	static _kernel_virtual_end: u8;
	static _kernel_physical_start: u8;
	static _kernel_physical_end: u8;
}

/// Function to get the physical start address of the kernel image.
pub fn get_kernel_physical_start() -> PhysAddr {
	unsafe { PhysAddr::new(&_kernel_physical_start as *const u8 as usize) }
}

/// Function to get the physical end address of the kernel image.
pub fn get_kernel_physical_end() -> PhysAddr {
	unsafe { PhysAddr::new(&_kernel_physical_end as *const u8 as usize) }
}

/// Function to get the virtual end address of the kernel image.
pub fn get_kernel_virtual_end() -> VirtAddr {
	unsafe { VirtAddr::new(&_kernel_virtual_end as *const u8 as usize) }
}

/* -------------------------------------- */

/// The offset of the kernel
const KERNEL_OFFSET: usize = 0xc0000000;
/// Defines the system's page size
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

/* -------------------------------------- */

const NODE_POOL_VIRT_START: usize = 0xc1000000;

const VIRT_START: usize = 0xd000_0000;
const VIRT_SIZE: usize = 1024 * 1024 * 128;
const VIRT_END: usize = VIRT_START + VIRT_SIZE;

static NEXT_FREE_VIRT_ADDR: AtomicUsize = AtomicUsize::new(VIRT_START);

/// Function to allocate a contiguous block of virtual address space
/// Returns the start virtual address of the allocated block, or None if out of
/// space.
pub fn allocate_dynamic_virt_range(size: usize) -> Option<VirtAddr> {
	let size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
	let current_start = NEXT_FREE_VIRT_ADDR.fetch_add(size, Ordering::SeqCst);
	let allocation_end = current_start.checked_add(size)?;

	if allocation_end > VIRT_END {
		NEXT_FREE_VIRT_ADDR.fetch_sub(size, Ordering::SeqCst);
		return None;
	}

	Some(VirtAddr::new(current_start))
}

/* -------------------------------------- */

/// Represents a segment of memory in the system's memory map.
/// Each segment has a start address, size, and type that indicates its usage.
#[derive(Debug, Copy, Clone)]
pub struct MemorySegment {
	start_addr: PhysAddr,
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
		start_addr: PhysAddr,
		len: usize,
		segment_type: RegionType,
	) -> Self {
		Self {
			start_addr,
			len,
			segment_type,
		}
	}

	/// Creates an empty memory segment with zero base address, size, and
	/// `Unknown` type. Useful for initializing arrays or representing
	/// invalid/unused segments.
	pub const fn empty() -> Self {
		Self {
			start_addr: PhysAddr::new(0),
			len: 0,
			segment_type: RegionType::Unknown,
		}
	}

	/// Returns the physical start address of this memory segment
	pub const fn start_addr(&self) -> PhysAddr {
		self.start_addr
	}

	/// Returns the size of this memory segment in bytes
	pub const fn size(&self) -> usize {
		self.len
	}

	/// Returns the type of this memory segment (Available, Reserved, etc)
	pub const fn segment_type(&self) -> RegionType {
		self.segment_type
	}
}
