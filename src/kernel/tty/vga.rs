pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VgaColour {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGrey = 7,
	DarkGrey = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	LightMagenta = 13,
	LightBrown = 14,
	White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColourCode(u8);

#[allow(dead_code)] // Remove in future
impl ColourCode {
	pub fn new(foreground: VgaColour, background: VgaColour) -> ColourCode {
		ColourCode((background as u8) << 4 | (foreground as u8))
	}

	pub fn set_foreground_colour(&mut self, foreground: VgaColour) {
		self.0 = (self.0 & 0xf0) | (foreground as u8);
	}

	pub fn set_background_colour(&mut self, background: VgaColour) {
		self.0 = (self.0 & 0x0f) | ((background as u8) << 4);
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VgaChar {
	pub ascii_character: u8,
	pub colour_code: ColourCode,
}

#[repr(transparent)]
pub struct Buffer {
	pub chars: [[VgaChar; VGA_WIDTH]; VGA_HEIGHT],
}
