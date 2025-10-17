//! Output format generators

pub mod chunker;
pub mod markdown;

// TODO: Implement other output formats
// pub mod image;
// pub mod json;
// pub mod csv;

pub use chunker::{ChunkStrategy, Chunker, TextChunk};
pub use markdown::MarkdownGenerator;
