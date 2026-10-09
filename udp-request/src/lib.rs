//! udp-request
//!
//! A simple UDP request library for sending and receiving UDP packets,
//! designed to handle network communication in Rust applications.

mod common;
mod request;
mod response;

pub use {request::*, response::*};

use common::*;

use lombok_macros::*;

use std::{
    error::Error,
    fmt::Debug,
    fmt::{self, Display},
    io,
    net::UdpSocket,
    sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard},
    time::Duration,
};
