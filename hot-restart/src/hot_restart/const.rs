/// Cargo executable name used to probe and install cargo-watch.
pub(crate) const CARGO_PROGRAM: &str = "cargo";

/// Cargo subcommand used to list installed binaries.
pub(crate) const CARGO_LIST_ARG: &str = "--list";

/// Cargo subcommand used to install cargo-watch.
pub(crate) const CARGO_INSTALL_ARG: &str = "install";

/// Watcher executable that performs the hot restart.
pub(crate) const CARGO_WATCH_PROGRAM: &str = "cargo-watch";
