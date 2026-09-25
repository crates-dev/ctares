use super::*;

/// Represents the internal state of the connection context.
///
/// This structure holds all the data associated with a single connection,
/// including the stream, request, response, and any custom data.
#[derive(Clone)]
pub(crate) struct ContextData {
    /// A flag indicating whether the connection handling has been aborted.
    pub(crate) aborted: bool,
    /// A flag indicating whether the connection has been closed.
    pub(crate) closed: bool,
    /// The underlying network stream for the connection.
    pub(crate) stream: Option<ArcRwLockStream>,
    /// The incoming request data.
    pub(crate) request: Request,
    /// The outgoing response.
    pub(crate) response: Response,
    /// Attributes storage for holding arbitrary type data during connection processing.
    pub(crate) attributes: HashMapArcAnySendSync,
}

/// The main connection context, providing thread-safe access to connection data.
///
/// This is a wrapper around `ContextData` that uses an `Arc<RwLock<ContextData>>` to allow
/// for shared, mutable access across asynchronous tasks.
#[derive(Clone)]
pub struct Context(pub(super) ArcRwLock<ContextData>);
