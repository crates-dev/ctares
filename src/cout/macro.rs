/// Print formatted output to standard output using `print!`.
///
/// # Parameters
/// - `args`: A format string followed by optional expressions, just like in `print!`.
///
/// # Returns
/// - Nothing. This macro prints directly to standard output.
#[macro_export]
macro_rules! cout {
    ($($args: tt)*) => {
        ::std::print!($($args)*);
        let _: ::std::io::Result<()> = ::std::io::Write::flush(&mut ::std::io::stdout());
    };
}

/// Print a newline character and flush the standard output buffer.
///
/// # Parameters
/// - (none): This macro takes no arguments.
///
/// # Returns
/// - Nothing. This macro prints `\n` and flushes the output.
#[macro_export]
macro_rules! endl {
    () => {{
        $crate::cout!("\n");
    }};
}

/// Print formatted output with a newline and flush the standard output buffer.
///
/// # Parameters
/// - `args`: A format string followed by optional expressions, just like in `println!`.
///
/// # Returns
/// - Nothing. This macro prints to standard output and flushes the buffer.
#[macro_export]
macro_rules! cout_endl {
    ($($args:tt)*) => {
        $crate::cout!($($args)*);
        $crate::endl!();
    };
}
