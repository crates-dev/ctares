//! compare_version
//!
//! A Rust library for comparing semantic versioning
//! strings and checking version compatibility.

mod r#const;
mod r#enum;
mod r#impl;
mod r#struct;

pub use {r#enum::*, r#struct::*};

use r#const::*;

use std::{cmp::Ordering, fmt, num::ParseIntError, str::SplitN};
