/// Creates a new `Cell` instance.
///
/// This macro takes an expression and wraps it in a `Cell`, providing interior mutability for the given value.
/// The `Cell` type allows for mutation of its contents even when shared among multiple references.
#[macro_export]
macro_rules! cell {
    ($val:expr) => {
        std::cell::Cell::new($val)
    };
}
