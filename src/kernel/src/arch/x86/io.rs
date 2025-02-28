use core::arch::asm;

#[inline]
#[doc(hidden)]
pub fn inb(addr: u16) -> u8 {
	let mut out: u8;

	unsafe {
		asm!("in al, dx", out("al") out, in("dx") addr, options(nomem, nostack, preserves_flags));
	}

	return out;
}

#[inline]
#[doc(hidden)]
pub fn outb(addr: u16, val: u8) {
	unsafe {
		asm!("out dx, al", in("dx") addr, in("al") val, options(nomem, nostack, preserves_flags));
	}
}

#[inline]
#[doc(hidden)]
pub fn inw(addr: u16) -> u16 {
	let mut out: u16;

	unsafe {
		asm!("in al, dx", out("ax") out, in("dx") addr, options(nomem, nostack, preserves_flags));
	}

	return out;
}

#[inline]
#[doc(hidden)]
pub fn outw(addr: u16, val: u16) {
	unsafe {
		asm!("out dx, ax", in("dx") addr, in("ax") val, options(nomem, nostack, preserves_flags));
	}
}

#[inline]
#[doc(hidden)]
pub fn inl(addr: u16) -> u32 {
	let mut out: u32;

	unsafe {
		asm!("in al, dx", out("eax") out, in("dx") addr, options(nomem, nostack, preserves_flags));
	}

	return out;
}

#[inline]
#[doc(hidden)]
pub fn outl(addr: u16, val: u32) {
	unsafe {
		asm!("out dx, eax", in("dx") addr, in("eax") val, options(nomem, nostack, preserves_flags));
	}
}

#[inline]
pub fn io_wait() {
	outb(0x80, 0);
}
