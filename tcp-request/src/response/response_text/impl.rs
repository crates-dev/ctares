use super::*;

/// ResponseTrait implementation for text TCP responses.
impl ResponseTrait for TcpResponseText {
    type OutputText = TcpResponseText;
    type OutputBinary = TcpResponseBinary;

    /// Creates a text response from raw bytes.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The raw response data.
    ///
    /// # Returns
    ///
    /// - `Self::OutputText` - The text response instance.
    #[inline(always)]
    fn from(response: &[u8]) -> Self::OutputText
    where
        Self: Sized,
    {
        <TcpResponseBinary as ResponseTrait>::from(response).text()
    }

    /// Gets the text representation of the response.
    ///
    /// # Returns
    ///
    /// - `Self::OutputText` - The text response data.
    #[inline(always)]
    fn text(&self) -> Self::OutputText {
        self.clone()
    }

    /// Converts the text response to binary representation.
    ///
    /// # Returns
    ///
    /// - `TcpResponseBinary` - The binary representation of the response.
    #[inline(always)]
    fn binary(&self) -> TcpResponseBinary {
        self.clone().into_bytes()
    }
}
