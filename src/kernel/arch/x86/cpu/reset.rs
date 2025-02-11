use super::control::halt_loop;
use crate::arch::x86::io::{inb, outb};

pub fn reboot() -> ! {
	let mut good: u8 = 0x02;

	while good == 0x02 {
		good = inb(0x64);
	}

	outb(0x64, 0xfe);

	unsafe {
		halt_loop();
	}
}
