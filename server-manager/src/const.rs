#[cfg(not(windows))]
/// Executable name of the Unix process termination utility.
pub(crate) const KILL: &str = "kill";

/// Executable name of the Cargo package manager.
pub(crate) const CARGO: &str = "cargo";

/// Cargo subcommand that installs a crate from crates.io.
pub(crate) const INSTALL: &str = "install";

/// Executable name of the cargo-watch hot-reload tool.
pub(crate) const CARGO_WATCH: &str = "cargo-watch";

#[cfg(not(windows))]
/// Signal passed to the Unix `kill` utility for a graceful termination.
pub(crate) const KILL_SIGNAL: &str = "-TERM";

/// Environment variable name for daemon mode detection.
pub const RUNNING_AS_DAEMON: &str = "RUNNING_AS_DAEMON";

/// Arguments installing the named crate with cargo.
pub(crate) const CARGO_INSTALL_ARGS: [&str; 2] = ["install", "cargo-watch"];

/// Value indicating the process is running in daemon mode.
pub const RUNNING_AS_DAEMON_VALUE: &str = "1";

/// Message printed when cargo-watch is missing and an installation is attempted.
pub(crate) const CARGO_WATCH_ABSENT_MESSAGE: &str =
    "Cargo-watch not found. Attempting to install...";

/// Message printed after a successful automatic cargo-watch installation.
pub(crate) const CARGO_WATCH_INSTALLED_MESSAGE: &str = "Cargo-watch installed successfully.";

/// Arguments listing every globally installed cargo package.
pub(crate) const CARGO_WATCH_INSTALL_LIST_ARGS: [&str; 1] = ["--list"];

/// Error message returned when the automatic cargo-watch installation fails.
pub(crate) const CARGO_WATCH_INSTALL_FAILED_MESSAGE: &str =
    "Failed to install cargo-watch. Please install it manually: `cargo install cargo-watch`";
