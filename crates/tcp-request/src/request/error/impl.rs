use super::*;

/// Standard error implementation for RequestError.
impl std::error::Error for RequestError {}

/// Display formatting implementation for RequestError.
impl Display for RequestError {
    /// Formats the error for display.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter<'_>` - The formatter to write to.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - The result of the formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUrl => write!(f, "Invalid URL"),
            Self::TcpStreamConnectError => write!(f, "TCP Stream Connection Error"),
            Self::RequestError => write!(f, "Request Error"),
            Self::ReadConnectionError => write!(f, "Connection Read Error"),
            Self::SetReadTimeoutError => write!(f, "Failed to Set Read Timeout"),
            Self::SetWriteTimeoutError => write!(f, "Failed to Set Write Timeout"),
            Self::ReadResponseError => write!(f, "Read response error"),
        }
    }
}
