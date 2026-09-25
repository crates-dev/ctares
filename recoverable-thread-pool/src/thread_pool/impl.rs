use super::*;

/// Sync implementation of thread pool operations.
impl ThreadPool {
    /// Creates a new thread pool with the specified number of workers.
    ///
    /// # Arguments
    ///
    /// - `usize` - The number of worker threads to create.
    ///
    /// # Returns
    ///
    /// - `ThreadPool` - The new thread pool instance.
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver: Arc<Mutex<Receiver<ThreadPoolJob>>> = Arc::new(Mutex::new(receiver));
        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        let mut id: usize = 0;
        loop {
            if id >= size {
                break;
            }
            let worker: Option<Worker> = Worker::new(id, Arc::clone(&receiver));
            if worker.is_some() {
                workers.push(worker.unwrap_or_default());
                id += 1;
            }
        }
        ThreadPool { workers, sender }
    }

    /// Executes a synchronous job in the thread pool.
    ///
    /// # Arguments
    ///
    /// - `F` - The synchronous function to execute.
    ///
    /// # Returns
    ///
    /// - `SendResult` - Result of the job submission.
    pub fn execute<F>(&self, job: F) -> SendResult
    where
        F: RecoverableFunction,
    {
        let job_with_handler: ThreadPoolJob = Box::new(move || {
            let _: std::thread::Result<()> = run_function(job);
        });
        self.sender.send(job_with_handler)
    }

    /// Executes a synchronous job with error handling in the thread pool.
    ///
    /// # Arguments
    ///
    /// - `F` - The synchronous function to execute.
    /// - `E` - The error handler function.
    ///
    /// # Returns
    ///
    /// - `SendResult` - Result of the job submission.
    pub fn execute_with_catch<F, E>(&self, job: F, handle_error: E) -> SendResult
    where
        F: RecoverableFunction,
        E: ErrorHandlerFunction,
    {
        let job_with_handler: ThreadPoolJob = Box::new(move || {
            if let Err(err) = run_function(job) {
                let err_string: String = spawn_error_to_string(&err);
                let _: std::thread::Result<()> =
                    run_error_handle_function(handle_error, &err_string);
            }
        });
        self.sender.send(job_with_handler)
    }

    /// Executes a synchronous job with error handling and finalization in the thread pool.
    ///
    /// # Arguments
    ///
    /// - `F` - The synchronous function to execute.
    /// - `E` - The error handler function.
    /// - `L` - The finally handler function.
    ///
    /// # Returns
    ///
    /// - `SendResult` - Result of the job submission.
    pub fn execute_with_catch_finally<F, E, L>(
        &self,
        job: F,
        handle_error: E,
        finally: L,
    ) -> SendResult
    where
        F: RecoverableFunction,
        E: ErrorHandlerFunction,
        L: RecoverableFunction,
    {
        let job_with_handler: ThreadPoolJob = Box::new(move || {
            if let Err(err) = run_function(job) {
                let err_string: String = spawn_error_to_string(&err);
                let _: std::thread::Result<()> =
                    run_error_handle_function(handle_error, &err_string);
            }
            let _: std::thread::Result<()> = run_function(finally);
        });
        self.sender.send(job_with_handler)
    }

    /// Executes an async job in the thread pool.
    ///
    /// # Arguments
    ///
    /// - `F` - The async function to execute.
    ///
    /// # Returns
    ///
    /// - `SendResult` - Result of the job submission.
    pub fn async_execute<F>(&self, job: F) -> SendResult
    where
        F: AsyncRecoverableFunction,
    {
        let job_with_handler = Box::new(move || {
            Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async move {
                    let _: AsyncSpawnResult = async_run_function(move || async {
                        job.call().await;
                    })
                    .await;
                });
        });
        self.sender.send(job_with_handler)
    }

    /// Executes an async job with error handling in the thread pool.
    ///
    /// # Arguments
    ///
    /// - `F` - The async function to execute.
    /// - `E` - The async error handler function.
    ///
    /// # Returns
    ///
    /// - `SendResult` - Result of the job submission.
    pub fn async_execute_with_catch<F, E>(&self, job: F, handle_error: E) -> SendResult
    where
        F: AsyncRecoverableFunction,
        E: AsyncErrorHandlerFunction,
    {
        let job_with_handler = Box::new(move || {
            Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async move {
                    let run_result: AsyncSpawnResult = async_run_function(move || async {
                        job.call().await;
                    })
                    .await;
                    if let Err(err) = run_result {
                        let err_string: String = tokio_error_to_string(&err);
                        let _: AsyncSpawnResult = async_run_error_handle_function(
                            move |err_str| async move {
                                handle_error.call(err_str).await;
                            },
                            Arc::new(err_string),
                        )
                        .await;
                    }
                });
        });
        self.sender.send(job_with_handler)
    }

    /// Executes an async job with error handling and finalization in the thread pool.
    ///
    /// # Arguments
    ///
    /// - `F` - The async function to execute.
    /// - `E` - The async error handler function.
    /// - `L` - The async finally handler function.
    ///
    /// # Returns
    ///
    /// - `SendResult` - Result of the job submission.
    pub fn async_execute_with_catch_finally<F, E, L>(
        &self,
        job: F,
        handle_error: E,
        finally: L,
    ) -> SendResult
    where
        F: AsyncRecoverableFunction,
        E: AsyncErrorHandlerFunction,
        L: AsyncRecoverableFunction,
    {
        let job_with_handler = Box::new(move || {
            Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async move {
                    let run_result: AsyncSpawnResult = async_run_function(move || async {
                        job.call().await;
                    })
                    .await;
                    if let Err(err) = run_result {
                        let err_string: String = tokio_error_to_string(&err);
                        let _: AsyncSpawnResult = async_run_error_handle_function(
                            move |err_str| async move {
                                handle_error.call(err_str).await;
                            },
                            Arc::new(err_string),
                        )
                        .await;
                    }
                    let _: AsyncSpawnResult = async_run_function(move || async {
                        finally.call().await;
                    })
                    .await;
                });
        });
        self.sender.send(job_with_handler)
    }
}
