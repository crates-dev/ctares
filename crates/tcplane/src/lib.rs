//! tcplane
//!
//! tcplane is a lightweight and high-performance Rust TCP server
//! library designed to simplify network service development.
//! It supports TCP communication, data stream management,
//! and connection handling, focusing on providing efficient
//! low-level network connections and data transmission capabilities,
//! making it ideal for building modern network services.

mod common;
mod config;
mod context;
mod error;
mod handler;
mod request;
mod response;
mod server;
mod stream;
mod utils;

pub use {
    common::*, config::*, context::*, error::*, handler::*, request::*, response::*, server::*,
    stream::*, utils::*,
};

pub use tokio;

use std::{
    any::Any,
    collections::HashMap,
    fmt::{self, Display},
    future::Future,
    net::SocketAddr,
    pin::Pin,
    sync::Arc,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    spawn,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, watch::Sender, watch::channel},
    task::{JoinError, JoinHandle},
};
