//! bin-encode-decode
//!
//! A high-performance binary encode and decode library
//! that supports customizable character sets beyond Base64.

mod common;
mod decode;
mod encode;

pub use {common::*, decode::*, encode::*};

use std::{collections::HashSet, fmt};
