use super::*;

/// Provides encoding functionality.
impl Encode {
    /// Encodes input string using specified character set.
    ///
    /// Processes 3-byte chunks to produce 4-character encoded segments.
    ///
    /// # Arguments
    ///
    /// - `&str` - The character set used for encoding.
    /// - `&str` - String to encode.
    ///
    /// # Returns
    ///
    /// - `Result<String, EncodeError>` - Encoded string or error.
    pub fn execute(charset: &str, encode_str: &str) -> Result<String, EncodeError> {
        if !Charset::judge_charset_safe(charset) {
            return Err(EncodeError::CharsetError);
        }
        let mut result: String = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        for &byte in encode_str.as_bytes() {
            buffer.extend_from_slice(&[0, 0, byte]);
        }
        for chunk in buffer.chunks(3) {
            let combined: usize =
                ((chunk[0] as usize) << 16) | ((chunk[1] as usize) << 8) | (chunk[2] as usize);
            for i in (0..4).rev() {
                let idx: usize = (combined >> (i * 6)) & 0b111111;
                result.push(charset.chars().nth(idx).unwrap_or_default());
            }
        }
        Ok(result)
    }
}
