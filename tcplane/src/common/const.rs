use super::*;

/// Default host address for the server.
pub const DEFAULT_HOST: &str = "0.0.0.0";

/// Default listening port for the server.
pub const DEFAULT_PORT: u16 = 8080;

/// Default buffer size for network operations (4KB).
pub const DEFAULT_BUFFER_SIZE: usize = 4096;

/// Default socket address.
pub const DEFAULT_SOCKET_ADDR: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);

/// Request separator bytes for detecting end of request.
pub const SPLIT_REQUEST_BYTES: &[u8] = b"\r\n\r\n";

/// Colon symbol used in address formatting.
pub(crate) const COLON: &str = ":";

/// Colon with space for display formatting.
pub(crate) const COLON_SPACE: &str = ": ";

/// Context attribute key holding the error message passed to read-error handlers.
pub(crate) const CONTEXT_ERROR_KEY: &str = "error";
