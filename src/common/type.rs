use super::*;

/// Error type for spawn operations.
pub type SpawnError = Box<dyn Any + Send>;

/// Result type for asynchronous spawn operations.
///
/// # Returns
///
/// - `Result<(), JoinError>` - The spawn operation result.
pub type AsyncSpawnResult = Result<(), JoinError>;

/// Result type for synchronous spawn operations.
///
/// # Returns
///
/// - `Result<(), SpawnError>` - The spawn operation result.
pub type SyncSpawnResult = Result<(), SpawnError>;

/// Arc-wrapped asynchronous recoverable function.
///
/// # Type Parameters
///
/// - `O` - The output type.
/// - `F` - The future type.
pub type ArcAsyncRecoverableFunction<O, F> =
    Arc<dyn AsyncRecoverableFunction<Output = O, Future = F>>;

/// Arc-wrapped asynchronous error handler function.
///
/// # Type Parameters
///
/// - `O` - The future type.
pub type ArcAsyncErrorHandlerFunction<O> = Arc<dyn AsyncErrorHandlerFunction<Future = O>>;
