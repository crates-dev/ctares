//! static-file-resolver
//!
//! A library that resolves the file a request asks for under a root
//! directory. It refuses a path that climbs out of the root, so a request
//! cannot reach a file the root does not hold.

mod r#const;
mod r#impl;
mod r#struct;

pub use r#struct::*;

pub use lombok_macros::*;

pub(crate) use r#const::*;

use std::path::{Path, PathBuf};
