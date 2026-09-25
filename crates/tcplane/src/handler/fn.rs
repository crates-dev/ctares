use super::*;

/// Creates a handler function from an async function.
///
/// # Arguments
///
/// - `Fn(Context) -> Fut + Send + Sync + 'static` - The async function to wrap.
/// - `Future<Output = ()> + Send + 'static` - The future type returned by the function.
///
/// # Returns
///
/// - `HandlerFunc` - A boxed handler function.
pub fn handler_fn<F, Fut>(func: F) -> HandlerFunc
where
    F: Fn(Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    Box::new(move |ctx| Box::pin(func(ctx)))
}
