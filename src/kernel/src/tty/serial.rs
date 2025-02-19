#![allow(missing_docs)]

use crate::arch::x86::io::{inb, outb};
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

/* -------------------------------------- */

const PORT: u16 = 0x3f8;

#[derive(Default)]
pub struct Serial {}

// Implement the core::fmt::Write trait so we can use Rust's formatting macros
impl fmt::Write for Serial {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_serial_string(s);
		return Ok(());
	}
}

impl Serial {
	fn is_transmit_empty(&self) -> u8 {
		return inb(PORT + 5) & 0x20;
	}

	fn write_serial_byte(&self, a: u8) {
		while self.is_transmit_empty() == 0 {}

		outb(PORT, a);
	}

	fn write_serial_string(&self, s: &str) {
		for c in s.bytes() {
			self.write_serial_byte(c);
		}
	}

	pub fn init(&self) {
		outb(PORT + 1, 0x00); // Disable all interrupts
		outb(PORT + 3, 0x80); // Enable DLAB (set baud rate divisor)
		outb(PORT, 0x03); // Set divisor to 3 (lo byte) 38400 baud
		outb(PORT + 1, 0x00); //                  (hi byte)
		outb(PORT + 3, 0x03); // 8 bits, no parity, one stop bit
		outb(PORT + 2, 0xc7); // Enable FIFO, clear them, with 14-byte threshold
		outb(PORT + 4, 0x0b); // IRQs enabled, RTS/DSR set
		outb(PORT + 4, 0x1e); // Set in loopback mode, test the serial chip
		outb(PORT, 0xae); // Test serial chip (send byte 0xAE and check if serial returns same
					// byte)

		if inb(PORT) != 0xae {
			panic!("Port: {} unusable", PORT);
		}

		outb(PORT + 4, 0x0f);
	}
}

/* -------------------------------------- */

lazy_static! {
	pub static ref SERIAL: Mutex<Serial> = Mutex::new(Serial::default());
}
