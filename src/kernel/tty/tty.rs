use super::vga::{
	Buffer, ColourCode, VgaChar, VgaColour, VGA_HEIGHT, VGA_WIDTH,
};
use core::fmt;

pub struct Writer {
	column_position: usize,
	row_position: usize,
	pub colour_code: ColourCode,
	buffer: &'static mut Buffer,
}

impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);
		Ok(())
	}
}

impl Writer {
	pub fn new() -> Writer {
		let mut writer = Writer {
			column_position: 0,
			row_position: 0,
			colour_code: ColourCode::new(
				VgaColour::LightGrey,
				VgaColour::Black,
			),
			buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
		};
		writer.clear_screen();

		writer
	}

	pub fn write_string(&mut self, str: &str) {
		for byte in str.bytes() {
			match byte {
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				_ => self.write_byte(0xfe),
			}
		}
	}

	fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.column_position >= VGA_WIDTH {
					self.new_line()
				}

				let row = self.row_position;
				let col = self.column_position;
				let colour_code = self.colour_code;

				self.buffer.chars[row][col] = VgaChar {
					ascii_character: byte,
					colour_code,
				};

				self.column_position += 1;
				if self.row_position + 1 == VGA_HEIGHT {
					self.column_position = 0;
					self.row_position = 0;
				}
			}
		}
	}

	fn new_line(&mut self) {
		self.column_position = 0;
		self.row_position += 1;
	}

	fn clear_screen(&mut self) {
		let blank = VgaChar {
			ascii_character: b' ',
			colour_code: self.colour_code,
		};

		self.buffer.chars = [[blank; VGA_WIDTH]; VGA_HEIGHT];
	}
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}
