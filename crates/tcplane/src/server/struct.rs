use super::*;

/// Represents the internal, mutable state of the TCP server.
///
/// This struct consolidates all the core components required for the server to operate,
/// including configuration and handler list. It is not intended to be used directly by end-users,
/// but rather wrapped within the `Server` struct for thread-safe access.
#[derive(Clone)]
pub struct ServerData {
    /// Stores the server's configuration settings, such as address, port, and buffer size.
    pub(crate) server_config: ServerConfigData,
    /// A collection of request hooks that are invoked for each incoming connection.
    pub(crate) hook: ServerHookList,
    /// A collection of task panic handlers that are invoked when a panic occurs during connection processing.
    pub(crate) task_panic: ServerHookList,
    /// The error handlers for server operations.
    pub(crate) read_error: ServerHookList,
}

/// The primary server structure that provides a thread-safe interface to the server's state.
///
/// This struct acts as a public-facing wrapper around an `Arc<RwLock<ServerData>>`.
/// It allows multiple parts of the application to safely share and modify the server's
/// configuration and state across different threads and asynchronous tasks.
#[derive(Clone)]
pub struct Server(pub(super) ArcRwLock<ServerData>);

/// Represents the hooks for managing the server's lifecycle, specifically for waiting and shutting down.
#[derive(Clone)]
pub struct ServerControlHook {
    /// A hook that returns a future, which completes when the server's main task finishes.
    pub(crate) wait_hook:
        Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync>,
    /// A hook that, when called, initiates a graceful shutdown of the server.
    pub(crate) shutdown_hook:
        Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync>,
}
