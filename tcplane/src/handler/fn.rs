use super::*;

/// Creates a handler function from an async function.
///
/// # Arguments
///
/// - `F` - The async function to wrap, satisfying `Fn(Context) -> Fut` where
///   `Fut: Future<Output = ()> + Send + 'static`.
///
/// # Returns
///
/// - `HandlerFunc` - A boxed handler function.
pub fn handler_fn<F, Fut>(func: F) -> HandlerFunc
where
    F: Fn(Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    Box::new(move |ctx: Context| Box::pin(func(ctx)))
}

/// Implementation of server hook handler factory functions.
/// Creates a server hook handler factory from a type implementing `ServerHook`.
///
/// # Returns
///
/// - `ServerHookHandler` - A boxed handler function.
pub fn server_hook_factory<H>() -> ServerHookHandler
where
    H: ServerHook,
{
    Arc::new(|ctx: Context| {
        Box::pin(async move {
            let hook: H = H::new(&ctx).await;
            hook.handle(&ctx).await;
        })
    })
}
