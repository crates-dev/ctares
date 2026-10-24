//! chunkify
//!
//! A simple and efficient chunking library for Rust.

mod r#const;
mod r#enum;
mod r#impl;
mod r#static;
mod r#struct;
mod r#trait;
mod r#type;

pub use {r#const::*, r#enum::*, r#struct::*, r#trait::*, r#type::*};

use r#static::*;

use std::{
    error, fmt,
    fs::{File, OpenOptions, create_dir_all, remove_file},
    hash::BuildHasherDefault,
    io::{BufWriter, Error, Write},
    path::Path,
    sync::Arc,
};

use {
    dashmap::{DashMap, mapref::one::RefMut},
    lombok_macros::*,
    once_cell::sync::Lazy,
    tokio::{
        fs,
        io::{AsyncReadExt, AsyncWriteExt},
        sync::{RwLock, RwLockWriteGuard},
    },
    twox_hash::XxHash3_64,
};
