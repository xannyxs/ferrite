/// An Entry of 32-bits
pub type Entry = u32;

/// All tables (PD & PT) contain 1024 4-byte entries, making them 4 KiB each. In
/// the page directory, each entry points to a page table. In the page table,
/// each entry points to a 4 KiB physical page frame.
#[repr(C, align(4096))]
pub struct Page {
	/// A page table/directory entry structured as:
	/// ```text
	/// 31        12 11  9 8 7 6 5 4 3 2 1 0
	/// +----------+-----+-+-+-+-+-+-+-+-+-+
	/// | PhysAddr |Avail|G|P|D|A|C|W|U|R|P|
	/// +----------+-----+-+-+-+-+-+-+-+-+-+
	/// ```
	/// PhysAddr: 4KB-aligned physical address
	/// Flags: P(resent), R(ead/Write), remaining bits for various controls
	pub entries: [Entry; 1024],
}

static mut PAGE_DIRECTORY: Page = Page {
	entries: [0x00000002; 1024],
};

static mut PAGE_TABLE: Page = Page {
	entries: [0; 1024],
};

/// Initializes a new the Paging Table.
pub fn create_page_table() -> Page {
	let mut page = Page {
		entries: [0; 1024],
	};

	for (i, entry) in page.entries.iter_mut().enumerate() {
		*entry = ((i as u32) * 0x1000) | 3;
	}

	return page;
}

/// Initializes the Paging Directory and Page Table structures.
/// Called from assembly code to set up paging entries and returns
/// a pointer to the Page Directory.
///
/// # Safety
///
/// This function uses raw pointers and static mutable variables,
/// requiring unsafe blocks for memory operations. The caller must
/// ensure that:
/// - The physical memory is available and properly aligned
/// - No other code is modifying these page structures concurrently
#[no_mangle]
pub unsafe fn init_paging_directory() -> *mut Page {
	unsafe {
		let table_ptr = (&raw mut PAGE_TABLE.entries);
		for i in 0..1024 {
			(*table_ptr)[i] = ((i as u32) * 0x1000) | 3;
		}
		let page_table_addr = &raw const PAGE_TABLE as u32;
		PAGE_DIRECTORY.entries[0] = page_table_addr | 3;
		return &raw mut PAGE_DIRECTORY;
	}
}
