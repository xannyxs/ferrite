//! The Interrupt Descriptor Table (IDT) is a binary data structure specific to
//! the x86-64 architecture. It is the Protected Mode counterpart to the Real
//! Mode Interrupt Vector Table (IVT) telling the CPU where the Interrupt
//! Service Routines (ISR) are located (one per interrupt vector). It is similar
//! to the Global Descriptor Table in structure.
//!
//! The IDT entries are called gates. It can contain Interrupt Gates, Task Gates
//! and Trap Gates.
//!
//! Before you implement the IDT, make sure you have a working GDT.

use super::exceptions::{self, InterruptHandler, InterruptHandlerWithError};
use crate::{
	arch::x86::{
		exceptions::{InterruptHandlerType, INTERRUPT_HANDLERS},
		DescriptorTable,
	},
	println_serial,
};
use core::arch::asm;
use kernel_sync::Mutex;
use lazy_static::lazy_static;

#[doc(hidden)]
pub const IDT_ENTRY_COUNT: usize = 256;

/// An Interrupt Descriptor Table entry.
///
/// The generic parameter can either be `HandlerFunc` or
/// `HandlerFuncWithErrCode`, depending on the interrupt vector.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct InterruptDescriptorEntry {
	pointer_low: u16,    // offset bits 0..15
	selector: u16,       // a code segment selector in GDT or LDT
	zero: u8,            // unused, set to 0
	type_attributes: u8, // gate type, dpl, and p fields
	pointer_high: u16,   // offset bits 16..31
}

impl InterruptDescriptorEntry {
	const fn new() -> Self {
		return Self {
			pointer_low: 0,
			selector: 0,
			zero: 0,
			type_attributes: 0b1000_1110,
			pointer_high: 0,
		};
	}

	/// Configures an IDT entry with the specified interrupt handler
	pub fn set_handler(&mut self, handler: InterruptHandler) {
		self.pointer_low = (handler as usize & 0xffff) as u16;
		self.selector = 0x08;
		self.zero = 0;
		self.type_attributes = 0b1000_1110;
		self.pointer_high = ((handler as usize >> 16) & 0xffff) as u16;
	}

	pub fn set_handler_with_error_code(
		&mut self,
		handler: InterruptHandlerWithError,
	) {
		self.pointer_low = (handler as usize & 0xffff) as u16;
		self.selector = 0x08;
		self.zero = 0;
		self.type_attributes = 0b1000_1110;
		self.pointer_high = ((handler as usize >> 16) & 0xffff) as u16;
	}
}

/// Static array of 256 IDT entries, zero-initialized
///
/// Lives for the duration of kernel execution
pub static mut IDT_ENTRIES: [InterruptDescriptorEntry; IDT_ENTRY_COUNT] =
	[InterruptDescriptorEntry::new(); IDT_ENTRY_COUNT];

/// Initializes the Interrupt Descriptor Table (IDT) for the system.
///
/// It should be called during early boot before interrupts are enabled.
///
/// # Safety
///
/// This function uses the `lidt` assembly instruction which directly
/// interacts with the CPU. Improper IDT setup can cause system crashes
/// if interrupt handlers point to invalid code.
#[no_mangle]
pub fn idt_init() {
	use core::mem::size_of;
	unsafe {
		for i in 0..INTERRUPT_HANDLERS.len() {
			match INTERRUPT_HANDLERS[i] {
				InterruptHandlerType::Regular(handler) => {
					IDT_ENTRIES[i].set_handler(handler);
				}
				InterruptHandlerType::WithErrorCode(handler) => {
					IDT_ENTRIES[i].set_handler_with_error_code(handler);
				}
			}
		}

		let idt_descriptor = DescriptorTable {
			size: (size_of::<[InterruptDescriptorEntry; IDT_ENTRY_COUNT]>() - 1)
				as u16,
			offset: &raw const IDT_ENTRIES as *const _ as u32,
		};

		asm!("lidt [{}]", in(reg) &idt_descriptor);
	}
}
