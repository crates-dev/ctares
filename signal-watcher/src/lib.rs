//! signal-watcher
//!
//! A library that waits for the signal that asks a process to stop. It turns
//! the interrupt and the terminate signal of the operating system into a call
//! of a handler, so a server can shut down in an orderly way.

mod r#const;
mod r#enum;
mod r#fn;
mod r#impl;

pub use {r#enum::*, r#fn::*};

pub(crate) use r#const::*;

use std::{fmt, future::Future, io::Error};
