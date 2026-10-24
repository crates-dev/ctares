use super::*;

/// The rules that decide which file a request is answered with.
///
/// The resolver holds no server state, so the same rules can be used by any
/// server that serves the files of a directory.
#[derive(Clone, Copy, Debug, Getter)]
pub struct StaticFileResolver<'a> {
    /// The directory the files are served from.
    #[get(type(copy))]
    pub root: &'a Path,
    /// The file that answers a request for a directory.
    #[get(type(copy))]
    pub index: &'a str,
}
