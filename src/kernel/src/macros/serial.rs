use crate::tty::serial::SERIAL;
use core::fmt;

#[doc(hidden)]
#[macro_export]
macro_rules! print_serial {
    ($($arg:tt)*) => ($crate::macros::serial::_print_serial(format_args!($($arg)*)));
}

#[doc(hidden)]
#[macro_export]
macro_rules! println_serial {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_serial!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print_serial(args: fmt::Arguments) {
	use core::fmt::Write;
	SERIAL.lock().write_fmt(args).unwrap();
}
