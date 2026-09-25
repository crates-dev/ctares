/// Error types for request operations.
#[derive(Debug)]
pub enum RequestError {
    /// Invalid URL provided.
    InvalidUrl,
    /// Failed to establish TCP connection.
    TcpStreamConnectError,
    /// General request error.
    RequestError,
    /// Error reading from connection.
    ReadConnectionError,
    /// Failed to set read timeout.
    SetReadTimeoutError,
    /// Failed to set write timeout.
    SetWriteTimeoutError,
    /// Error reading response.
    ReadResponseError,
}
