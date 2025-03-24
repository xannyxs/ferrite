use core::arch::asm;

pub unsafe fn get_page_directory() -> *mut u32 {
	let page_dir_addr: u32;
	unsafe {
		asm!("mov {}, cr3", out(reg) page_dir_addr);
	}

	let ptr: *mut u32 =
		core::ptr::with_exposed_provenance_mut(page_dir_addr as usize);

	return ptr;
}
