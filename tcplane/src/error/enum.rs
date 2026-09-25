/// Represents errors that can occur at the server level.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServerError {
    /// An error occurred while trying to bind to a TCP socket.
    TcpBind(String),
    /// An error occurred while reading from TCP stream.
    TcpRead(String),
    /// An error occurred while writing to TCP stream.
    TcpWrite(String),
    /// The connection was closed unexpectedly.
    ConnectionClosed,
    /// An unknown or unexpected error occurred.
    Unknown(String),
}

/// Represents errors related to response operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResponseError {
    /// The stream was not found.
    NotFoundStream,
    /// The connection has been terminated.
    Terminated,
    /// An error occurred while writing to the stream.
    WriteError(String),
    /// An error occurred while flushing the stream.
    FlushError(String),
}
