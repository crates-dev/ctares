/// An empty string slice.
///
/// Used as a default value for various string fields.
pub const EMPTY_STR: &str = "";

/// The default port for web connections.
///
/// This is the standard port for HTTP.
pub const DEFAULT_WEB_PORT: usize = 80;

/// The default buffer size for I/O operations.
///
/// Set to 512 KB.
pub const DEFAULT_BUFFER_SIZE: usize = 512_000;

/// The default timeout value for network operations.
///
/// Set to the maximum value of `u64`.
pub const DEFAULT_TIMEOUT: u64 = u64::MAX;
