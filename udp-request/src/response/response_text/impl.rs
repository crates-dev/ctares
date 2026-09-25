use super::*;

/// Implements the `ResponseTrait` for `UdpResponseText`.
impl ResponseTrait for UdpResponseText {
    type OutputText = UdpResponseText;
    type OutputBinary = UdpResponseBinary;

    /// Creates a `UdpResponseText` from a byte slice.
    ///
    /// This method first converts the byte slice to `UdpResponseBinary`
    /// and then to `UdpResponseText`.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The byte slice containing the response data.
    ///
    /// # Returns
    ///
    /// - `Self::OutputText` - The text representation of the response.
    #[inline(always)]
    fn from(response: &[u8]) -> Self::OutputText
    where
        Self: Sized,
    {
        <UdpResponseBinary as ResponseTrait>::from(response).text()
    }

    /// Returns the text representation of the response.
    ///
    /// # Returns
    ///
    /// - `Self::OutputText` - The text data of the response.
    #[inline(always)]
    fn text(&self) -> Self::OutputText {
        self.clone()
    }

    /// Converts the text response to its binary representation.
    ///
    /// # Returns
    ///
    /// - `UdpResponseBinary` - The binary representation of the response.
    #[inline(always)]
    fn binary(&self) -> UdpResponseBinary {
        self.clone().into_bytes()
    }
}
