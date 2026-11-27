use super::*;

/// Package information in monorepo
#[derive(Clone, Debug, Eq, Getter, New, PartialEq)]
pub struct Package {
    /// Package name
    pub(crate) name: String,
    /// Package version
    pub(crate) version: String,
    /// Path to package directory
    pub(crate) path: PathBuf,
    /// Dependencies within the monorepo
    pub(crate) local_dependencies: Vec<String>,
    /// Whether the package should be published (false skips publishing)
    #[get(type(copy))]
    pub(crate) publish: bool,
}

/// Publish result for a single package
#[derive(Clone, Debug, Getter, New)]
pub struct PublishResult {
    /// Package name
    pub(crate) package_name: String,
    /// Whether publish succeeded
    #[get(type(copy))]
    pub(crate) success: bool,
    /// Error message if failed
    pub(crate) error: Option<String>,
    /// Number of retries performed
    #[get(type(copy))]
    pub(crate) retries: u32,
}
