/// Execute a synchronous function with given arguments.
///
/// # Parameters
/// - `$path`: The function path.
/// - `$array`: The primary argument.
/// - `$arg`...: Optional trailing arguments.
///
/// # Returns
/// - The result of the function call.
#[macro_export]
macro_rules! execute {
    ($path: path, $array: expr) => {
        $path($array)
    };
    ($path: path, $array: expr, $($arg: expr), *) => {
        $path($array, $($arg), *)
    };
}

/// Execute an asynchronous function and return a future.
///
/// # Parameters
/// - `$path`: The async function path.
/// - `$array`: The primary argument.
/// - `$arg`...: Optional trailing arguments.
///
/// # Returns
/// - A future that must be `.await`ed.
#[macro_export]
macro_rules! execute_async {
    ($path: path, $array: expr) => {
        async {
            $path($array).await
        }
    };
    ($path: path, $array: expr, $($arg: expr),*) => {
        async {
            $path($array, $($arg),*).await
        }
    };
}
