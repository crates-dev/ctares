use super::*;

/// Implementation of server hook handler factory functions.
/// Creates a server hook handler factory from a type implementing `ServerHook`.
///
/// # Arguments
///
/// - `ServerHook` - The hook type that implements `ServerHook`.
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
