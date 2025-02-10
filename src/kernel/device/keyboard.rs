use crate::arch::x86::io;

pub fn get_keyboard_input() -> Option<u8> {
	const KEYBOARD_DATA_PORT: u16 = 0x60;
	const KEYBOARD_STATUS_PORT: u16 = 0x64;

	if io::inb(KEYBOARD_STATUS_PORT) & 1 == 0 {
		return None;
	}

	let scan_code = io::inb(KEYBOARD_DATA_PORT);

	if scan_code >= 0x80 {
		return None;
	}

	return Some(scan_code);
}
