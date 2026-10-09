//! crate-cli
//!
//! A command-line tool for managing Cargo package lifecycles:
//! version bump, workspace dependency sync,
//! members-ordered publish and code formatting.

mod bump;
mod cli;
mod command;
mod config;
mod fmt;
mod help;
mod logger;
mod manifest;
mod message;
mod publish;
mod sync;
mod tool;
mod version;

pub use {
    bump::*, command::*, config::*, fmt::*, help::*, logger::*, manifest::*, publish::*, sync::*,
    version::*,
};

pub(crate) use {cli::*, message::*, tool::*};

pub(crate) use std::{
    collections::HashMap,
    env::args,
    io,
    path::{Path, PathBuf},
    process::Stdio,
    sync::{Arc, LazyLock},
};

pub(crate) use {
    color_output::*,
    log::{self, SetLoggerError},
    lombok_macros::*,
    regex::{Captures, Regex},
    std::ffi::OsStr,
    tokio::{
        fs::{ReadDir, read_dir, read_to_string, write},
        process::Command,
        spawn,
        sync::{Mutex, MutexGuard},
        task::JoinHandle,
        time::{Duration, sleep},
    },
    toml::Value,
    toml_edit::{DocumentMut, Item, TableLike, TomlError, value},
    which::which,
};

use std::{
    error::Error, fmt::Arguments, fs, process::Output, str::Lines, str::Split,
    str::SplitWhitespace, time::SystemTime, time::UNIX_EPOCH,
};
