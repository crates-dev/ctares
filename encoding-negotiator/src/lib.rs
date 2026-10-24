//! encoding-negotiator
//!
//! A library that negotiates the content encoding of an HTTP answer. It picks
//! the encoding a client asked for out of the encodings that are offered, and
//! tells whether a body of a content type is worth compressing.

mod r#const;
mod r#impl;
mod r#struct;

pub use r#struct::*;

pub use lombok_macros::*;

pub(crate) use r#const::*;
