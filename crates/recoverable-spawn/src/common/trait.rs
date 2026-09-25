use super::*;

/// Trait alias for functions that can be executed in a recoverable context.
///
/// - Functions implementing this trait must satisfy `FnOnce() + Send + Sync + 'static`.
pub trait FunctionOnceTrait: FnOnce() + Send + Sync + 'static {}

impl<T> FunctionOnceTrait for T where T: FnOnce() + Send + Sync + 'static {}

/// Trait alias for functions that can be executed in a recoverable context.
///
/// - Functions implementing this trait must satisfy `Fn() + Send + Sync + 'static`.
pub trait FunctionTrait: Fn() + Send + Sync + 'static {}

impl<T> FunctionTrait for T where T: Fn() + Send + Sync + 'static {}

/// Trait alias for functions that can be executed in a recoverable context.
///
/// - Functions implementing this trait must satisfy `FnMut() + Send + Sync + 'static`.
pub trait FunctionMutTrait: FnMut() + Send + Sync + 'static {}

impl<T> FunctionMutTrait for T where T: FnMut() + Send + Sync + 'static {}

/// Trait alias for asynchronous functions that can be executed in a recoverable context.
///
/// # Arguments
///
/// - `FnOnce() -> Future` - Function that returns a Future.
///
/// # Returns
///
/// - `Future` - The asynchronous computation result.
pub trait AsyncRecoverableFunction: Send + Sync + 'static {
    type Output: Send;
    type Future: Future<Output = Self::Output> + Send;

    /// Executes the asynchronous function.
    fn call(self) -> Self::Future;
}

impl<F, Fut, O> AsyncRecoverableFunction for F
where
    F: FnOnce() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = O> + Send + 'static,
    O: Send + 'static,
{
    type Output = O;
    type Future = Fut;

    fn call(self) -> Self::Future {
        self()
    }
}

/// Trait alias for asynchronous error-handling functions used in a recoverable context.
///
/// # Arguments
///
/// - `Arc<String>` - The error message to handle.
///
/// # Returns
///
/// - `Future` - The asynchronous error handling result.
pub trait AsyncErrorHandlerFunction: Send + Sync + 'static {
    type Future: Future<Output = ()> + Send;

    /// Handles an error asynchronously.
    ///
    /// - `error`: The error message to handle.
    fn call(self, error: Arc<String>) -> Self::Future;
}

impl<F, Fut> AsyncErrorHandlerFunction for F
where
    F: FnOnce(Arc<String>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    type Future = Fut;

    fn call(self, error: Arc<String>) -> Self::Future {
        self(error)
    }
}

/// Trait alias for functions that can be executed in a recoverable context.
///
/// - Functions implementing this trait must satisfy `FnOnce() + Send + Sync + 'static`.
pub trait RecoverableFunction: FnOnce() + Send + Sync + 'static {}

impl<T> RecoverableFunction for T where T: FnOnce() + Send + Sync + 'static {}

/// Trait alias for error-handling functions used in a recoverable context.
///
/// # Arguments
///
/// - `&str` - The error message to handle.
pub trait ErrorHandlerFunction: FnOnce(&str) + Send + Sync + 'static {}

impl<T> ErrorHandlerFunction for T where T: FnOnce(&str) + Send + Sync + 'static {}
