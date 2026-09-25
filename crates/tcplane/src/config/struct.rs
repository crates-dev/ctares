use super::*;

/// Represents the inner, mutable server configuration.
///
/// This structure holds all the settings for the TCP server,
/// including network parameters and buffer sizes.
#[derive(Clone)]
pub struct ServerConfigData {
    /// The host address the server will bind to.
    pub(crate) host: String,
    /// The port number the server will listen on.
    pub(crate) port: u16,
    /// The network buffer size for read operations.
    pub(crate) buffer_size: usize,
}

/// Represents the thread-safe, shareable server configuration.
///
/// This structure wraps `ServerConfigData` in an `Arc<RwLock<ServerConfigData>>`
/// to allow for safe concurrent access and modification of the server settings.
#[derive(Clone)]
pub struct ServerConfig(pub(super) ArcRwLock<ServerConfigData>);
