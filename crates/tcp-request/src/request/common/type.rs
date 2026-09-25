use super::*;

/// Result type for request operations containing either a boxed response trait or request error.
pub type RequestResult = Result<BoxResponseTrait, RequestError>;

/// Boxed trait object for request operations.
pub type BoxRequestTrait = Box<dyn RequestTrait<RequestResult = RequestResult>>;
