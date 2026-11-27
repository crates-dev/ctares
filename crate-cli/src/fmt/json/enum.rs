/// Order-preserving JSON AST node supporting lossless number formatting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum JsonNode {
    /// JSON `null` literal.
    Null,
    /// JSON boolean literal.
    Bool(bool),
    /// Exact original numeric lexeme.
    Number(String),
    /// Raw double-quoted JSON string token (including quotes and escapes).
    StringRaw(String),
    /// Ordered JSON array elements.
    Array(Vec<JsonNode>),
    /// Ordered JSON object key/value pairs (key stored as raw quoted JSON string).
    Object(Vec<(String, JsonNode)>),
}
