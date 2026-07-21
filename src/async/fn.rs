use super::*;

/// Executes a recoverable function within a panic-safe context.
///
/// # Arguments
///
/// - `F` - Function implementing AsyncRecoverableFunction.
///
/// # Returns
///
/// - `AsyncSpawnResult` - The spawn operation result.
pub async fn async_run_function<F: AsyncRecoverableFunction>(func: F) -> AsyncSpawnResult {
    set_hook(Box::new(move |_| {}));
    let func = async move {
        func.call().await;
    };
    tokio::spawn(func).await
}

/// Executes an error-handling function within a panic-safe context.
///
/// # Arguments
///
/// - `E` - Function implementing AsyncErrorHandlerFunction.
/// - `Arc<String>` - The error message.
///
/// # Returns
///
/// - `AsyncSpawnResult` - The spawn operation result.
pub async fn async_run_error_handle_function<E: AsyncErrorHandlerFunction>(
    func: E,
    error: Arc<String>,
) -> AsyncSpawnResult {
    set_hook(Box::new(move |_| {}));
    let func = async move {
        func.call(error.clone()).await;
    };
    tokio::spawn(func).await
}

/// Converts a panic-captured error value into a string.
///
/// # Arguments
///
/// - `&JoinError` - The captured error value.
///
/// # Returns
///
/// - `String` - The error string representation.
pub fn tokio_error_to_string(err: &JoinError) -> String {
    err.to_string()
}

/// Spawns a recoverable function.
///
/// # Arguments
///
/// - `F` - Function implementing AsyncRecoverableFunction.
///
/// # Returns
///
/// - `AsyncSpawnResult` - The spawn operation result.
pub async fn async_recoverable_spawn<F>(function: F) -> AsyncSpawnResult
where
    F: AsyncRecoverableFunction,
{
    async_run_function(function).await
}

/// Spawns a recoverable function with error handling.
///
/// # Arguments
///
/// - `F` - Function implementing AsyncRecoverableFunction.
/// - `E` - Function implementing AsyncErrorHandlerFunction.
///
/// # Returns
///
/// - `AsyncSpawnResult` - The spawn operation result.
pub async fn async_recoverable_spawn_catch<F, E>(
    function: F,
    error_handle_function: E,
) -> AsyncSpawnResult
where
    F: AsyncRecoverableFunction,
    E: AsyncErrorHandlerFunction,
{
    let run_result: AsyncSpawnResult = async_run_function(function).await;
    if let Err(err) = run_result.as_ref() {
        let err_string: String = tokio_error_to_string(err);
        let _: AsyncSpawnResult =
            async_run_error_handle_function(error_handle_function, Arc::new(err_string)).await;
    }
    run_result
}

/// Spawns a recoverable function with error handling and finalization.
///
/// # Arguments
///
/// - `F` - Function implementing AsyncRecoverableFunction.
/// - `E` - Function implementing AsyncErrorHandlerFunction.
/// - `L` - Function implementing AsyncRecoverableFunction.
///
/// # Returns
///
/// - `AsyncSpawnResult` - The spawn operation result.
pub async fn async_recoverable_spawn_catch_finally<F, E, L>(
    function: F,
    error_handle_function: E,
    finally: L,
) -> AsyncSpawnResult
where
    F: AsyncRecoverableFunction,
    E: AsyncErrorHandlerFunction,
    L: AsyncRecoverableFunction,
{
    let run_result: AsyncSpawnResult = async_run_function(function).await;
    if let Err(err) = run_result.as_ref() {
        let err_string: String = tokio_error_to_string(err);
        let _: AsyncSpawnResult =
            async_run_error_handle_function(error_handle_function, Arc::new(err_string)).await;
    }
    let _: AsyncSpawnResult = async_run_function(finally).await;
    run_result
}
