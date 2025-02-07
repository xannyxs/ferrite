// VGA Text Mode Driver
//
// This module implements a text-mode display driver for the VGA hardware,
// allowing us to write text to the screen. The VGA text buffer is located
// at physical address 0xB8000 and provides a 80x25 character display where
// each character cell consists of:
//   - An ASCII character (8 bits)
//   - A colour code (8 bits) specifying foreground and background colours
//
// The driver provides safe abstractions over this hardware interface and
// implements basic console functionality like newlines and screen clearing.
//------------------------------------------------------------------------------

use super::vga::{
	Buffer, ColourCode, VgaChar, VgaColour, VGA_HEIGHT, VGA_WIDTH,
};
use core::fmt;

/// Represents a text-mode VGA writer that can output characters to the screen.
/// Keeps track of the current cursor position and text colours.
pub struct Writer {
	column_position: usize,
	row_position: usize,
	pub colour_code: ColourCode,
	buffer: &'static mut Buffer, // Points to VGA memory at 0xB8000
}

// Implement the core::fmt::Write trait so we can use Rust's formatting macros
impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);
		Ok(())
	}
}

impl Writer {
	/// Creates a new Writer instance with default light grey on black colours.
	/// Initializes the VGA buffer pointer and clears the screen.
	pub fn new() -> Writer {
		let mut writer = Writer {
			column_position: 0,
			row_position: 0,
			colour_code: ColourCode::new(
				VgaColour::LightGrey,
				VgaColour::Black,
			),
			// Safety: 0xB8000 is the VGA buffer's physical address.
			// This is safe because we know this memory is always mapped
			// and we have exclusive access to it at kernel level.
			buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
		};
		writer.clear_screen();
		writer
	}

	/// Writes a string to the screen, handling both printable ASCII characters
	/// and newlines. Any unprintable characters are replaced with ■ (0xFE).
	pub fn write_string(&mut self, str: &str) {
		for byte in str.bytes() {
			match byte {
				// Only handle printable ASCII chars (0x20 to 0x7e) and newlines
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				// Replace unprintable characters with ■
				_ => self.write_byte(0xfe),
			}
		}
	}

	/// Writes a single byte to the screen, handling newlines and screen
	/// wrapping. Updates cursor position after writing.
	fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.column_position >= VGA_WIDTH {
					self.new_line();
				}

				let row = self.row_position;
				let col = self.column_position;
				let colour_code = self.colour_code;

				// Write the character to the buffer
				self.buffer.chars[row][col] = VgaChar {
					ascii_character: byte,
					colour_code,
				};

				self.column_position += 1;

				// Reset to top when we reach the bottom of the screen
				if self.row_position + 1 == VGA_HEIGHT {
					self.column_position = 0;
					self.row_position = 0;
				}
			}
		}
	}

	/// Moves the cursor to the start of the next line
	fn new_line(&mut self) {
		self.column_position = 0;
		self.row_position += 1;
	}

	/// Clears the entire screen by filling it with spaces
	/// Resets column & row value to 0
	fn clear_screen(&mut self) {
		self.column_position = 0;
		self.row_position = 0;

		let blank = VgaChar {
			ascii_character: b' ',
			colour_code: self.colour_code,
		};
		// Fill the entire buffer with blank characters
		self.buffer.chars = [[blank; VGA_WIDTH]; VGA_HEIGHT];
	}
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
	/// Global writer instance protected by a mutex for safe concurrent access.
	/// This allows us to use the writer from anywhere in the kernel.
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}
