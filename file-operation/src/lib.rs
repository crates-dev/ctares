//! file-operation
//!
//! A Rust library providing comprehensive utilities for file operations with both sync/async support.
//! Includes operations for copy, delete, move, read and write files. Simplifies file handling
//! in Rust projects with safe and efficient methods for file manipulation and metadata querying.

mod copy;
mod delete;
mod file;
mod r#move;
mod read;
mod write;

pub use {copy::*, delete::*, file::*, r#move::*, read::*, write::*};

use std::{
    error,
    ffi::OsString,
    fmt,
    fs::DirEntry,
    fs::File,
    fs::Metadata,
    fs::OpenOptions,
    fs::copy,
    fs::create_dir_all,
    fs::metadata,
    fs::read_dir,
    fs::remove_dir,
    fs::remove_dir_all,
    fs::remove_file,
    fs::rename,
    io::Error,
    io::Read,
    io::Write,
    path::{Path, PathBuf},
    pin::Pin,
    string::FromUtf8Error,
};

use tokio::{fs::ReadDir, spawn, task::JoinHandle};
