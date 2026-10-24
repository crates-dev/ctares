/// The smallest body that is compressed by default.
pub(crate) const DEFAULT_MIN_SIZE: usize = 1024;

/// The name of the gzip content encoding.
pub(crate) const ENCODING_GZIP: &str = "gzip";

/// The name of the brotli content encoding.
pub(crate) const ENCODING_BROTLI: &str = "br";

/// The content type of a stream of events, which is never compressed.
pub(crate) const TEXT_EVENT_STREAM: &str = "text/event-stream";

/// The separator that splits the values of a header.
pub(crate) const SEPARATOR_COMMA: &str = ",";

/// The token that stands for every encoding.
pub(crate) const TOKEN_ANY: &str = "*";

/// The prefix of the quality parameter of an encoding.
pub(crate) const PARAMETER_QUALITY: &str = "q=";

/// The content types that carry a body that is already compressed.
pub(crate) const CONTENT_TYPES_COMPRESSED: [&str; 6] = [
    "application/zip",
    "application/gzip",
    "application/x-gzip",
    "application/x-bzip2",
    "application/x-xz",
    "application/zstd",
];

/// The content type prefixes that carry a body that is already compressed.
pub(crate) const CONTENT_TYPE_PREFIXES_COMPRESSED: [&str; 4] =
    ["image/", "video/", "audio/", "font/"];
