//! recoverable-thread-pool
//!
//! A thread pool that supports automatic recovery from panics,
//! allowing threads to restart after a panic. Useful for resilient
//! and fault-tolerant concurrency in network and web programming.

mod thread_pool;
mod worker;

pub use {thread_pool::*, worker::*};

use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, SendError, Sender},
    },
    thread::spawn,
};

use {recoverable_spawn::*, tokio::runtime::Builder};
