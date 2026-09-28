//! stripe-pay-core
//!
//! Shared Stripe domain types for the stripe-pay-sdk workspace: money
//! handling, typed Stripe identifiers, payment intents, charges,
//! refunds, structured errors, and Stripe's bracketed form encoding.

mod r#const;

mod charge;
mod error;
mod form;
mod id;
mod money;
mod payment_intent;
mod payment_method;
mod refund;

pub use r#const::*;

pub use {
    charge::*, error::*, form::*, id::*, money::*, payment_intent::*, payment_method::*, refund::*,
};

pub use rust_decimal::{
    Decimal,
    prelude::{FromPrimitive, ToPrimitive},
};
pub use serde::{Deserialize, Serialize};
pub use serde_json::{Value, json};

use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};
