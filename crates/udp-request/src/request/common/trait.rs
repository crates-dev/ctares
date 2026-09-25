use super::*;

/// A trait for sending UDP requests.
///
/// This trait defines the core functionality for sending data over UDP.
/// It requires the `Send` and `Debug` traits.
pub trait RequestTrait: Send + Debug {
    /// The result type for the request operation.
    ///
    /// This associated type must be `Sized`.
    type RequestResult: Sized;

    /// Sends data through the UDP socket.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The data to be sent as a byte slice.
    ///
    /// # Returns
    ///
    /// - `Self::RequestResult` - The result of the send operation, as defined by the implementor.
    fn send(&mut self, data: &[u8]) -> Self::RequestResult;
}
