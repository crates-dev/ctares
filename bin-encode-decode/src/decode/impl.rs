use super::*;

impl Decode {
    /// Decodes an encoded string using specified character set.
    ///
    /// Processes 4-character groups to reconstruct original bytes by mapping each
    /// character to its position in the charset.
    ///
    /// # Arguments
    ///
    /// - `&str` - The character set used for decoding.
    /// - `&str` - The string to decode.
    ///
    /// # Returns
    ///
    /// - `Result<String, DecodeError>` - Result of decoding operation.
    pub fn execute(charset: &str, decode_str: &str) -> Result<String, DecodeError> {
        if !Charset::judge_charset_safe(charset) {
            return Err(DecodeError::CharsetError);
        }
        let mut buffer: Vec<u8> = Vec::new();
        let mut decoded: Vec<u8> = Vec::new();
        for ch in decode_str.chars() {
            if let Some(idx) = charset.chars().position(|character: char| character == ch) {
                buffer.push(idx as u8);
            }
            if buffer.len() == 4 {
                let combined: usize = ((buffer[0] as usize) << 18)
                    | ((buffer[1] as usize) << 12)
                    | ((buffer[2] as usize) << 6)
                    | (buffer[3] as usize);
                decoded.push((combined >> 16) as u8);
                decoded.push((combined >> 8) as u8);
                decoded.push(combined as u8);
                buffer.clear();
            }
        }
        let decode_res: String =
            String::from_utf8(decoded.into_iter().filter(|&x| x != 0).collect())
                .unwrap_or_default();
        Ok(decode_res)
    }
}
