use super::*;

/// Executes a recoverable function within a panic-safe context.
///
/// # Arguments
///
/// - `F` - Function implementing RecoverableFunction.
///
/// # Returns
///
/// - `SyncSpawnResult` - The spawn operation result.
pub fn run_function<F: RecoverableFunction>(func: F) -> SyncSpawnResult {
    set_hook(Box::new(move |_| {}));
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        func();
    }))
}

/// Executes an error-handling function within a panic-safe context.
///
/// # Arguments
///
/// - `E` - Function implementing ErrorHandlerFunction.
/// - `&str` - The error message.
///
/// # Returns
///
/// - `SyncSpawnResult` - The spawn operation result.
pub fn run_error_handle_function<E: ErrorHandlerFunction>(func: E, error: &str) -> SyncSpawnResult {
    set_hook(Box::new(move |_| {}));
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        func(error);
    }))
}

/// Converts a panic-captured error value into a string.
///
/// # Arguments
///
/// - `&SpawnError` - The captured error value.
///
/// # Returns
///
/// - `String` - The error string representation.
pub fn spawn_error_to_string(err: &SpawnError) -> String {
    match err.downcast_ref::<&str>() {
        Some(str_slice) => str_slice.to_string(),
        None => match err.downcast_ref::<String>() {
            Some(string) => string.to_owned(),
            None => format!("{err:?}"),
        },
    }
}

/// Spawns a recoverable function.
///
/// # Arguments
///
/// - `F` - Function implementing RecoverableFunction.
///
/// # Returns
///
/// - `SyncSpawnResult` - The spawn operation result.
pub fn recoverable_spawn<F>(function: F) -> SyncSpawnResult
where
    F: RecoverableFunction,
{
    run_function(function)
}

/// Spawns a recoverable function with error handling.
///
/// # Arguments
///
/// - `F` - Function implementing RecoverableFunction.
/// - `E` - Function implementing ErrorHandlerFunction.
///
/// # Returns
///
/// - `SyncSpawnResult` - The spawn operation result.
pub fn recoverable_spawn_catch<F, E>(function: F, error_handle_function: E) -> SyncSpawnResult
where
    F: RecoverableFunction,
    E: ErrorHandlerFunction,
{
    let run_result: SyncSpawnResult = run_function(function);
    if let Err(err) = run_result.as_ref() {
        let err_string: String = spawn_error_to_string(err);
        let _: SyncSpawnResult = run_error_handle_function(error_handle_function, &err_string);
    }
    run_result
}

/// Spawns a recoverable function with error handling and finalization.
///
/// # Arguments
///
/// - `F` - Function implementing RecoverableFunction.
/// - `E` - Function implementing ErrorHandlerFunction.
/// - `L` - Function implementing RecoverableFunction.
///
/// # Returns
///
/// - `SyncSpawnResult` - The spawn operation result.
pub fn recoverable_spawn_catch_finally<F, E, L>(
    function: F,
    error_handle_function: E,
    finally: L,
) -> SyncSpawnResult
where
    F: RecoverableFunction,
    E: ErrorHandlerFunction,
    L: RecoverableFunction,
{
    let run_result: SyncSpawnResult = run_function(function);
    if let Err(err) = run_result.as_ref() {
        let err_string: String = spawn_error_to_string(err);
        let _: SyncSpawnResult = run_error_handle_function(error_handle_function, &err_string);
    }
    let _: SyncSpawnResult = run_function(finally);
    run_result
}
