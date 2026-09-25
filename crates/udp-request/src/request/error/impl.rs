use super::*;

/// Implements the `StdError` trait for `RequestError`.
impl std::error::Error for RequestError {}

/// Implements the `Display` trait for `RequestError`.
impl Display for RequestError {
    /// Formats the `RequestError` for display.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter<'_>` - The formatter to write the output to.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - The result of the formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUrl => write!(f, "Invalid url"),
            Self::UdpSocketCreateError => write!(f, "Udp socket create error"),
            Self::UdpSocketConnectError => write!(f, "Udp socket connection error"),
            Self::RequestError => write!(f, "Request error"),
            Self::ReadConnectionError => write!(f, "Connection read error"),
            Self::SetReadTimeoutError => write!(f, "Failed to set read timeout"),
            Self::SetWriteTimeoutError => write!(f, "Failed to set write timeout"),
            Self::ReadResponseError => write!(f, "Read response error"),
            Self::SendResponseError(err) => write!(f, "Send response error: {err}"),
        }
    }
}
