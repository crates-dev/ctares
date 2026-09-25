/// Creates a new BTreeSet instance.
///
/// # Arguments
///
/// - `expr` - The elements to initialize the BTreeSet.
///
/// # Returns
///
/// - `BTreeSet<T>` - A new BTreeSet containing the given elements.
#[macro_export]
macro_rules! b_tree_set {
    () => {
        std::collections::BTreeSet::new()
    };
    ($($elem:expr),*) => {{
        let mut set = std::collections::BTreeSet::new();
        $( set.insert($elem); )*
        set
    }};
}
