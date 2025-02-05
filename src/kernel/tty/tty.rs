use super::vga::{Buffer, ColourCode, VgaChar, VGA_HEIGHT, VGA_WIDTH};

pub struct Writer {
	column_position: usize,
	row_position: usize,
	colour_code: ColourCode,
	buffer: &'static mut Buffer,
}

impl Writer {
	pub fn new(colour_code: ColourCode) -> Writer {
		let mut writer = Writer {
			column_position: 0,
			row_position: 0,
			colour_code,
			buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
		};
		writer.clear_screen();
		writer
	}

	pub fn write_string(&mut self, str: &str) {
		for byte in str.bytes() {
			self.write_byte(byte)
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
