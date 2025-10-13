/// Document model inspired by docling-core
/// Pure Rust implementation for 100% Python independence
pub mod types;
pub mod types_extended;
pub mod parser;
pub mod serializer;
pub mod text_utils;
pub mod page_assembler;
pub mod hierarchy_builder;

pub use types::*;
pub use types_extended::*;
pub use parser::DoclingJsonParser;
pub use serializer::MarkdownSerializer;
pub use text_utils::{TextSanitizer, sanitize_text};
pub use page_assembler::{PageAssembler, PageAssemblerOptions};
pub use hierarchy_builder::{HierarchyBuilder, RelationshipBuilder};

