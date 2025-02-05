use super::vga::{self, vga_entry, vga_entry_colour, VGA_HEIGHT, VGA_WIDTH};

static mut TERMINAL_COLUMN: usize = 0;
static mut TERMINAL_ROW: usize = 0;
static mut TERMINAL_COLOUR: u8 = 0;

pub fn get_terminal_row() -> usize {
	unsafe { TERMINAL_ROW }
}

pub fn set_terminal_row(row: usize) {
	unsafe {
		TERMINAL_ROW = row;
	}
}

pub fn get_terminal_colour() -> u8 {
	unsafe { TERMINAL_COLOUR }
}

pub fn set_terminal_colour(colour: u8) {
	unsafe {
		TERMINAL_COLOUR = colour;
	}
}

pub fn get_terminal_column() -> usize {
	unsafe { TERMINAL_COLUMN }
}

pub fn set_terminal_column(column: usize) {
	unsafe {
		TERMINAL_COLUMN = column;
	}
}

pub fn init_tty() {
	set_terminal_column(0);
	set_terminal_row(0);
	set_terminal_colour(vga_entry_colour(
		vga::VgaColour::LightGrey,
		vga::VgaColour::Black,
	));
	let buffer = vga::VGA_MEMORY;

	unsafe {
		for y in 0..VGA_HEIGHT {
			for x in 0..VGA_WIDTH {
				let index: usize = y * VGA_HEIGHT + x;
				*buffer.add(index) = vga_entry(b' ', TERMINAL_COLOUR);
			}
		}
	}
}

pub fn terminal_putentryat(uc: u8, colour: u8, x: usize, y: usize) {
	let index: usize = y * VGA_WIDTH + x;
	unsafe {
		let buffer = vga::VGA_MEMORY.add(index);
		*buffer = vga_entry(uc, colour);
	}
}

pub fn terminal_putchar(c: char) {
	let uc = c as u8;

	if uc == b'\n' {
		set_terminal_row(get_terminal_row() + 1);
		set_terminal_column(0);
		return;
	}

	terminal_putentryat(
		uc,
		get_terminal_colour(),
		get_terminal_column(),
		get_terminal_row(),
	);
	if get_terminal_column() + 1 == VGA_WIDTH {
		set_terminal_column(0);
		if get_terminal_row() + 1 == VGA_HEIGHT {
			set_terminal_row(VGA_HEIGHT - 1);
		} else {
			set_terminal_row(get_terminal_row() + 1);
		}
	} else {
		set_terminal_column(get_terminal_column() + 1);
	}
}

pub fn terminal_writestring(str: &str) {
	for byte in str.bytes() {
		terminal_putchar(byte as char);
	}
}
