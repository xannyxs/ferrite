use crate::println;
use core::arch::asm;

#[doc(hidden)]
pub fn print_gdt() {
	let gdtr: [u8; 6] = [0; 6];

	unsafe {
		asm!("sgdt [{}]", in(reg) &gdtr);
	}

	let limit = u16::from_le_bytes([gdtr[0], gdtr[1]]);
	let base = u32::from_le_bytes([gdtr[2], gdtr[3], gdtr[4], gdtr[5]]);

	println!("GDTR limit: 0x{:04x}, base: 0x{:08x}", limit, base);
}
