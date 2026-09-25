/// Creates a new hash map instance.
///
/// # Arguments
///
/// - `expr` - The key-value pairs to initialize the hash map.
///
/// # Returns
///
/// - `HashMap<K, V>` - A new hash map containing the given key-value pairs.
#[macro_export]
macro_rules! hash_map {
    () => {
        std::collections::HashMap::new()
    };
    ($($key:expr => $val:expr),*) => {{
        let mut map = std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
    }};
}
