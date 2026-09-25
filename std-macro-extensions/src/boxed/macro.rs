/// Creates a new `Box` instance.
///
/// This macro takes an expression and wraps it in a `Box`. It offers a more concise syntax compared to the standard `Box::new` function.
#[macro_export]
macro_rules! boxed {
    ($val:expr) => {
        std::boxed::Box::new($val)
    };
}
