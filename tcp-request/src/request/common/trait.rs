use super::*;

/// Trait defining the interface for request operations.
pub trait RequestTrait: Send + Debug {
    type RequestResult: Sized;

    /// Sends data through the request.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The data to be sent.
    ///
    /// # Returns
    ///
    /// - `Self::RequestResult` - The result of the send operation.
    fn send(&mut self, data: &[u8]) -> Self::RequestResult;
}
