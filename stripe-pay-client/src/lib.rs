//! stripe-pay-client
//!
//! Browser side of the Stripe payment SDK: the payment element
//! configuration the checkout page mounts, built over the shared
//! `stripe-pay-core` request and response types.

mod r#element;
mod r#host;
mod r#mount;

pub use {r#element::*, r#host::*, r#mount::*};

pub use lombok_macros::*;

pub use std::fmt::{self, Display, Formatter};
pub use stripe_pay_core::*;
