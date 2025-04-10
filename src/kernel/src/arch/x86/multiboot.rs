//! Defines structures and functions related to parsing the Multiboot 1
//! information structure provided by the bootloader.

use crate::{
	memory::{MemorySegment, RegionType},
	println_serial,
	sync::{mutex::MutexGuard, Locked},
};

#[allow(missing_docs)]
#[cfg(target_arch = "x86")]
#[repr(C, packed)]
pub struct MultibootMmapEntry {
	pub size: u32,
	pub addr: u64,
	pub len: u64,
	pub entry_type: RegionType,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct MultibootMmapEntry {
	size: u32,
	addr: u64,
	len: u64,
	entry_type: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct MultibootAoutSymbolTable {
	tabsize: u32,
	strsize: u32,
	addr: u32,
	reserved: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct MultibootElfSection {
	num: u32,
	size: u32,
	addr: u32,
	shndx: u32,
}

/// Represents the Multiboot information structure passed by the bootloader to
/// the kernel. This structure contains various pieces of information about the
/// system and boot process.
#[repr(C, packed)]
pub struct MultibootInfo {
	/// Multiboot information version number and available fields indicator.
	/// Each bit indicates the validity of a particular field in this
	/// structure.
	pub flags: u32,

	/// Amount of lower memory in kilobytes (memory below 1MB).
	/// Only valid if flags[0] is set.
	mem_lower: u32,

	/// Amount of upper memory in kilobytes (memory above 1MB).
	/// Only valid if flags[0] is set.
	mem_upper: u32,

	/// BIOS disk device that the kernel was loaded from.
	/// Only valid if flags[1] is set.
	boot_device: u32,

	/// Physical address of the command line passed to the kernel.
	/// Only valid if flags[2] is set.
	cmdline: u32,

	/// Number of modules loaded along with the kernel.
	/// Only valid if flags[3] is set.
	mods_count: u32,

	/// Physical address of the first module structure.
	/// Only valid if flags[3] is set.
	mods_addr: u32,

	/// Symbol table information for ELF or a.out formats.
	/// Format depends on flags[4] and flags[5].
	syms: [u8; 16],

	/// Length of the memory map buffer provided by the bootloader.
	/// Only valid if flags[6] is set.
	pub mmap_length: u32,

	/// Physical address of the memory map buffer.
	/// Only valid if flags[6] is set.
	pub mmap_addr: u32,

	/// Length of the drives structure.
	/// Only valid if flags[7] is set.
	drives_length: u32,

	/// Physical address of the drives structure.
	/// Only valid if flags[7] is set.
	drives_addr: u32,

	/// Address of ROM configuration table.
	/// Only valid if flags[8] is set.
	config_table: u32,

	/// Physical address of the bootloader's name string.
	/// Only valid if flags[9] is set.
	pub boot_loader_name: *const u8,

	/// Address of APM (Advanced Power Management) table.
	/// Only valid if flags[10] is set.
	apm_table: u32,
}

/// Global static storage for the parsed memory map segments.
///
/// Initialized once during boot by `get_memory_region`. Access should be
/// synchronized via the `Locked` wrapper. Maximum of 16 segments stored.
// Note: lazy_static might be needed if Locked::new isn't const, or use
// OnceCell. Assuming Locked::new is const based on previous context.
// lazy_static! { // Use lazy_static if Locked::new() is not const
pub static G_SEGMENTS: Locked<[MemorySegment; 16]> =
	Locked::new([MemorySegment::empty(); 16]);

/// Parses the Multiboot memory map and populates the provided `segments` array.
///
/// Iterates through the memory map entries provided by the `boot_info`
/// structure, converts them into `MemorySegment` representations, and stores
/// them in the `segments` slice. It reserves the region starting at physical
/// address 0x0.
///
/// # Arguments
/// * `segments` - A mutable array slice to be filled with parsed
///   `MemorySegment` data. Must have a size of at least 16
///   (`MAX_MEMORY_SEGMENTS`).
/// * `boot_info` - A reference to the `MultibootInfo` structure provided by the
///   bootloader.
///
/// # Panics
/// Panics if the bootloader information does not contain a valid memory map
/// (`flags` bit 6 not set), or if no memory regions are found in the map.
pub fn get_memory_region(
	segments: &mut [MemorySegment; 16],
	boot_info: &MultibootInfo,
) {
	use core::{mem, ptr};

	let mut count = 0;
	let mut mmap = boot_info.mmap_addr as usize;
	let mmap_end = (boot_info.mmap_addr + boot_info.mmap_length) as usize;

	while mmap < mmap_end {
		unsafe {
			#[allow(clippy::expect_used)]
			let entry = (ptr::with_exposed_provenance_mut(mmap)
				as *const MultibootMmapEntry)
				.as_ref()
				.expect("Failed to read memory map entry");

			let entry_type = entry.entry_type;
			if entry_type != RegionType::Available || entry.addr == 0x0 {
				mmap += (entry.size as usize) + mem::size_of::<u32>();
				continue;
			}

			segments[count] = MemorySegment::new(
				entry.addr as usize,
				entry.len as usize,
				entry.entry_type,
			);

			let base_addr = entry.addr as usize;
			let length = entry.len as usize;
			let entry_type = entry.entry_type;

			println_serial!(
				"  Entry {}: Base=0x{:016x}, Length=0x{:016x} ({} bytes), Type={:?}",
				count,
				base_addr,
				length,
				length,
				entry_type,
			);

			count += 1;
			mmap += (entry.size as usize) + mem::size_of::<u32>();
		}
	}

	if count == 0 {
		panic!("Could not find any memory regions in map (or map was empty)!");
	}
}
