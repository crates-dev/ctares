/// An enumeration of possible errors that can occur during a UDP request.
///
/// This enum covers a range of potential issues, from socket creation to network timeouts.
#[derive(Debug)]
pub enum RequestError {
    /// An error indicating that the provided URL is invalid.
    InvalidUrl,
    /// An error indicating that the UDP socket could not be created.
    UdpSocketCreateError,
    /// An error indicating that the UDP socket could not connect to the specified address.
    UdpSocketConnectError,
    /// A general request error, used for unspecified issues.
    RequestError,
    /// An error indicating that reading from the connection failed.
    ReadConnectionError,
    /// An error indicating that setting the read timeout for the socket failed.
    SetReadTimeoutError,
    /// An error indicating that setting the write timeout for the socket failed.
    SetWriteTimeoutError,
    /// An error indicating that reading the response from the socket failed.
    ReadResponseError,
    /// An error indicating that sending the request failed, with a descriptive message.
    SendResponseError(String),
}
