/// Creates a new `LinkedList` instance.
///
/// This macro can be used in two forms:
/// - Without arguments, it creates an empty `LinkedList`.
/// - With elements, it creates a `LinkedList` and appends the provided elements to the back of the list.
#[macro_export]
macro_rules! linked_list {
    () => {
        std::collections::LinkedList::new()
    };
    ($($elem:expr),*) => {{
        std::collections::LinkedList::from([$($elem),*])
    }};
}
