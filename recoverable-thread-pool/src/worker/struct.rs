use super::*;

/// A worker thread in the thread pool.
///
/// Each worker is responsible for executing jobs
/// from the shared job queue.
#[derive(Debug, Data, Default)]
pub struct Worker {
    /// The unique identifier for this worker.
    #[get(type(copy))]
    pub(super) id: usize,
}
