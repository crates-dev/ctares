use super::*;

/// Cursor state for the zero-dependency order-preserving JSON parser.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct JsonParser<'a> {
    /// Full UTF-8 source text.
    #[get(pub(crate), type(copy))]
    #[set(skip)]
    #[get_mut(skip)]
    pub(crate) source: &'a str,
    /// Raw byte view of `source`.
    #[get(pub(crate), type(copy))]
    #[set(skip)]
    #[get_mut(skip)]
    pub(crate) bytes: &'a [u8],
    /// Current byte offset.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    #[get_mut(skip)]
    pub(crate) pos: usize,
}
