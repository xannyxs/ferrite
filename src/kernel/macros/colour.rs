/// Changes the foreground color of the VGA text output
#[macro_export]
macro_rules! set_fg_color {
	($colour:expr) => {{
		use $crate::tty::{tty::WRITER, vga::VgaColour};
		WRITER.lock().colour_code.set_foreground_colour($colour);
	}};
}

/// Changes the background color of the VGA text output
#[macro_export]
macro_rules! set_bg_color {
	($colour:expr) => {{
		use $crate::tty::{tty::WRITER, vga::VgaColour};
		WRITER.lock().colour_code.set_background_colour($colour);
	}};
}

/// Temporarily changes the foreground color for a block of code
#[macro_export]
macro_rules! with_fg_color {
    ($colour:expr, $($code:tt)*) => {{
        use $crate::tty::{tty::WRITER, vga::VgaColour};
        let original = WRITER.lock().colour_code.get_foreground_colour();
        WRITER.lock().colour_code.set_foreground_colour($colour);
        let result = { $($code)* };
        WRITER.lock().colour_code.set_foreground_colour(original);
        result
    }};
}

/// Temporarily changes the background color for a block of code
#[macro_export]
macro_rules! with_bg_color {
    ($colour:expr, $($code:tt)*) => {{
        use $crate::tty::{tty::WRITER, vga::VgaColour};
        let original = WRITER.lock().colour_code.get_background_colour();
        WRITER.lock().colour_code.set_background_colour($colour);
        let result = { $($code)* };
        WRITER.lock().colour_code.set_background_colour(original);
        result
    }};
}

/// Temporarily changes both foreground and background colors
#[macro_export]
macro_rules! with_colors {
    ($fg:expr, $bg:expr, $($code:tt)*) => {{
        use $crate::tty::{tty::WRITER, vga::VgaColour};
        let original_fg = WRITER.lock().colour_code.get_foreground_colour();
        let original_bg = WRITER.lock().colour_code.get_background_colour();
        WRITER.lock().colour_code.set_foreground_colour($fg);
        WRITER.lock().colour_code.set_background_colour($bg);
        let result = { $($code)* };
        WRITER.lock().colour_code.set_foreground_colour(original_fg);
        WRITER.lock().colour_code.set_background_colour(original_bg);
        result
    }};
}
