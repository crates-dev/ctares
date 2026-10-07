/// Constant for the "get" function type.
pub(crate) const GET: &str = "get";

/// Constant for the "get_mut" function type.
pub(crate) const GET_MUT: &str = "get_mut";

/// Constant for the "set" function type.
pub(crate) const SET: &str = "set";

/// Constant for the "debug" attribute.
pub(crate) const DEBUG: &str = "debug";

/// Constant for the "new" function type.
pub(crate) const NEW: &str = "new";

/// Constant for the "skip" attribute.
pub(crate) const SKIP: &str = "skip";

/// Constant for the "pub" visibility modifier.
pub(crate) const PUB: &str = "pub";

/// Constant for private visibility.
pub(crate) const PRIVATE: &str = "private";

/// Constant for the "crate" visibility modifier.
pub(crate) const CRATE: &str = "crate";

/// Constant for the "pub(crate)" visibility modifier.
pub(crate) const PUB_CRATE: &str = "pub(crate)";

/// Constant for the "super" visibility modifier.
pub(crate) const SUPER: &str = "super";

/// Constant for the "pub(super)" visibility modifier.
pub(crate) const PUB_SUPER: &str = "pub(super)";

/// Constant for return clone type.
pub(crate) const CLONE: &str = "clone";

/// Constant for return copy type.
pub(crate) const COPY: &str = "copy";

/// Constant for return deref type.
pub(crate) const DEREF: &str = "deref";

/// Constant for type specification.
pub(crate) const CUSTOM_TYPE: &str = "type";

/// Constant for AsRef trait bound prefix.
pub(crate) const AS_REF_PREFIX: &str = "AsRef<";

/// Constant for Into trait bound prefix.
pub(crate) const INTO_PREFIX: &str = "Into<";

/// Constant for AsMut trait bound prefix.
pub(crate) const AS_MUT_PREFIX: &str = "AsMut<";

/// Constant for Deref trait bound prefix.
pub(crate) const DEREF_PREFIX: &str = "Deref<";

/// Constant for impl keyword prefix.
pub(crate) const IMPL_PREFIX: &str = "impl ";

/// Constant for opening angle bracket character.
pub(crate) const OPEN_BRACKET: char = '<';

/// Constant for closing angle bracket character.
pub(crate) const CLOSE_BRACKET: char = '>';
