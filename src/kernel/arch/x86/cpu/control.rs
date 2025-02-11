use core::arch::asm;

#[inline]
pub unsafe fn cli() {
	asm!("cli", options(nomem, nostack));
}

#[inline]
pub unsafe fn halt() {
	asm!("hlt", options(nomem, nostack));
}

#[inline]
pub unsafe fn halt_loop() -> ! {
	loop {
		halt();
	}
}
