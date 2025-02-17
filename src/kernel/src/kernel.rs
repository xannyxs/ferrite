//! The kernel starts here.
//!
//! Here all the lint rules & additional compile rules will be declared.
//!
//! The program is designed in a Unix-like way under the x86 (i386)
//! architecturethe
//!
//! This is not meant as an actual Kernel, and should not be used in production.

#![no_std] // Don't link to standard library - essential for kernels
#![no_main] // Don't use normal entry points - we define our own
#![feature(strict_provenance_lints)] // Enable stricter pointer safety checks
#![deny(fuzzy_provenance_casts)] // Enforce proper pointer provenance

// Safety and Documentation
#![deny(missing_docs)] // Require documentation for public items
#![deny(unsafe_op_in_unsafe_fn)] // Require explicit unsafe blocks even in unsafe functions
#![deny(rustdoc::broken_intra_doc_links)] // Catch broken documentation links

// Code Quality
#![deny(unreachable_pub)] // Catch unnecessarily public items
#![deny(unused_must_use)] // Enforce handling of Result/Option returns
#![deny(unused_crate_dependencies)] // Catch unused dependencies
#![deny(clippy::unwrap_used)] // Prevent unwrap() in kernel code
#![deny(clippy::expect_used)] // Prevent expect() in kernel code

// Memory Safety
#![deny(invalid_reference_casting)] // Prevent dangerous reference casts

// Style and Consistency
#![allow(clippy::tabs_in_doc_comments)] // Your existing allowance for tabs
#![deny(clippy::implicit_clone)] // Make cloning explicit
#![deny(clippy::needless_pass_by_value)] // Optimize parameter passing

// Development Helpers
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

/* -------------------------------------- */

/// Specific Bare Metal support
pub mod arch;
/// Device Support - Keyboard & Mouse
pub mod device;
/// Libc - STD Library (Should move in future)
pub mod libc;
/// Macro directory
pub mod macros;
/// TTY Support - Specifically VGA
pub mod tty;

use core::panic::PanicInfo;
use device::keyboard::Keyboard;
use libc::console::console::Console;

/* -------------------------------------- */

/// The kernel's name.
pub const NAME: &str = env!("CARGO_PKG_NAME");
/// Current kernel version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/* -------------------------------------- */

#[no_mangle]
#[doc(hidden)]
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
	with_fg_color!(VgaColour::Red, {
		println!("{}", _info);
	});

	loop {}
}
