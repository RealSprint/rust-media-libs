use std::io;
use thiserror::Error;

/// Data for when an error occurs while attempting to deserialize a RTMP chunk
/// An enumeration defining all the possible errors that could occur while deserializing
/// RTMP chunks.
#[derive(Debug, Error)]
pub enum ChunkDeserializationError {
    /// The max chunk size does not allow chunk sizes more than 2,147,483,647 (since it's encoded in only
    /// 31 bytes of the SetChunkSize message), so this error occurs when a chunk size of greater than
    /// this value is attempted to be set
    #[error("Requested an invalid max chunk size of {chunk_size}.  The largest chunk size possible is 2147483647")]
    InvalidMaxChunkSize { chunk_size: usize },

    /// An I/O error occurred while reading the input buffer
    #[error("{0}")]
    Io(#[from] io::Error),
}
