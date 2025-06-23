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

// Testing
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]
// Safety and Documentation
#![feature(strict_provenance_lints)] // Enable stricter pointer safety checks
#![feature(abi_x86_interrupt)]
#![feature(dropck_eyepatch)]
#![feature(linked_list_cursors)]
#![feature(allocator_api)]
#![deny(fuzzy_provenance_casts)] // Enforce proper pointer provenance
#![warn(missing_docs)] // Require documentation for public items
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
/// Collectiosn - Datatypes and structures
pub mod collections;
/// Device Support - Keyboard & Mouse
pub mod device;
/// Libc - STD Library (Should move in future)
pub mod libc;
/// Macro directory
pub mod macros;
pub mod memory;
/// Panic
pub mod panic;
pub mod sync;
/// Tests
pub mod tests;
/// TTY Support - Specifically VGA
pub mod tty;

use alloc::boxed::Box;
use arch::x86::multiboot::MultibootInfo;
use core::ffi::c_void;
use device::keyboard::Keyboard;
use libc::console::console::Console;
use memory::{allocator::memory_init, frame::FRAME_ALLOCATOR, FrameAllocator};
use tty::serial::SERIAL;

extern crate alloc;

/* -------------------------------------- */

const MAGIC_VALUE: u32 = 0x2badb002;

extern "C" {
	/* fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
	fn memset(str: *mut c_void, c: i32, len: usize) -> *mut c_void;
	fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> i32; */
}

/* -------------------------------------- */

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn kernel_main(
	magic_number: u32,
	boot_info: &'static MultibootInfo,
) -> ! {
	if magic_number != MAGIC_VALUE {
		panic!("Incorrect magic number.");
	}

	SERIAL.lock().init();

	memory_init(boot_info);

	let mut keyboard = Keyboard::default();
	let mut console = Console::default();

	#[cfg(test)]
	test_main();

	let test1 = Box::new("Test 1");
	let test2 = Box::new(10);

	println_serial!("{} - {}", test1, test2);

	loop {
		let c = match keyboard.input() {
			Some(key) => key,
			None => continue,
		};

		console.add_buffer(c);
	}
}
