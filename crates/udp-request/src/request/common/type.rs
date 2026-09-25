use super::*;

/// A type alias for the result of a UDP request.
pub type RequestResult = Result<BoxResponseTrait, RequestError>;

/// A type alias for a boxed `RequestTrait` object.
pub type BoxRequestTrait = Box<dyn RequestTrait<RequestResult = RequestResult>>;
