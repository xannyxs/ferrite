use crate::{
	print, println, println_serial,
	tty::vga::{ColourCode, VgaColour},
	with_fg_color,
};

#[derive(Debug)]
pub enum StatusProgram {
	OK,
	WARNING,
	ERROR,
	INIT,
	RUNNING,
	STOPPED,
	FAILED,
}

impl core::fmt::Display for StatusProgram {
	#[allow(clippy::implicit_return)]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			StatusProgram::OK => write!(f, "OK"),
			StatusProgram::WARNING => write!(f, "WARNING"),
			StatusProgram::ERROR => write!(f, "ERROR"),
			StatusProgram::INIT => write!(f, "INIT"),
			StatusProgram::RUNNING => write!(f, "RUNNING"),
			StatusProgram::STOPPED => write!(f, "STOPPED"),
			StatusProgram::FAILED => write!(f, "FAILED"),
		}
	}
}

pub struct Logger;

impl Logger {
	pub fn error(msg: &str) {
		with_fg_color!(VgaColour::Red, {
			println!("[ERROR] {}", msg);
		});
	}

	pub fn warn(msg: &str) {
		with_fg_color!(VgaColour::Yellow, {
			println!("[WARN]  {}", msg);
		});
	}

	pub fn info(msg: &str) {
		with_fg_color!(VgaColour::LightCyan, {
			println!("[INFO]  {}", msg);
		});
	}

	pub fn debug(msg: &str) {
		#[cfg(debug_assertions)]
		{
			with_fg_color!(VgaColour::LightGrey, {
				println!("[DEBUG] {}", msg);
			});
		}
	}

	pub fn status(program: &str, status: &StatusProgram) {
		let status_colour = match status {
			StatusProgram::OK => VgaColour::Green,
			StatusProgram::WARNING => VgaColour::Yellow,
			StatusProgram::ERROR => VgaColour::Red,
			StatusProgram::INIT => VgaColour::Cyan,
			StatusProgram::RUNNING => VgaColour::LightGreen,
			StatusProgram::STOPPED => VgaColour::DarkGrey,
			StatusProgram::FAILED => VgaColour::LightRed,
		};

		print!("{} ", program);

		with_fg_color!(status_colour, {
			println!("[ {} ]", status);
		});
	}

	pub fn component_status(
		component: &str,
		status: &StatusProgram,
		message: Option<&str>,
	) {
		let status_colour = match status {
			StatusProgram::OK => VgaColour::Green,
			StatusProgram::WARNING => VgaColour::Yellow,
			StatusProgram::ERROR => VgaColour::Red,
			StatusProgram::INIT => VgaColour::Cyan,
			StatusProgram::RUNNING => VgaColour::LightGreen,
			StatusProgram::STOPPED => VgaColour::DarkGrey,
			StatusProgram::FAILED => VgaColour::LightRed,
		};

		with_fg_color!(status_colour, {
			print!("[ {} ]", status);
		});

		with_fg_color!(VgaColour::LightGrey, {
			print!(" {}", component);

			if let Some(msg) = message {
				match status {
					StatusProgram::OK
					| StatusProgram::INIT
					| StatusProgram::RUNNING => {
						println!(": {}", msg);
					}
					StatusProgram::WARNING => {
						print!(": ");
						with_fg_color!(VgaColour::Yellow, {
							println!("{}", msg);
						});
					}
					StatusProgram::ERROR | StatusProgram::FAILED => {
						print!(": ");
						with_fg_color!(VgaColour::Red, {
							println!("{}", msg);
						});
					}
					_ => {
						println!(": {}", msg);
					}
				}
			} else {
				println!();
			}
		});
	}

	// Even more concise status reporting methods for common cases

	pub fn ok(component: &str, message: Option<&str>) {
		Self::component_status(component, &StatusProgram::OK, message);
	}

	pub fn warn_component(component: &str, message: &str) {
		Self::component_status(
			component,
			&StatusProgram::WARNING,
			Some(message),
		);
	}

	pub fn error_component(component: &str, message: &str) {
		Self::component_status(component, &StatusProgram::ERROR, Some(message));
	}

	pub fn init(component: &str, message: Option<&str>) {
		Self::component_status(component, &StatusProgram::INIT, message);
	}

	/// A method for reporting progress during initialization
	pub fn init_step(component: &str, step: &str, success: bool) {
		with_fg_color!(VgaColour::LightGrey, {
			print!("  - {} - {} ", component, step);
		});

		if success {
			with_fg_color!(VgaColour::Green, {
				println!("[OK]");
			});
		} else {
			with_fg_color!(VgaColour::Red, {
				println!("[FAILED]");
			});
		}
	}

	/// A method to create a divider line for visual separation
	pub fn divider() {
		with_fg_color!(VgaColour::LightGrey, {
			println!("----------------------------------------");
		});
	}

	pub fn newline() {
		println!();
	}
}
