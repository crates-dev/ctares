/// Creates a new `BinaryHeap<T>`.
///
/// This macro provides two ways to initialize a `BinaryHeap`:
///
/// 1. **Empty Heap**:
///    - Calling `binary_heap!()` creates an empty `BinaryHeap`.
///
/// 2. **With Elements**:
///    - You can initialize a `BinaryHeap` with elements by providing a comma-separated list of values.
///    - This will create a `BinaryHeap` containing the specified elements.
#[macro_export]
macro_rules! binary_heap {
    () => {
        std::collections::BinaryHeap::new()
    };
    ($($elem:expr),*) => {
        std::collections::BinaryHeap::from(vec![$($elem),*])
    };
}
