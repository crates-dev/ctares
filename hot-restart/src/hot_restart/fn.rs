use super::*;

/// Internal function to handle hot restart process.
///
/// # Arguments
///
/// - `&[&str]` - Arguments to pass to cargo-watch.
/// - `bool` - Whether to wait for process completion.
/// - `F` - The future to run before hot restart.
///
/// # Returns
///
/// - `ResultHotRestartError` - Result of hot restart operation.
async fn run_hot_restart<F>(run_args: &[&str], wait: bool, before_hook: F) -> ResultHotRestartError
where
    F: Future<Output = ()>,
{
    before_hook.await;
    let check_output: Output = Command::new(CARGO_PROGRAM)
        .args([CARGO_INSTALL_ARG, CARGO_LIST_ARG])
        .output()
        .map_err(|error: Error| HotRestartError::Other(error.to_string()))?;
    let check_output_str: Cow<'_, str> = String::from_utf8_lossy(&check_output.stdout);
    if !check_output_str.contains(CARGO_WATCH_PROGRAM) {
        eprintln!("Cargo-watch is not installed. Attempting to install...");
        let install_status: ExitStatus = Command::new(CARGO_PROGRAM)
            .args([CARGO_INSTALL_ARG, CARGO_WATCH_PROGRAM])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
            .wait()?;
        if !install_status.success() {
            return Err(HotRestartError::CargoWatchNotInstalled);
        }
        eprintln!("Cargo-watch installed successfully.");
    }
    let mut command: Command = Command::new(CARGO_WATCH_PROGRAM);
    command
        .args(run_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit());
    let mut child: Child = command
        .spawn()
        .map_err(|error: Error| HotRestartError::CommandSpawnFailed(error.to_string()))?;
    if wait {
        child
            .wait()
            .map_err(|error: Error| HotRestartError::CommandWaitFailed(error.to_string()))?;
    }
    exit(0);
}

/// Starts hot restart process without waiting for completion.
///
/// # Arguments
///
/// - `&[&str]` - Arguments to pass to cargo-watch.
/// - `F` - The future to run before hot restart.
///
/// # Returns
///
/// - `ResultHotRestartError` - Result of hot restart operation.
pub async fn hot_restart<F>(run_args: &[&str], before_hook: F) -> ResultHotRestartError
where
    F: Future<Output = ()>,
{
    run_hot_restart(run_args, false, before_hook).await
}

/// Starts hot restart process and waits for completion.
///
/// # Arguments
///
/// - `&[&str]` - Arguments to pass to cargo-watch.
/// - `F` - The future to run before hot restart.
///
/// # Returns
///
/// - `ResultHotRestartError` - Result of hot restart operation.
pub async fn hot_restart_wait<F>(run_args: &[&str], before_hook: F) -> ResultHotRestartError
where
    F: Future<Output = ()>,
{
    run_hot_restart(run_args, true, before_hook).await
}
