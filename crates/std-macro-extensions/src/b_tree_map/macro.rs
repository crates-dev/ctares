/// Creates a new BTreeMap instance.
///
/// # Arguments
///
/// - `expr` - The key-value pairs to initialize the BTreeMap.
///
/// # Returns
///
/// - `BTreeMap<K, V>` - A new BTreeMap containing the given key-value pairs.
#[macro_export]
macro_rules! b_tree_map {
    () => {
        std::collections::BTreeMap::new()
    };
    ($($key:expr => $val:expr),*) => {{
        let mut map = std::collections::BTreeMap::new();
        $( map.insert($key, $val); )*
        map
    }};
}
