#[allow(missing_docs)]
#[cfg(target_arch = "x86")]
#[repr(C, packed)]
pub struct MultibootMmapEntry {
	pub size: u32,
	pub addr: u64,
	pub len: u64,
	pub entry_type: u32,
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

#[allow(missing_docs)]
#[repr(C, packed)]
pub struct MultibootInfo {
	/* Multiboot info version number */
	pub flags: u32,

	/* Available memory from BIOS */
	mem_lower: u32,
	mem_upper: u32,

	/* "root" partition */
	boot_device: u32,

	/* Kernel command line */
	cmdline: u32,

	/* Boot-Module list */
	mods_count: u32,
	mods_addr: u32,

	syms: [u8; 16],

	/* Memory Mapping buffer */
	pub mmap_length: u32,
	pub mmap_addr: u32,

	/* Drive Info buffer */
	drives_length: u32,
	drives_addr: u32,

	/* ROM configuration table */
	config_table: u32,

	/* Boot Loader Name */
	pub boot_loader_name: *const u8,

	/* APM table */
	apm_table: u32,
}
