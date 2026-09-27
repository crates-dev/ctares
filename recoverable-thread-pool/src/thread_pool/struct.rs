use super::*;

/// A thread pool that can execute tasks concurrently.
///
/// Manages a collection of worker threads and provides methods
/// to submit tasks for execution.
///
/// # Returns
///
/// - `ThreadPool` - A new thread pool instance.
#[derive(Data, Debug)]
pub struct ThreadPool {
    /// The collection of worker threads.
    ///
    /// # Returns
    ///
    /// - `Vec<Worker>` - The collection of worker threads.
    pub(crate) workers: Vec<Worker>,
    /// The sender channel for submitting jobs to workers.
    ///
    /// # Returns
    ///
    /// - `Sender<ThreadPoolJob>` - The sender channel for submitting jobs to workers.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) sender: Sender<ThreadPoolJob>,
}
