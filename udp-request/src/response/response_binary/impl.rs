use super::*;

/// Implements the `ResponseTrait` for `UdpResponseBinary`.
impl ResponseTrait for UdpResponseBinary {
    type OutputText = UdpResponseText;
    type OutputBinary = UdpResponseBinary;

    /// Creates a `UdpResponseBinary` instance from a byte slice.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The byte slice containing the response data.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `UdpResponseBinary` instance.
    #[inline(always)]
    fn from(response: &[u8]) -> Self
    where
        Self: Sized,
    {
        response.to_vec()
    }

    /// Returns the binary representation of the response.
    ///
    /// # Returns
    ///
    /// - `Self::OutputBinary` - The binary data of the response.
    #[inline(always)]
    fn binary(&self) -> Self::OutputBinary {
        self.clone()
    }

    /// Converts the binary response to a text representation.
    ///
    /// # Returns
    ///
    /// - `UdpResponseText` - The text representation of the response, with invalid UTF-8 sequences replaced.
    #[inline(always)]
    fn text(&self) -> UdpResponseText {
        let data: String = String::from_utf8_lossy(self).to_string();
        data
    }
}
