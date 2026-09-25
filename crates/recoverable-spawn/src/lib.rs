//! recoverable-spawn
//!
//! A thread that supports automatic recovery from panics,
//! allowing threads to restart after a panic. Useful for resilient
//! and fault-tolerant concurrency in network and web programming.

mod r#async;
mod common;
mod sync;

pub use {r#async::*, common::*, sync::*};

use std::{any::Any, panic::set_hook, sync::Arc};

use tokio::task::JoinError;
