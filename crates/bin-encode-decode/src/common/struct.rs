/// Handles string encoding and decoding operations.
///
/// Uses a custom character set for the encoding/decoding process.
///
/// The character set should contain unique characters and ideally have 64 characters
/// for base64-like encoding.
#[derive(Clone, Copy, Debug)]
pub struct Charset<'a>(
    /// Reference to the character set used for encoding/decoding.
    pub &'a str,
);
