use super::vga::{self, VGA_HEIGHT, VGA_WIDTH};

pub fn init_tty() {
	let terminal_column: usize = 0;
	let terminal_row: usize = 0;
	let terminal_colour = vga_entry_colour(
		vga::VGA_COLOUR::VGA_COLOR_LIGHT_GREY,
		vga::VGA_COLOUR::VGA_COLOR_BLACK,
	);
	let terminal_buffer = vga::VGA_MEMORY;

	for y in 0..VGA_HEIGHT {
		for x in 0..VGA_WIDTH {
			let index: usize = y * VGA_HEIGHT + x;
			terminal_buffer[index] = vga_entry(' ', terminal_colour);
		}
	}
}
