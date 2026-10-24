/// The signal that asks a process to stop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminationSignal {
    /// The process was interrupted.
    Interrupted,
    /// The process was terminated.
    Terminated,
}
