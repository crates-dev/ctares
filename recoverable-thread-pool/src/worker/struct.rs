/// A worker thread in the thread pool.
///
/// Each worker is responsible for executing jobs
/// from the shared job queue.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Worker {
    /// The unique identifier for this worker.
    pub(super) id: usize,
}
