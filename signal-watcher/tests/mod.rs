mod watcher;

use signal_watcher::*;

use std::{
    process::{Command, ExitStatus, id},
    time::Duration,
};

use tokio::{
    signal::unix::{Signal, SignalKind},
    sync::oneshot,
    task::JoinHandle,
};
