/// Creates a new atomic reference counted pointer.
///
/// # Arguments
///
/// - `expr` - The value to be reference counted.
///
/// # Returns
///
/// - `Arc<T>` - A thread-safe reference-counting pointer.
#[macro_export]
macro_rules! arc {
    ($val:expr) => {
        std::sync::Arc::new($val)
    };
}
