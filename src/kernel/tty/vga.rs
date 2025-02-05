#[derive(Debug)]
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

pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;
pub const VGA_MEMORY: *mut u16 = 0xb8000 as *mut u16;

#[inline]
pub fn vga_entry_colour(foreground: VgaColour, background: VgaColour) -> u8 {
	(foreground as u8) | ((background as u8) << 4)
}

#[inline]
pub fn vga_entry(c: u8, colour: u8) -> u16 {
	(c as u16) | (colour as u16) << 8
}
