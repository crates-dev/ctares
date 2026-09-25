/// Creates a new string instance.
///
/// # Arguments
///
/// - `expr` - The expression to initialize the string.
///
/// # Returns
///
/// - `String` - A new string containing the given value.
#[macro_export]
macro_rules! string {
    () => {
        std::string::String::new()
    };
    ($s:expr) => {
        std::string::String::from($s)
    };
}
