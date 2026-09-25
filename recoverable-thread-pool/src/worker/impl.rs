use super::*;

/// Worker implementation for handling thread pool jobs.
impl Worker {
    /// Creates a new worker thread.
    ///
    /// # Arguments
    ///
    /// - `usize` - The worker identifier.
    /// - `Arc<Mutex<Receiver<ThreadPoolJob>>>` - The shared job receiver.
    ///
    /// # Returns
    ///
    /// - `Option<Worker>` - The new worker instance.
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<ThreadPoolJob>>>) -> Option<Worker> {
        spawn(|| {
            let _: std::thread::Result<()> = recoverable_spawn(move || {
                loop {
                    if let Ok(receiver_lock) = receiver.lock()
                        && let Ok(job) = receiver_lock.recv()
                    {
                        let _: std::thread::Result<()> = recoverable_spawn(job);
                    }
                }
            });
        });
        Some(Worker { id })
    }
}
