use super::*;

impl TerminationSignal {
    /// The message that names the signal.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The message of the signal.
    pub fn message(&self) -> &'static str {
        match self {
            Self::Interrupted => MESSAGE_INTERRUPTED,
            Self::Terminated => MESSAGE_TERMINATED,
        }
    }
}

impl fmt::Display for TerminationSignal {
    /// Writes the message that names the signal.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter<'_>` - The formatter the message is written to.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Nothing, or the reason the message could not be
    ///   written.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())
    }
}
