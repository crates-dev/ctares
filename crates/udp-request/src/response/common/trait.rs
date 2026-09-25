use super::*;

/// A trait for handling UDP responses.
///
/// This trait defines the common operations for processing UDP responses,
/// including converting them to text or binary formats.
pub trait ResponseTrait: Send + Debug {
    /// The associated type for the text representation of the response.
    type OutputText: Clone + Sized;
    /// The associated type for the binary representation of the response.
    type OutputBinary: Clone + Sized;

    /// Converts the response to its text format.
    ///
    /// # Returns
    ///
    /// - `Self::OutputText` - The text representation of the response.
    fn text(&self) -> Self::OutputText;

    /// Returns the binary representation of the response.
    ///
    /// # Returns
    ///
    /// - `Self::OutputBinary` - The binary representation of the response.
    fn binary(&self) -> Self::OutputBinary;

    /// Creates a response instance from a byte slice.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The byte slice containing the response data.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance of the response type.
    fn from(response: &[u8]) -> Self
    where
        Self: Sized;
}
