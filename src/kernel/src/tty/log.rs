use crate::{
	print, println, println_serial,
	tty::vga::{ColourCode, VgaColour},
	with_fg_color,
};
use core::fmt;

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
	Error,
	Warn,
	Info,
	Debug,
}

#[allow(missing_docs, unused)]
pub fn _log(
	level: LogLevel,
	file: &str,
	line: u32,
	module: &str,
	args: fmt::Arguments,
) {
	let (level_str, color) = match level {
		LogLevel::Error => ("[ERROR]", VgaColour::Red),
		LogLevel::Warn => ("[WARN]", VgaColour::Yellow),
		LogLevel::Info => ("[INFO]", VgaColour::LightCyan),
		LogLevel::Debug => ("[DEBUG]", VgaColour::LightGreen),
	};

	with_fg_color!(color, {
		println!("[{}] {} {}", format_args!("{}", module), level_str, args);
	});
}
