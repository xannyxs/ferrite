use core::arch::asm;

#[inline]
#[doc(hidden)]
pub fn cli() {
	unsafe {
		asm!("cli", options(nomem, nostack));
	}
}

#[inline]
#[doc(hidden)]
pub fn halt() {
	unsafe {
		asm!("hlt", options(nomem, nostack));
	}
}

#[inline]
#[doc(hidden)]
pub fn halt_loop() -> ! {
	loop {
		halt();
	}
}
