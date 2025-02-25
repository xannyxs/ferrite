use crate::{arch::x86::idt::IDT_ENTRIES, println};
use core::arch::asm;

///Stores the content the interrupt descriptor table register (IDTR) in the
/// destination operand. The destination operand specifies a 6-byte memory
/// location. In non-64-bit modes, the 16-bit limit field of the register is
/// stored in the low 2 bytes of the memory location and the 32-bit base address
/// is stored in the high 4 bytes.
pub fn print_idt() {
	let idtr: [u8; 6] = [0; 6];
	unsafe {
		asm!("sidt [{}]", in(reg) &idtr);
	}

	let limit = u16::from_le_bytes([idtr[0], idtr[1]]);
	let base = u32::from_le_bytes([idtr[2], idtr[3], idtr[4], idtr[5]]);

	println!("IDTR limit: {:04x}, base: 0x{:08x}", limit, base);
}
