use super::*;

/// Configuration for chunking operations.
///
/// Contains all necessary parameters for performing chunked file operations.
#[derive(Data)]
pub struct ChunkStrategy<'a> {
    /// The starting index for chunking operations.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) start_chunk_index: usize,
    /// Directory where chunks will be uploaded.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) upload_dir: &'a str,
    /// Function for generating chunk file names.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) file_name_func: Box<dyn ChunkNaming<'a>>,
    /// Unique identifier for the file being chunked.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) file_id: &'a str,
    /// Original name of the file being chunked.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) file_name: &'a str,
    /// Total number of chunks to create.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) total_chunks: usize,
}
