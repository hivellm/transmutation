//! Output format generators

pub mod chunker;
pub mod markdown;

// TODO: Implement other output formats
// pub mod image;
// pub mod json;
// pub mod csv;

pub use chunker::{Chunker, ChunkStrategy, TextChunk};
pub use markdown::MarkdownGenerator;
