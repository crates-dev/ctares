use super::*;

/// A thread pool that can execute tasks concurrently.
///
/// Manages a collection of worker threads and provides methods
/// to submit tasks for execution.
///
/// # Returns
///
/// - `ThreadPool` - A new thread pool instance.
#[derive(Debug)]
pub struct ThreadPool {
    /// The collection of worker threads.
    ///
    /// # Returns
    ///
    /// - `Vec<Worker>` - The collection of worker threads.
    #[allow(dead_code)]
    pub(crate) workers: Vec<Worker>,
    /// The sender channel for submitting jobs to workers.
    ///
    /// # Returns
    ///
    /// - `Sender<ThreadPoolJob>` - The sender channel for submitting jobs to workers.
    pub(crate) sender: Sender<ThreadPoolJob>,
}
