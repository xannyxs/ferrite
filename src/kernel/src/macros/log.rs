//! Macros to easily log

/// Logs a formatted error message including the source location.
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        $crate::tty::log::_log(
            $crate::tty::log::LogLevel::Error,
            file!(),
            line!(),
            module_path!(),
            format_args!($($arg)*)
        );
    }};
}

/// Logs a formatted warning message including the source location.
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        $crate::tty::log::_log(
            $crate::tty::log::LogLevel::Warn,
            file!(),
            line!(),
            module_path!(),
            format_args!($($arg)*)
        );
    }};
}

/// Logs a formatted informational message including the source location.
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        $crate::tty::log::_log(
            $crate::tty::log::LogLevel::Info,
            file!(),
            line!(),
            module_path!(),
            format_args!($($arg)*)
        );
    }};
}

/// Logs a formatted debug message including the source location (only in debug
/// builds).
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            $crate::tty::log::_log(
                $crate::tty::log::LogLevel::Debug,
                file!(),
                line!(),
                module_path!(),
                format_args!($($arg)*)
            );
        }
    }};
}
