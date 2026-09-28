/// Macro for outputting colored text to the terminal.
///
/// # Arguments
///
/// - `ColorOutput` or `ColorOutputBuilder` - One or more output instances to execute
#[macro_export]
macro_rules! output_macro {
    ($($output:expr),*) => {
        $($output.output();)*
    };
}

/// Prints a success message with green background and white text.
///
/// Supports format string syntax like the `format!` macro:
/// - `println_success!(body)` - Simple string with no arguments
/// - `println_success!(body, name)` - Positional arguments replaced by `{}`
/// - `println_success!(body, name = value)` - Named arguments replaced by `{name}` (Rust 1.58+)
#[macro_export]
macro_rules! println_success {
    ($($arg:tt)*) => {
        $crate::__println_text(ColorType::Use(Color::White), ColorType::Use(Color::Green), &format!($($arg)*));
    };
}

/// Prints a warning message with yellow background and white text.
///
/// Supports format string syntax like the `format!` macro:
/// - `println_warning!(body)` - Simple string with no arguments
/// - `println_warning!(body, error)` - Positional arguments replaced by `{}`
/// - `println_warning!(body, error = value)` - Named arguments replaced by `{error}` (Rust 1.58+)
#[macro_export]
macro_rules! println_warning {
    ($($arg:tt)*) => {
        $crate::__println_text(ColorType::Use(Color::White), ColorType::Use(Color::Yellow), &format!($($arg)*));
    };
}

/// Prints an error message with red background and white text.
///
/// Supports format string syntax like the `format!` macro:
/// - `println_error!(body)` - Simple string with no arguments
/// - `println_error!(body, message)` - Positional arguments replaced by `{}`
/// - `println_error!(body, message = value)` - Named arguments replaced by `{message}` (Rust 1.58+)
#[macro_export]
macro_rules! println_error {
    ($($arg:tt)*) => {
        $crate::__println_text(ColorType::Use(Color::White), ColorType::Use(Color::Red), &format!($($arg)*));
    };
}
