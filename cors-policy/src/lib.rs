//! cors-policy
//!
//! A library that decides the cross origin policy of an HTTP answer. It tells
//! whether an origin is allowed and builds the cross origin headers of the
//! answer.

mod r#const;
mod r#impl;
mod r#struct;

pub use r#struct::*;

pub use lombok_macros::*;

pub(crate) use r#const::*;
