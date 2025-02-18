use crate::{
	arch::x86::cpu::reboot,
	libc::console::bin::gdt,
	print, println,
	tty::{tty::WRITER, vga::VGA_HEIGHT},
};
use core::str::from_utf8;

#[doc(hidden)]
pub struct Console {
	b_pos: usize,
	buffer: [u8; 256],
	prompt: &'static str,
}

impl Console {
	/// Creates a new console instance with default settings. Only one should
	/// exist in the kernel.
	///
	/// This function initializes a new console with an empty command buffer and
	/// the default shell prompt "[shelly]$ ". The console is immediately ready
	/// for input after creation, as it displays the prompt right away.
	///
	/// # Returns
	/// Returns a new `Console` instance ready to accept input.
	/// ```
	pub fn new() -> Self {
		let console = Console {
			b_pos: 0,
			buffer: [0; 256],
			prompt: "[shelly]$ ",
		};
		print!("{}", console.prompt);
		console
	}

	/// Processes a single character of input for the shell.
	///
	/// This function handles all input to the shell, managing special
	/// characters and building up the command buffer character by character.
	/// It supports regular typing, command execution (on newline), and
	/// backspace for editing.
	///
	/// # Arguments
	/// * `c` - The character to process, which can be:
	///   - A printable character (stored in buffer and displayed)
	///   - Newline ('\n') to execute the current command
	///   - Backspace ('\x08') to delete the last character
	///
	/// # Behavior
	/// - Regular characters are stored in the buffer and displayed
	/// - Newline triggers command execution
	/// - Backspace removes the last character
	/// - Buffer overflow and invalid characters are ignored
	///
	/// # Implementation Details
	/// The function maintains a buffer position (b_pos) that:
	/// - Increases with each added character
	/// - Is bounded by the buffer size (256 bytes)
	/// - Is adjusted when backspace is used
	///
	/// # Example
	/// ```
	/// shell.add_buffer('l'); // Types 'l'
	/// shell.add_buffer('s'); // Types 's'
	/// shell.add_buffer('\n'); // Executes "ls" command
	/// ```
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
				"panic" => panic!("Test panic"),
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
