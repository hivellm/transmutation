/// Document model inspired by docling-core
/// Pure Rust implementation for 100% Python independence
pub mod types;
pub mod types_extended;
pub mod parser;
pub mod serializer;

pub use types::*;
pub use types_extended::*;
pub use parser::DoclingJsonParser;
pub use serializer::MarkdownSerializer;

