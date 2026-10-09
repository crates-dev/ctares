mod request_builder;

use std::{
    sync::{Arc, Mutex},
    thread::{JoinHandle, spawn},
    time::{Duration, Instant},
};
use udp_request::*;
