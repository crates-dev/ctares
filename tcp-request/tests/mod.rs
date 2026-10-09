mod request_builder;

use std::{
    sync::{Arc, Mutex},
    thread::{JoinHandle, spawn},
    time::{Duration, Instant},
};
use tcp_request::*;
