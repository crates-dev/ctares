use super::*;

/// Trait for server lifecycle hooks that process connections.
///
/// `ServerHook` provides a unified interface for different types of connection
/// processing handlers in the server lifecycle.
///
/// This trait is designed to work with the server's connection processing pipeline.
pub trait ServerHook: Send + Sync + 'static {
    /// Creates a new instance of this hook from the context.
    ///
    /// This method is called by the framework to instantiate the hook.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The connection context containing all request/response data.
    ///
    /// # Returns
    ///
    /// A future that resolves to a new instance of this hook.
    fn new(ctx: &Context) -> impl Future<Output = Self> + Send;

    /// Executes the hook's processing logic.
    ///
    /// This method contains the actual logic for processing the connection.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The connection context for accessing request/response data.
    ///
    /// # Returns
    ///
    /// A future that resolves when the processing is complete.
    fn handle(self, ctx: &Context) -> impl Future<Output = ()> + Send;
}

/// Trait for async function handlers.
///
/// This trait is used for handlers that return a future directly.
pub trait AsyncFuncWithoutPin<Fut>: Fn(Context) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send + 'static,
{
}

/// Blanket implementation for all types implementing the required bounds.
impl<T, Fut> AsyncFuncWithoutPin<Fut> for T
where
    T: Fn(Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
}
