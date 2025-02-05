#![no_std]
#![no_main]

mod arch;
mod tty;

use crate::tty::tty::Writer;
use core::panic::PanicInfo;
use tty::vga::{ColourCode, VgaColour};

/// The kernel's name.
pub const NAME: &str = env!("CARGO_PKG_NAME");
/// Current kernel version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
	let mut writer =
		Writer::new(ColourCode::new(VgaColour::LightGrey, VgaColour::Black));

	writer.write_string("Kernel Name:");
	writer.write_string(NAME);
	writer.write_string("\nVersion: ");
	writer.write_string(VERSION);
	writer.write_string("\n\n");
	writer.write_string("Hello, Kernel world!\nI am shown in a VM\n");

	loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}
