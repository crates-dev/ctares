/// Represents errors that can occur during encoding.
///
/// These errors are related to character set validation and encoding process.
#[derive(Clone, Copy, Debug, Default)]
pub enum EncodeError {
    /// Indicates invalid character set configuration.
    #[default]
    CharsetError,
}

/// Represents errors that can occur during decoding.
///
/// These errors are related to character set validation and decoding process.
#[derive(Clone, Copy, Debug, Default)]
pub enum DecodeError {
    /// Indicates invalid character set configuration.
    #[default]
    CharsetError,
}
