use super::*;

/// Provides a default implementation for PanicData.
impl Default for PanicData {
    /// Creates a default PanicData with empty message and no location.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with default values.
    #[inline(always)]
    fn default() -> Self {
        Self {
            message: String::new(),
            location: None,
        }
    }
}

/// Implementation of methods for PanicData.
impl PanicData {
    /// Creates a new PanicData from a panic message.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The panic message.
    ///
    /// # Returns
    ///
    /// - `Self` - A new PanicData instance.
    pub fn from_message<M>(message: M) -> Self
    where
        M: AsRef<str>,
    {
        Self {
            message: message.as_ref().to_owned(),
            location: None,
        }
    }

    /// Creates a new PanicData from a JoinError (for panics in spawned tasks).
    ///
    /// # Arguments
    ///
    /// - `JoinError` - The JoinError from a panicked task.
    ///
    /// # Returns
    ///
    /// - `Self` - A new PanicData instance.
    pub fn from_join_error(error: tokio::task::JoinError) -> Self {
        Self {
            message: error.to_string(),
            location: None,
        }
    }
}

/// Implementation of Display for PanicData.
impl Display for PanicData {
    /// Formats the PanicData for display.
    ///
    /// # Arguments
    ///
    /// - `f` - Formatter for the output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Panic: {}", self.get_message())?;
        if let Some(location) = self.get_location() {
            write!(f, " at {location}")?;
        }
        Ok(())
    }
}

/// Implementation of StdError for PanicData.
impl std::error::Error for PanicData {}
