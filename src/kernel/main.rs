#![no_std]
#![no_main]

mod arch;
mod tty;

use core::panic::PanicInfo;
use tty::{tty::WRITER, vga::VgaColour};

/// The kernel's name.
pub const NAME: &str = env!("CARGO_PKG_NAME");
/// Current kernel version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
	use core::fmt::Write;

	WRITER.lock().write_str("Hello\n").unwrap();
	write!(WRITER.lock(), "Hello {}\n", 42).unwrap();

	WRITER
		.lock()
		.colour_code
		.set_foreground_colour(VgaColour::Red);

	WRITER
		.lock()
		.colour_code
		.set_background_colour(VgaColour::LightBlue);
	WRITER.lock().write_str("Hello again\n").unwrap();

	loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}
