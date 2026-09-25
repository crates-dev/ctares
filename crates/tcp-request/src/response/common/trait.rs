use super::*;

/// Trait defining the interface for response operations.
pub trait ResponseTrait: Send + Debug {
    type OutputText: Clone + Sized;
    type OutputBinary: Clone + Sized;

    /// Gets the response as text.
    ///
    /// # Returns
    ///
    /// - `Self::OutputText` - The response text content.
    fn text(&self) -> Self::OutputText;

    /// Gets the response as binary data.
    ///
    /// # Returns
    ///
    /// - `Self::OutputBinary` - The response binary content.
    fn binary(&self) -> Self::OutputBinary;

    /// Creates a response from raw bytes.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The raw response data.
    ///
    /// # Returns
    ///
    /// - `Self` - A new response instance.
    fn from(response: &[u8]) -> Self
    where
        Self: Sized;
}
