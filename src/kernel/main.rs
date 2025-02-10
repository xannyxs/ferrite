#![no_std]
#![no_main]

mod arch;
mod device;
mod libc;
mod tty;

use core::panic::PanicInfo;
use device::keyboard::Keyboard;
use libc::console::console::Console;
use tty::{tty::WRITER, vga::VgaColour};

/// The kernel's name.
pub const NAME: &str = env!("CARGO_PKG_NAME");
/// Current kernel version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
	let mut keyboard = Keyboard::new();
	let mut console = Console::new();

	loop {
		let c = match keyboard.input() {
			Some(key) => key,
			None => continue,
		};

		console.add_buffer(c);
	}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	WRITER
		.lock()
		.colour_code
		.set_foreground_colour(VgaColour::Red);

	println!("{}", _info);

	WRITER
		.lock()
		.colour_code
		.set_foreground_colour(VgaColour::LightGrey);

	loop {}
}
