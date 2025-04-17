use crate::{
	println, println_serial,
	tty::{tty::WRITER, Buffer, ColourCode, VgaColour, VGA_HEIGHT, VGA_WIDTH},
};

#[test_case]
fn test_println_simple() {
	println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
	for _ in 0..200 {
		println!("test_println_many output");
	}
}
#[test_case]
fn test_println_output() {
	let s = "Some test string that fits on a single line";
	println!("{}", s);
	for (i, c) in s.chars().enumerate() {
		let screen_char = WRITER.lock().buffer.chars[VGA_HEIGHT - 2][i];
		assert_eq!(char::from(screen_char.ascii_character), c);
	}
}
