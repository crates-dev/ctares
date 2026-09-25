/// Creates a new `Rc` (Reference Counted) instance.
///
/// This macro takes a value and wraps it in an `Rc`, providing shared ownership of the value.
/// Multiple references to the same value can be made, and the value will be dropped when the last reference goes out of scope.
#[macro_export]
macro_rules! rc {
    ($val:expr) => {
        std::rc::Rc::new($val)
    };
}
