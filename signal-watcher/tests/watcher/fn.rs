use super::*;

#[test]
fn names_the_interrupt_signal() {
    let signal: TerminationSignal = TerminationSignal::Interrupted;
    assert_eq!(signal.message(), "the process was interrupted");
    assert_eq!(signal.to_string(), "the process was interrupted");
}

#[test]
fn names_the_terminate_signal() {
    let signal: TerminationSignal = TerminationSignal::Terminated;
    assert_eq!(signal.message(), "the process was terminated");
    assert_eq!(signal.to_string(), "the process was terminated");
}

#[tokio::test]
async fn starts_the_watcher() {
    spawn_watcher(false, |_: TerminationSignal| async {});
}

#[cfg(unix)]
#[tokio::test]
async fn waits_for_the_terminate_signal() {
    let _signal: Signal = tokio::signal::unix::signal(SignalKind::terminate()).unwrap();
    let watcher: JoinHandle<Option<TerminationSignal>> =
        tokio::spawn(async { wait_for_signal(true).await });
    tokio::time::sleep(Duration::from_millis(200)).await;
    let status: ExitStatus = Command::new("kill")
        .args(["-TERM", &id().to_string()])
        .status()
        .unwrap();
    assert!(status.success());
    let signal: Option<TerminationSignal> = tokio::time::timeout(Duration::from_secs(10), watcher)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(signal, Some(TerminationSignal::Terminated));
}

#[cfg(unix)]
#[tokio::test]
async fn runs_the_handler_when_the_signal_arrives() {
    let _signal: Signal = tokio::signal::unix::signal(SignalKind::terminate()).unwrap();
    let (sender, receiver): (
        oneshot::Sender<TerminationSignal>,
        oneshot::Receiver<TerminationSignal>,
    ) = tokio::sync::oneshot::channel();
    spawn_watcher(true, move |signal: TerminationSignal| {
        let sender: oneshot::Sender<TerminationSignal> = sender;
        async move {
            let _: Result<(), TerminationSignal> = sender.send(signal);
        }
    });
    tokio::time::sleep(Duration::from_millis(200)).await;
    let status: ExitStatus = Command::new("kill")
        .args(["-TERM", &id().to_string()])
        .status()
        .unwrap();
    assert!(status.success());
    let signal: TerminationSignal = tokio::time::timeout(Duration::from_secs(10), receiver)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(signal, TerminationSignal::Terminated);
}
