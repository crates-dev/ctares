/// HTTP header name for file ID in chunking operations.
pub const CHUNKIFY_FILE_ID_HEADER: &str = "x-file-id";

/// HTTP header name for chunk index in chunking operations.
pub const CHUNKIFY_CHUNK_INDEX_HEADER: &str = "x-chunk-index";

/// HTTP header name for total chunks count in chunking operations.
pub const CHUNKIFY_TOTAL_CHUNKS_HEADER: &str = "x-total-chunks";

/// HTTP header name for original file name in chunking operations.
pub const CHUNKIFY_FILE_NAME_HEADER: &str = "x-file-name";

/// Error message for a missing `X-File-Id` header.
pub(crate) const MSG_MISSING_FILE_ID_HEADER: &str = "Missing X-File-Id header";

/// Error message for an invalid `X-Chunk-Index` header.
pub(crate) const MSG_INVALID_CHUNK_INDEX_HEADER: &str = "Invalid X-Chunk-Index header";

/// Error message for a missing `X-Chunk-Index` header.
pub(crate) const MSG_MISSING_CHUNK_INDEX_HEADER: &str = "Missing X-Chunk-Index header";

/// Error message for an invalid `X-Total-Chunks` header.
pub(crate) const MSG_INVALID_TOTAL_CHUNKS_HEADER: &str = "Invalid X-Total-Chunks header";

/// Error message for a missing `X-Total-Chunks` header.
pub(crate) const MSG_MISSING_TOTAL_CHUNKS_HEADER: &str = "Missing X-Total-Chunks header";

/// Error message for a missing `X-File-Name` header.
pub(crate) const MSG_MISSING_FILE_NAME_HEADER: &str = "Missing X-File-Name header";

/// Error message for an empty chunk payload.
pub(crate) const MSG_EMPTY_CHUNK_DATA: &str = "Empty chunk data";

/// Error message for a failed file merge operation.
pub(crate) const MSG_MERGE_FAILED: &str = "Failed to complete the file merge operation";
