//! udp-request
//!
//! A simple UDP request library for sending and receiving UDP packets,
//! designed to handle network communication in Rust applications.

mod common;
mod request;
mod response;

pub use {request::*, response::*};

use common::*;

use std::{
    fmt::Debug,
    fmt::{self, Display},
    net::UdpSocket,
    sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard},
    time::Duration,
};
