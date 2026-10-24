use super::*;

/// Waits for the signal that asks the process to stop.
///
/// The terminate signal is only watched on unix, so the call waits for the
/// interrupt signal on the other systems.
///
/// # Arguments
///
/// - `bool` - Whether the terminate signal also stops the process.
///
/// # Returns
///
/// - `Option<TerminationSignal>` - The signal that arrived, when one did.
pub async fn wait_for_signal(watch_terminate: bool) -> Option<TerminationSignal> {
    #[cfg(unix)]
    {
        if watch_terminate {
            return wait_for_terminate().await;
        }
    }
    let _: bool = watch_terminate;
    wait_for_interrupt().await
}

/// Waits for the signal that asks the process to stop and runs the handler.
///
/// The watcher runs in the background, so the call returns as soon as the
/// task is started.
///
/// # Arguments
///
/// - `bool` - Whether the terminate signal also stops the process.
/// - `F` - The handler that runs when the signal arrives.
pub fn spawn_watcher<F, Fut>(watch_terminate: bool, handler: F)
where
    F: FnOnce(TerminationSignal) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send,
{
    tokio::spawn(async move {
        let signal: Option<TerminationSignal> = wait_for_signal(watch_terminate).await;
        if let Some(signal) = signal {
            handler(signal).await;
        }
    });
}

/// Waits for the interrupt signal.
///
/// # Returns
///
/// - `Option<TerminationSignal>` - The signal that arrived, when one did.
async fn wait_for_interrupt() -> Option<TerminationSignal> {
    let interrupt: Result<(), Error> = tokio::signal::ctrl_c().await;
    if interrupt.is_err() {
        return None;
    }
    Some(TerminationSignal::Interrupted)
}

/// Waits for the terminate signal.
///
/// # Returns
///
/// - `Option<TerminationSignal>` - The signal that arrived, when one did.
#[cfg(unix)]
async fn wait_for_terminate() -> Option<TerminationSignal> {
    match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
        Ok(mut signal) => {
            signal.recv().await;
            Some(TerminationSignal::Terminated)
        }
        Err(_) => None,
    }
}
