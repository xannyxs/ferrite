pub type Entry = u32;

#[repr(C, align(4096))]
pub struct Page {
	pub entries: [Entry; 1024],
}

static mut PAGE_DIRECTORY: Page = Page {
	entries: [0x00000002; 1024],
};

static mut PAGE_TABLE: Page = Page {
	entries: [0x00000002; 1024],
};

#[no_mangle]
pub fn init_paging_directory() -> *mut Page {
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
