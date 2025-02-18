#![allow(missing_docs)]

use crate::{
	arch::x86::{
		cpu::halt_loop,
		io::{outb, outl},
	},
	print_serial, println_serial,
};
use core::any::type_name;

pub mod unit;

const QPORT: u16 = 0xf4;
pub const QSUCCES: u32 = 0x10;
pub const QFAILURE: u32 = 0x11;

pub fn exit_qemu(exit_code: u32) {
	outl(QPORT, exit_code);
}

pub trait Testable {
	fn run(&self);
}

impl<T> Testable for T
where
	T: Fn(),
{
	fn run(&self) {
		print_serial!("{}...\t", type_name::<T>());
		self();
		println_serial!("[ok]");
	}
}

pub fn test_runner(tests: &[&dyn Testable]) {
	println_serial!("Running {} tests", tests.len());

	for test in tests {
		test.run();
	}

	exit_qemu(QSUCCES);
}
