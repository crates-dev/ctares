use super::*;

/// The rules that decide which encoding an answer carries.
///
/// The policy holds no server state, so the same rules can be used by any
/// server that answers with a compressed body.
#[derive(Clone, Debug, Getter)]
pub struct CompressionPolicy {
    /// The encodings that are offered, in the order they are preferred.
    pub encodings: Vec<String>,
    /// The smallest body that is worth compressing.
    #[get(type(copy))]
    pub min_size: usize,
}
