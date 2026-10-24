/// The header that names the origin that is allowed.
pub(crate) const HEADER_ALLOW_ORIGIN: &str = "access-control-allow-origin";

/// The header that names the methods that are allowed.
pub(crate) const HEADER_ALLOW_METHODS: &str = "access-control-allow-methods";

/// The header that names the request headers that are allowed.
pub(crate) const HEADER_ALLOW_HEADERS: &str = "access-control-allow-headers";

/// The header that names the seconds a browser may cache the answer.
pub(crate) const HEADER_MAX_AGE: &str = "access-control-max-age";

/// The header that tells a browser that credentials are allowed.
pub(crate) const HEADER_ALLOW_CREDENTIALS: &str = "access-control-allow-credentials";

/// The header that names the headers the answer depends on.
pub(crate) const HEADER_VARY: &str = "vary";

/// The value of the header that names the headers the answer depends on.
pub(crate) const VALUE_ORIGIN: &str = "origin";

/// The value that stands for every origin.
pub(crate) const WILDCARD_ANY: &str = "*";

/// The methods that are allowed by default.
pub(crate) const ALL_METHODS: [&str; 9] = [
    "GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS", "CONNECT", "TRACE",
];

/// The separator that joins the values of a header.
pub(crate) const SEPARATOR: &str = ", ";

/// The value that tells a browser that credentials are allowed.
pub(crate) const VALUE_TRUE: &str = "true";
