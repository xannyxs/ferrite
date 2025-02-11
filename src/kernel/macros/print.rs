use crate::tty::tty::WRITER;
use core::fmt;

/// Prints formatted text to the VGA buffer.
///
/// This macro works similarly to the standard library's `print!` macro but
/// writes directly to the VGA text buffer instead of standard output. It
/// accepts format strings and arguments just like the standard `print!` macro.
///
/// # Examples
///
/// ```
/// print!("Hello"); // Prints "Hello"
/// print!("Number: {}", 42); // Prints "Number: 42"
/// print!("{value}", value = "text"); // Prints "text"
/// ```
///
/// # Implementation Details
///
/// The macro forwards its arguments to the internal `_print` function using
/// `format_args!`. This avoids heap allocations since the formatting is handled
/// directly by the VGA writer. The actual printing is handled by the VGA buffer
/// writer which must be properly initialized before using this macro.
///
/// # Panics
///
/// This macro will panic if the VGA writer is not properly initialized or if
/// there are any issues with the VGA buffer lock.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::macros::print::_print(format_args!($($arg)*)));
}

/// Prints formatted text to the VGA buffer, followed by a newline.
///
/// This macro is identical to `print!` but automatically adds a newline at the
/// end of the output. It provides the same formatting capabilities as `print!`
/// while ensuring each output ends on its own line.
///
/// # Examples
///
/// ```
/// println!(); // Prints just a newline
/// println!("Hello"); // Prints "Hello" followed by a newline
/// println!("Value: {}", 42); // Prints "Value: 42" followed by a newline
/// ```
///
/// # Implementation Details
///
/// When called without arguments, it simply prints a newline character.
/// When called with arguments, it combines the formatted text with a newline
/// before sending it to the VGA buffer writer. This is done without any heap
/// allocation by using the `format_args!` macro internally.
///
/// # Panics
///
/// This macro will panic under the same conditions as `print!`: if the VGA
/// writer is not initialized or if there are issues with the VGA buffer lock.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;
	WRITER.lock().write_fmt(args).unwrap();
}
