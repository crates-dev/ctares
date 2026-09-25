use super::*;

/// Implements the `Default` trait for the `Config` struct.
impl Default for Config {
    /// Creates a default `Config` instance.
    ///
    /// # Returns
    ///
    /// - `Self` - A `Config` instance with default values: an empty host, default web port, maximum timeout, and default buffer size.
    #[inline(always)]
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
            buffer_size: DEFAULT_BUFFER_SIZE,
            host: EMPTY_STR.to_owned(),
            port: DEFAULT_WEB_PORT,
        }
    }
}
