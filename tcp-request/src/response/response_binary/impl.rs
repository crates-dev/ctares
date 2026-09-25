use super::*;

/// ResponseTrait implementation for binary TCP responses.
impl ResponseTrait for TcpResponseBinary {
    type OutputText = TcpResponseText;
    type OutputBinary = TcpResponseBinary;

    /// Creates a binary response from raw bytes.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The raw response data.
    ///
    /// # Returns
    ///
    /// - `Self` - A new binary response instance.
    #[inline(always)]
    fn from(response: &[u8]) -> Self
    where
        Self: Sized,
    {
        response.to_vec()
    }

    /// Gets the binary representation of the response.
    ///
    /// # Returns
    ///
    /// - `Self::OutputBinary` - The binary response data.
    #[inline(always)]
    fn binary(&self) -> Self::OutputBinary {
        self.clone()
    }

    /// Converts the binary response to text representation.
    ///
    /// # Returns
    ///
    /// - `TcpResponseText` - The text representation of the response.
    #[inline(always)]
    fn text(&self) -> TcpResponseText {
        let data: String = String::from_utf8_lossy(self).to_string();
        data
    }
}
