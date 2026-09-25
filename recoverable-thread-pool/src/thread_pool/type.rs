use super::*;

/// A job that can be executed by the thread pool.
pub type ThreadPoolJob = Box<dyn RecoverableFunction>;

/// Error type for failed job submissions.
pub type SendErrorBox = SendError<ThreadPoolJob>;

/// Result type for job submission operations.
pub type SendResult = Result<(), SendErrorBox>;
