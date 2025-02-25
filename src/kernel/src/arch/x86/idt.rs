use super::exceptions;
use core::arch::asm;
use kernel_sync::Mutex;
use lazy_static::lazy_static;

/// Number of entries in the x86 Interrupt Descriptor Table (0-255).
/// Covers CPU exceptions (0-31), hardware interrupts (32-47), and software
/// interrupts (48-255).
pub const IDT_ENTRY_COUNT: usize = 256;

lazy_static! {
	/// Global Interrupt Descriptor Table with thread-safe access.
	/// Lazily initialized array of all 256 interrupt handler entries.
	pub static ref IDT_ENTRIES: Mutex<[InterruptDescriptorEntry; IDT_ENTRY_COUNT]> =
		Mutex::new([InterruptDescriptorEntry::default(); IDT_ENTRY_COUNT]);
}

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

impl Default for InterruptDescriptorEntry {
	fn default() -> Self {
		return Self {
			pointer_low: 0,
			selector: 0,
			zero: 0,
			type_attributes: 0b1000_1110,
			pointer_high: 0,
		};
	}
}

#[no_mangle]
#[doc(hidden)]
pub fn idt_init() {
	use core::mem::size_of;

	let mut entries = IDT_ENTRIES.lock();
	for i in 0..IDT_ENTRY_COUNT {
		entries[i] = InterruptDescriptorEntry::default();
	}

	let idt_descriptor = InterruptDescriptorTable {
		size: (size_of::<[InterruptDescriptorEntry; IDT_ENTRY_COUNT]>() - 1)
			as u16,
		offset: &IDT_ENTRIES as *const _ as u32,
	};

	unsafe {
		asm!("lidt [{}]", in(reg) &idt_descriptor);
	}
}
