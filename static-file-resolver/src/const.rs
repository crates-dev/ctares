/// The file that answers a request for a directory.
pub(crate) const INDEX_DEFAULT: &str = "index.html";

/// The separator that splits the segments of a path.
pub(crate) const SEPARATOR_SLASH: &str = "/";

/// The separator that must not appear in a segment of a path.
pub(crate) const SEPARATOR_BACKSLASH: char = '\\';

/// The segment that names the directory a path is in.
pub(crate) const SEGMENT_CURRENT: &str = ".";

/// The segment that names the directory a path comes from.
pub(crate) const SEGMENT_PARENT: &str = "..";
