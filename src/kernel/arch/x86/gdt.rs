//------------------------------------------------------------------------------
// Global Descriptor Table Implementation (gdt.rs)
//
// This module implements the GDT entry structure and manipulation methods.
// Each GDT entry (Gate) is a 64-bit structure with the following layout:
//
// Base Address:  32 bits (split across bits 16-39 and 56-63)
// Segment Limit: 20 bits (bits 0-15 and 48-51)
// Access Byte:   8 bits  (bits 40-47)
//   - Present:     Bit 7 (P)
//   - DPL:         Bits 5-6 (Ring Level)
//   - Type:        Bit 4 (S)
//   - Type flags:  Bits 0-3
// Flags:        4 bits  (bits 52-55)
//   - Granularity: Bit 3 (G)
//   - Size:        Bit 2 (D/B)
//   - Long mode:   Bit 1 (L)
//   - Reserved:    Bit 0
//
// For more information go to:
// https://wiki.osdev.org/Global_Descriptor_Table
//------------------------------------------------------------------------------

#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct Gate(pub u64);

/// Must be packed to maintain exact CPU-required layout
#[repr(C, packed)]
pub struct GDTDescriptor {
	pub size: u16,
	pub offset: u32,
}

#[allow(unused)]
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

		c
	}

	#[inline]
	pub fn base(&mut self) -> u32 {
		(((self.0 >> 16) & 0xffffff) | ((self.0 >> 56) & 0xff) << 24) as u32
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
		((self.0 & 0xffff) | (((self.0 >> 48) & 0xf) << 16)) as u32
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
		(self.0 >> 40) as u8
	}

	#[inline]
	pub const fn set_access(&mut self, access: u8) {
		self.0 &= !(0xff << 40);
		self.0 |= (access as u64) << 40;
	}

	#[inline]
	pub fn flags(&mut self) -> u8 {
		((self.0 >> 52) & 0x0f) as u8
	}

	#[inline]
	pub const fn set_flags(&mut self, flags: u8) {
		self.0 &= !(0xf << 52);
		self.0 |= (flags as u64) << 52;
	}
}
