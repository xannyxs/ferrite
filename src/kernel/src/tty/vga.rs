use core::mem::transmute;

#[doc(hidden)]
pub const VGA_WIDTH: usize = 80;

#[doc(hidden)]
pub const VGA_HEIGHT: usize = 25;

#[doc(hidden)]
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
#[doc(hidden)]
pub struct ColourCode(u8);

impl ColourCode {
	pub fn new(foreground: VgaColour, background: VgaColour) -> ColourCode {
		return ColourCode(((background as u8) << 4) | (foreground as u8));
	}

	pub fn get_foreground_colour(&self) -> VgaColour {
		let value = (self.0 >> 4) & 0x0f;
		unsafe {
			return transmute::<u8, VgaColour>(value);
		}
	}

	pub fn get_background_colour(&self) -> VgaColour {
		let value = self.0 << 4;
		unsafe {
			return transmute::<u8, VgaColour>(value);
		}
	}

	pub fn set_foreground_colour(&mut self, foreground: VgaColour) {
		self.0 = (self.0 & 0xf0) | (foreground as u8);
	}

	pub fn set_background_colour(&mut self, background: VgaColour) {
		self.0 = (self.0 & 0x0f) | ((background as u8) << 4);
	}
}

/// Represents a single character cell in the VGA text buffer.
///
/// A character cell contains both the ASCII character to display and its
/// appearance attributes (colors). This structure exactly mirrors the VGA
/// hardware's memory layout where each character occupies two consecutive
/// bytes in video memory.
///
/// # Fields
/// * `ascii_character` - The actual character to display (0-255 ASCII code)
/// * `colour_code` - Combined foreground and background color information
///
/// # Examples
/// ```
/// let char_cell = VgaChar {
/// 	ascii_character: b'A',
/// 	colour_code: ColourCode::new(VgaColour::White, VgaColour::Black),
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VgaChar {
	/// The actual character to display (like 'A', 'b', '1', etc.)
	/// Uses u8 because VGA text mode uses ASCII encoding
	pub ascii_character: u8,
	/// Defines how the character should look (colors)
	/// This is a separate type that handles the color attributes
	pub colour_code: ColourCode,
}

/// Buffer which is the a 2D Array of the VGA
#[doc(hidden)]
#[repr(transparent)]
pub struct Buffer {
	pub chars: [[VgaChar; VGA_WIDTH]; VGA_HEIGHT],
}
