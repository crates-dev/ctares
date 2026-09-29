/// Cargo executable spawned as a child process.
pub(crate) const CARGO: &str = "cargo";

/// Cargo subcommand that reformats the workspace.
pub(crate) const CARGO_FMT: &str = "fmt";

/// Cargo subcommand that uploads a package to the registry.
pub(crate) const CARGO_PUBLISH: &str = "publish";

/// `clippy` subcommand / component name.
pub(crate) const CLIPPY: &str = "clippy";

/// Standalone `cargo-clippy` executable probed on `PATH`.
pub(crate) const CARGO_CLIPPY: &str = "cargo-clippy";

/// Rust toolchain manager executable spawned as a child process.
pub(crate) const RUSTUP: &str = "rustup";

/// `component` subcommand of the Rust toolchain manager.
pub(crate) const RUSTUP_COMPONENT: &str = "component";
