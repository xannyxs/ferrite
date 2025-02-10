use crate::{
	libc::console::bin::{gdt, reboot},
	print, println,
};
use core::str::from_utf8;

pub struct Console {
	b_pos: usize,
	buffer: [u8; 256],
	prompt: &'static str,
}

impl Console {
	pub fn new() -> Self {
		let console = Console {
			b_pos: 0,
			buffer: [0; 256],
			prompt: "[shelly]$ ",
		};

		print!("{}", console.prompt);

		return console;
	}

	// TODO: Implement signals
	pub fn add_buffer(&mut self, c: char) {
		match c {
			'\n' => self.execute(),
			_ => {
				self.buffer[self.b_pos] = c as u8;
				self.b_pos += 1;
				print!("{}", c)
			}
		}
	}

	fn execute(&mut self) {
		let buffer = self.buffer;
		let i = self.b_pos;

		println!();
		match from_utf8(&buffer[..i]) {
			Ok(s) => match s {
				_ if s == "reboot" => reboot::reboot(),
				_ if s == "gdt" => gdt::print_gdt(),
				_ if s == "" => (),
				_ => print!("{}: command not found", s),
			},
			Err(e) => {
				println!("Invalid UTF-8 sequence");
			}
		};

		self.b_pos = 0;
	}
}
