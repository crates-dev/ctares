use super::*;

/// Configuration for a UDP request.
///
/// This structure holds all the necessary configuration for making a UDP request,
/// including network settings and timeout values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    /// The remote host address to which the UDP request will be sent.
    pub(crate) host: String,
    /// The remote port number to which the UDP request will be sent.
    pub(crate) port: usize,
    /// The request timeout in milliseconds.
    pub(crate) timeout: u64,
    /// The buffer size for receiving data from the remote host.
    pub(crate) buffer_size: usize,
}
