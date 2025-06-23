use crate::memory::{PhysAddr, VirtAddr};
use core::{arch::asm, option};

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

#[inline]
#[doc(hidden)]
pub fn cr3() -> PhysAddr {
	let cr3: usize;

	unsafe {
		asm!("mov {}, cr3", out(reg) cr3, options(nomem, nostack, preserves_flags))
	};

	PhysAddr::new(cr3)
}

#[inline]
#[doc(hidden)]
pub fn cr2() -> VirtAddr {
	let cr2: usize;

	unsafe {
		asm!("mov {}, cr2", out(reg) cr2, options(nomem, nostack, preserves_flags))
	};

	VirtAddr::new(cr2)
}

#[inline(always)]
#[doc(hidden)]
pub fn invlpg(addr: VirtAddr) {
	unsafe {
		asm!("invlpg [{}]", in(reg) addr.as_usize(), options(nostack, preserves_flags));
	}
}
