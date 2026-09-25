use super::*;

/// Configuration for TCP request settings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Config {
    /// Target host address.
    pub(crate) host: String,
    /// Target port number.
    pub(crate) port: usize,
    /// Request timeout in seconds.
    pub(crate) timeout: u64,
    /// Buffer size for data transfer.
    pub(crate) buffer_size: usize,
}
