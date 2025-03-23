//! The Global Descriptor Table (GDT) is a binary data structure specific to the
//! x86-64 architecture. It contains entries telling the CPU about
//! memory segments. A similar Interrupt Descriptor Table exists containing task
//! and interrupt descriptors.
//!
//! For more information go to:
//! <https://wiki.osdev.org/Global_Descriptor_Table>

use super::DescriptorTable;
use crate::arch::x86::diagnostics::cpu::check_protection_status;

extern "C" {
	// src/arch/{target}/gdt.asm
	fn gdt_flush(gdt_ptr: *const DescriptorTable);
}

/// Entries in the table are accessed by Segment Selectors, which are loaded
/// into Segmentation registers either by assembly instructions or by hardware
/// functions such as Interrupts.
#[repr(C, align(8))]
pub struct Gate(pub u64);

/// Represents the complete Global Descriptor Table containing 5 descriptor
/// entries:
/// - Entry 0: Null Descriptor (required by CPU)
/// - Entry 1: Kernel Code Segment
/// - Entry 2: Kernel Data Segment
/// - Entry 3: User Code Segment
/// - Entry 4: User Data Segment
pub type GdtGates = [Gate; 5];

#[doc(hidden)]
impl Gate {
	/// Creates a new GDT entry with specified parameters
	///
	/// # Arguments
	/// * `base` - 32-bit base address of the segment
	/// * `limit` - 20-bit size of the segment
	/// * `access` - 8-bit access flags (present, DPL, type)
	/// * `flags` - 4-bit flags (granularity, size, long mode)
	pub const fn new(base: u32, limit: u32, access: u8, flags: u8) -> Self {
		let mut c = Self(0);
		c.set_base(base);
		c.set_limit(limit);
		c.set_access(access);
		c.set_flags(flags);

		return c;
	}

	#[inline]
	pub fn base(&mut self) -> u32 {
		return (((self.0 >> 16) & 0xffffff) | (((self.0 >> 56) & 0xff) << 24))
			as u32;
	}

	#[inline]
	pub const fn set_base(&mut self, base: u32) {
		self.0 &= !(0xffffff << 16);
		self.0 &= !(0xff << 56);

		self.0 |= (base as u64 & 0xffffff) << 16;
		self.0 |= ((base as u64 >> 24) & 0xff) << 56;
	}

	#[inline]
	pub fn limit(&mut self) -> u32 {
		return ((self.0 & 0xffff) | (((self.0 >> 48) & 0xf) << 16)) as u32;
	}

	#[inline]
	pub const fn set_limit(&mut self, limit: u32) {
		self.0 &= !0xffff;
		self.0 &= !(0xf << 48);

		self.0 |= limit as u64 & 0xffff;
		self.0 |= ((limit as u64 >> 16) & 0xf) << 48;
	}

	#[inline]
	pub fn access(&mut self) -> u8 {
		return (self.0 >> 40) as u8;
	}

	#[inline]
	pub const fn set_access(&mut self, access: u8) {
		self.0 &= !(0xff << 40);
		self.0 |= (access as u64) << 40;
	}

	#[inline]
	pub fn flags(&mut self) -> u8 {
		return ((self.0 >> 52) & 0x0f) as u8;
	}

	#[inline]
	pub const fn set_flags(&mut self, flags: u8) {
		self.0 &= !(0xf << 52);
		self.0 |= (flags as u64) << 52;
	}
}

#[no_mangle]
#[link_section = ".gdt"]
static GDT_ENTRIES: GdtGates = [
	Gate(0), // [0] Null Descriptor (CPU requirement)
	Gate::new(0, !0, 0b10011010, 0b1100), // [1] Kernel Code: Ring 0, executable
	Gate::new(0, !0, 0b10010010, 0b1100), // [2] Kernel Data: Ring 0, writable
	Gate::new(0, !0, 0b11111010, 0b1100), // [3] User Code: Ring 3, executable
	Gate::new(0, !0, 0b11110010, 0b1100), // [4] User Data: Ring 3, writable
];

/// Initializes the Global Descriptor Table (GDT) for the system.
/// It should be called during early boot.
///
/// # Safety
///
/// This function uses calls the assembly instruction, which is called in
/// `gdt_flush`.
#[no_mangle]
pub fn gdt_init() {
	use core::mem::size_of;

	let gdt_descriptor = DescriptorTable {
		size: (size_of::<GdtGates>() - 1) as u16,
		offset: &GDT_ENTRIES as *const _ as u32,
	};

	unsafe {
		gdt_flush(&gdt_descriptor as *const _);
	}

	check_protection_status();
}
