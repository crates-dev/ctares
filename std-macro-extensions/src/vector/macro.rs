/// Creates a new vector instance.
///
/// # Arguments
///
/// - `expr` - The expression(s) to initialize the vector.
///
/// # Returns
///
/// - `Vec<T>` - A new vector containing the given elements.
#[macro_export]
macro_rules! vector {
    () => {
        std::vec::Vec::new()
    };
    ($($elem:expr),*) => {
        std::vec::Vec::from([$($elem),*])
    };
}
