#![no_std]
#![no_main]

mod arch;
mod device;
mod libc;
mod tty;

use core::panic::PanicInfo;
use device::keyboard::get_keyboard_input;
use tty::{tty::WRITER, vga::VgaColour};

/// The kernel's name.
pub const NAME: &str = env!("CARGO_PKG_NAME");
/// Current kernel version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
	println!("shelly >>");

	loop {
		if let Some(code) = get_keyboard_input() {
			match code {
				28 => println!(),
				8 => print!("\x08"),
				_ => println!("input: {}", code),
			}
		}
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
