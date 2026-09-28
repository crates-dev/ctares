use super::*;

/// Represents the inner, mutable server configuration.
///
/// This structure holds all the settings for the UDP server,
/// including network parameters and buffer sizes. It is not intended to be used directly
/// by end-users, but rather through the `ServerConfig` wrapper.
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct ServerConfigData {
    /// The host address the server will bind to.
    #[set(type(AsRef<str>))]
    pub(super) host: String,
    /// The port number the server will listen on.
    #[get(type(copy))]
    pub(super) port: u16,
    /// The buffer size for receiving UDP packets.
    #[get(type(copy))]
    pub(super) buffer_size: usize,
    /// The `TCP_NODELAY` option for sockets (applied when applicable).
    #[get(type(copy))]
    pub(super) nodelay: Option<bool>,
    /// The `IP_TTL` option for sockets (applied when applicable).
    #[get(type(copy))]
    pub(super) ttl: Option<u32>,
}

/// Represents the thread-safe, shareable server configuration.
///
/// This structure wraps `ServerConfigData` in an `Arc<RwLock<ServerConfigData>>`
/// to allow for safe concurrent access and modification of the server settings.
#[derive(Clone, Debug)]
pub struct ServerConfig(pub(super) ArcRwLock<ServerConfigData>);
