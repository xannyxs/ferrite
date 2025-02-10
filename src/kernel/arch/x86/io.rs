use core::arch::asm;

pub fn inb(addr: u16) -> u8 {
	let mut out: u8;

	unsafe {
		asm!("in al, dx", out("al") out, in("dx") addr);
	}

	out
}

pub fn outb(addr: u16, val: u8) {
	unsafe {
		asm!("out dx, al", in("dx") addr, in("al") val);
	}
}
