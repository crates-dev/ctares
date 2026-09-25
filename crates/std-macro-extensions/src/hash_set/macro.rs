/// Creates a new `HashSet` instance.
///
/// This macro can be used in two forms:
/// - Without arguments, it creates an empty `HashSet`.
/// - With elements, it creates a `HashSet` and inserts the provided elements into it.
#[macro_export]
macro_rules! hash_set {
    () => {
        std::collections::HashSet::new()
    };
    ($($elem:expr),*) => {{
        let mut set = std::collections::HashSet::new();
        $( set.insert($elem); )*
        set
    }};
}
