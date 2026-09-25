/// Empty string constant.
pub const EMPTY_STR: &str = "";

/// Default web port number.
pub const DEFAULT_WEB_PORT: usize = 80;

/// Request split marker string.
pub const SPLIT_REQUEST: &str = "\r\n\r\n";

/// Request split marker bytes.
pub const SPLIT_REQUEST_BYTES: &[u8] = SPLIT_REQUEST.as_bytes();

/// Default buffer size for requests.
pub const DEFAULT_BUFFER_SIZE: usize = 512_000;

/// Default timeout value (maximum possible u64 value).
pub const DEFAULT_TIMEOUT: u64 = u64::MAX;
