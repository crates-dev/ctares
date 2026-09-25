/// Creates a new `VecDeque` instance.
///
/// This macro can be used in two forms:
/// - Without arguments, it creates an empty `VecDeque`.
/// - With elements, it creates a `VecDeque` initialized with the provided elements.
#[macro_export]
macro_rules! vector_deque {
    () => {
        std::collections::VecDeque::new()
    };
    ($($elem:expr),*) => {
        std::collections::VecDeque::from([$($elem),*])
    };
}
