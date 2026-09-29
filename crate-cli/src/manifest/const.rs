/// Manifest file name every Cargo package root is looked up under.
pub const CARGO_TOML: &str = "Cargo.toml";

/// `[workspace]` table key in a Cargo manifest.
pub(crate) const TOML_WORKSPACE: &str = "workspace";

/// `[package]` table key in a Cargo manifest.
pub(crate) const TOML_PACKAGE: &str = "package";

/// `[package].version` key in a Cargo manifest.
pub(crate) const TOML_VERSION: &str = "version";

/// `[package].name` key in a Cargo manifest.
pub(crate) const TOML_NAME: &str = "name";

/// `[workspace.members]` array key in a Cargo manifest.
pub(crate) const TOML_MEMBERS: &str = "members";

/// `[package].publish` key in a Cargo manifest.
pub(crate) const TOML_PUBLISH_KEY: &str = "publish";

/// `[dependencies]` table key in a Cargo manifest.
pub(crate) const TOML_DEPENDENCIES: &str = "dependencies";

/// `[dev-dependencies]` table key in a Cargo manifest.
pub(crate) const TOML_DEV_DEPENDENCIES: &str = "dev-dependencies";

/// `[build-dependencies]` table key in a Cargo manifest.
pub(crate) const TOML_BUILD_DEPENDENCIES: &str = "build-dependencies";

/// `[target]` table key in a Cargo manifest.
pub(crate) const TOML_TARGET: &str = "target";

/// `path` key of a local dependency entry in a Cargo manifest.
pub(crate) const TOML_PATH: &str = "path";
