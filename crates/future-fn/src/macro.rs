/// Generates an asynchronous closure that clones specified external variables and executes the given closure body asynchronously.
///
/// This macro supports two forms:
/// 1. An async closure with no parameters.
/// 2. An async closure with specified parameters.
#[macro_export]
macro_rules! future_fn {
    ($($var:ident),*, { $($closure_body:tt)* }) => {
        || {
            #[allow(unused_parens)]
            let ($($var),*) = ($($var.clone()),*);
            async move {
                $($closure_body)*
            }
        }
    };
    ($($var:ident),*, |$( $closure_param:ident $(: $closure_param_ty:ty)? ),*| { $($closure_body:tt)* }) => {
        {
            #[allow(unused_parens)]
            let ($($var),*) = ($($var.clone()),*);
            move |$( $closure_param $(: $closure_param_ty)? ),*| {
                let ($($var),*) = ($($var.clone()),*);
                async move {
                    $($closure_body)*
                }
            }
        }
    };
}
