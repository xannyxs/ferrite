#[inline]
pub fn check_protection_status() {
	let cr0: u32;
	unsafe {
		core::arch::asm!("mov {}, cr0", out(reg) cr0);
	}

	let pe_bit = cr0 & 1;
	if pe_bit == 0 {
		panic!(
			"CPU is not in Protected Mode! \
            CR0 value: {:#x} \
            PE bit (bit 0) is not set. \
            This likely means the GDT initialization failed.",
			cr0
		);
	}

	//let wp_bit = cr0 & (1 << 16);
	//if wp_bit == 0 {
	//	panic!(
	//		"Write Protection is not enabled! \
	//           This is required for proper memory protection. \
	//           CR0 value: {:#x}",
	//		cr0
	//	);
	//}
}
