use crate::{
	arch::x86::cpu::reboot,
	libc::console::bin::gdt,
	print, println,
	tty::{tty::WRITER, vga::VGA_HEIGHT},
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
		console
	}

	pub fn add_buffer(&mut self, c: char) {
		match c {
			'\n' => self.execute(),
			'\x08' => self.backspace(),
			c if self.b_pos < self.buffer.len() - 1 => {
				self.buffer[self.b_pos] = c as u8;
				self.b_pos += 1;
				print!("{}", c);
			}
			_ => {} // Buffer full or invalid character
		}
	}

	fn backspace(&mut self) {
		if self.b_pos <= 0 {
			return;
		}

		self.buffer[self.b_pos] = 0;
		self.b_pos -= 1;
		WRITER.lock().clear_char();
	}

	fn execute(&mut self) {
		println!();

		match from_utf8(&self.buffer[..self.b_pos]) {
			Ok(cmd) => match cmd.trim() {
				"reboot" => reboot(),
				"gdt" => gdt::print_gdt(),
				"clear" => self.clear_screen(),
				"help" => self.print_help(),
				"" => {}
				_ => println!("{}: command not found", cmd.trim()),
			},
			Err(_) => println!("Invalid UTF-8 sequence"),
		}

		self.buffer = [0; 256];
		self.b_pos = 0;

		WRITER.lock().set_position(0, VGA_HEIGHT - 1);
		WRITER.lock().clear_line();
		print!("{}", self.prompt);
	}

	#[inline]
	fn clear_screen(&mut self) {
		WRITER.lock().clear_screen();
	}

	fn print_help(&self) {
		println!("Available commands:");
		println!("  reboot  - Restart the system");
		println!("  gdt     - Print Global Descriptor Table");
		println!("  clear   - Clear the screen");
		println!("  help    - Show this help message");
	}
}
