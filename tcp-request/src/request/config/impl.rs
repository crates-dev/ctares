use super::*;

/// Default implementation for Config.
impl Default for Config {
    /// Creates a default configuration instance.
    ///
    /// # Returns
    ///
    /// - `Config` - A new Config instance with default values.
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
