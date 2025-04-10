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
#![feature(linked_list_cursors)]
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
#![deny(clippy::implicit_return)] // Force return keyword
#![allow(clippy::needless_return)] // Allow return keyword

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
// pub mod collections;
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

use crate::{arch::x86::multiboot::G_SEGMENTS, sync::Mutex};
use alloc::{boxed::Box, format};
use arch::x86::{
	cpu::halt,
	multiboot::{get_memory_region, MultibootInfo, MultibootMmapEntry},
};
use core::{alloc::Layout, arch::asm, ffi::c_void};
use device::keyboard::Keyboard;
use libc::console::console::Console;
use memory::{
	allocator::{BUDDY_PAGE_ALLOCATOR, EARLY_PHYSICAL_ALLOCATOR},
	buddy::BuddyAllocator,
	memblock::MemBlockAllocator,
	PAGE_SIZE,
};
use tty::{
	log::{Logger, StatusProgram},
	serial::SERIAL,
};

extern crate alloc;

/* -------------------------------------- */

const MAGIC_VALUE: u32 = 0x2badb002;

/* extern "C" {
	fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
	fn memset(str: *mut c_void, c: i32, len: usize) -> *mut c_void;
	fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> i32;
} */

/* -------------------------------------- */

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn kernel_main(
	magic_number: u32,
	boot_info: &'static MultibootInfo,
) -> ! {
	Logger::init("Kernel", Some("Starting initialization"));
	Logger::divider();
	Logger::newline();

	if magic_number != MAGIC_VALUE {
		panic!(
			"Incorrect magic number. Current magic number: 0x{:x}",
			magic_number
		);
	}

	if (boot_info.flags & 0x7) != 0x7 {
		let flags = boot_info.flags;

		panic!(
        "Required flags not set. Expected MBALIGN, MEMINFO, and VIDEO to be set, but flag value is: 0b{:b}",
        flags
    );
	}

	Logger::init(
		"Memory Management",
		Some("Starting memory subsystem initialization"),
	);

	Logger::init_step(
		"Memory Detection",
		"Reading memory map from bootloader",
		true,
	);
	SERIAL.lock().init();

	let mut segments = G_SEGMENTS.lock();
	get_memory_region(&mut segments, boot_info);

	Logger::init_step(
		"Memblock Allocator",
		"Initializing early memory allocator",
		true,
	);

	{
		// To avoid deadlocks, we will need to use a temporary block
		let mut memblock = EARLY_PHYSICAL_ALLOCATOR.lock();
		memblock.get_or_init(MemBlockAllocator::new);
		match memblock.get_mut() {
    Some(alloc) => {
        alloc.init(&mut segments);
        Logger::ok("Memblock Allocator", Some("Initialization successful"));
    },
    None => panic!(
        "Failed to initialize memory block allocator: unable to retrieve mutable reference. \
        This could indicate that the allocator was not properly initialized or was dropped unexpectedly. \
        Check EARLY_PHYSICAL_ALLOCATOR implementation."
    ),
};
	}

	Logger::init_step(
		"Buddy Allocator",
		"Initializing buddy page allocator",
		true,
	);

	{
		let mut buddy_allocator = BUDDY_PAGE_ALLOCATOR.lock();

		#[allow(clippy::implicit_return)]
		buddy_allocator.get_or_init(|| BuddyAllocator::new(&segments));
		match buddy_allocator.get_mut() {
			Some(_) => {
				Logger::ok("Buddy Allocator", Some("Initialized successfully"));
			}
			None => panic!("Was not able to initialize the Buddy Allocator"),
		}
	}

	{
		let mut buddy_allocator = BUDDY_PAGE_ALLOCATOR.lock();

		match buddy_allocator.get_mut() {
			Some(a) => unsafe {
				let layout = Layout::from_size_align(PAGE_SIZE * 2, PAGE_SIZE)
					.expect("Error while creating the Buddy Allocation Layout");

				println_serial!("ALLOCATING");
				a.alloc(layout);
				println_serial!("DONE");
			},
			None => panic!("Was not able to initialize the Buddy Allocator"),
		}
	}

	Logger::divider();
	Logger::status("Memory Management", &StatusProgram::OK);

	/* let test = Box::new("Hallo wereld");
	println_serial!("{}", test);
	let another_test = Box::new("cool");
	println_serial!("{}", test);
	println_serial!("{}", another_test); */

	let mut keyboard = Keyboard::default();
	let mut console = Console::default();

	#[cfg(test)]
	test_main();

	loop {
		let c = match keyboard.input() {
			Some(key) => key,
			None => continue,
		};

		console.add_buffer(c);
	}
}
