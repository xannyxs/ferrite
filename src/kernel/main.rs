#![no_std]
#![no_main]

mod arch;
mod tty;

use crate::tty::tty::{init_tty, terminal_writestring};
use core::panic::PanicInfo;

/// The kernel's name.
pub const NAME: &str = env!("CARGO_PKG_NAME");
/// Current kernel version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
	init_tty();

	terminal_writestring("Kernel Name: ");
	terminal_writestring(NAME);
	terminal_writestring("\nVersion: ");
	terminal_writestring(VERSION);
	terminal_writestring("\n\n");
	terminal_writestring("Hello, Kernel world!\nI am shown in a VM\n");

	loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}
