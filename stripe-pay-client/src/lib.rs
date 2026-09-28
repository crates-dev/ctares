//! stripe-pay-client
//!
//! Browser side of the Stripe payment SDK: the payment element
//! configuration the checkout page mounts, built over the shared
//! `stripe-pay-core` request and response types.

mod r#const;
mod r#element;
mod r#enum;
mod r#fn;

pub use {r#const::*, r#element::*, r#enum::*, r#fn::*};

pub use lombok_macros::*;

pub use std::fmt::{self, Display, Formatter};
pub use stripe_pay_core::*;
