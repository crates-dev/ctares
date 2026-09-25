/// Creates a new `RwLock` instance.
///
/// This macro takes a value and wraps it in an `RwLock`, providing thread-safe access with multiple readers or a single writer.
/// `RwLock` allows multiple immutable borrows or one mutable borrow at a time, enforcing read-write access at the runtime level.
#[macro_export]
macro_rules! rw_lock {
    ($val:expr) => {
        std::sync::RwLock::new($val)
    };
}
