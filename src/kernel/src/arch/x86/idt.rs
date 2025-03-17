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

use super::exceptions;
use crate::{arch::x86::exceptions::INTERRUPT_HANDLERS, println_serial};
use core::arch::asm;
use kernel_sync::Mutex;
use lazy_static::lazy_static;

#[doc(hidden)]
pub const IDT_ENTRY_COUNT: usize = 256;

/// The location of the IDT is kept in the IDTR (IDT register). This is loaded
/// using the LIDT assembly instruction, whose argument is a pointer to an IDT
/// Descriptor structure.
#[repr(C, packed)]
pub struct InterruptDescriptorTable {
	/// One less than the size of the IDT in bytes.
	size: u16,
	/// The linear address of the Interrupt Descriptor Table (not the
	/// physical address, paging applies).
	offset: u32,
}

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

	pub fn set_handler(&mut self, handler: extern "x86-interrupt" fn()) {
		self.pointer_low = (handler as usize & 0xffff) as u16;
		self.selector = 0x08;
		self.zero = 0;
		self.type_attributes = 0b1000_1110;
		self.pointer_high = ((handler as usize >> 16) & 0xffff) as u16;
	}
}

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
			IDT_ENTRIES[i].set_handler(INTERRUPT_HANDLERS[i]);
		}

		let idt_descriptor = InterruptDescriptorTable {
			size: (size_of::<[InterruptDescriptorEntry; IDT_ENTRY_COUNT]>() - 1)
				as u16,
			offset: &raw const IDT_ENTRIES as *const _ as u32,
		};

		asm!("lidt [{}]", in(reg) &idt_descriptor);
	}
}
