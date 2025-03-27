use super::MemorySegment;
use crate::{
	arch::x86::multiboot::{MultibootInfo, MultibootMmapEntry},
	memory::RegionType,
	println_serial,
};
use core::{mem, ptr};

pub fn get_primary_memory_region(boot_info: &MultibootInfo) -> MemorySegment {
	let mut mmap = boot_info.mmap_addr as usize;
	let mmap_end = (boot_info.mmap_addr + boot_info.mmap_length) as usize;

	while mmap < mmap_end {
		unsafe {
			#[allow(clippy::expect_used)]
			let entry = (ptr::with_exposed_provenance_mut(mmap)
				as *const MultibootMmapEntry)
				.as_ref()
				.expect("Failed to read memory map entry");
			let addr = entry.addr;
			let len = entry.len;
			let entry_type = entry.entry_type;

			if entry_type == RegionType::Available && addr == 0x100000 {
				println_serial!("\nMemory Region:");
				println_serial!("  Start Address: 0x{:x}", addr);
				println_serial!(
					"  Length: {} bytes ({} MB)",
					len,
					len / 1024 / 1024
				);

				return MemorySegment::new(
					entry.addr,
					entry.len,
					entry.entry_type,
				);
			}

			mmap += (entry.size as usize) + mem::size_of::<u32>()
		}
	}

	panic!("Could not find necessary memory region");
}
