use crate::{println, println_serial, with_fg_color};
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	with_fg_color!(VgaColour::Red, {
		println!("{}", info);
		println_serial!("{}", info);
	});

	loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	use crate::tests::{exit_qemu, QFAILURE};

	with_fg_color!(VgaColour::Red, {
		println_serial!("[failed]\n");
		println_serial!("Error: {}\n", _info);
	});

	exit_qemu(QFAILURE);
	loop {}
}
