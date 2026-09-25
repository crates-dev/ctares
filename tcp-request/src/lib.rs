//! tcp-request
//!
//! A Rust library for sending raw TCP requests, with features
//! for handling responses, managing redirects, and working
//! with compressed data over TCP connections.

mod common;
mod request;
mod response;

pub use {request::*, response::*};

use common::*;

use std::{
    fmt::Debug,
    fmt::{self, Display},
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, RwLock, RwLockReadGuard},
    time::Duration,
};
