//! A high-performance async library for JWT (JSON Web Token) authentication and authorization.
//! Supports token generation, validation, and custom claims with optimized memory usage,
//! ideal for HTTP clients/servers and web applications.

mod r#const;
mod r#enum;
mod r#impl;
mod r#struct;

pub use {r#enum::*, r#struct::*};

use r#const::*;

use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use {
    jsonwebtoken::{
        Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
    },
    serde::{Deserialize, Serialize},
    serde_json::Value,
};
