/// Document model inspired by docling-core
/// Pure Rust implementation for 100% Python independence
pub mod types;
pub mod types_extended;
pub mod parser;
pub mod serializer;
pub mod text_utils;
pub mod page_assembler;
pub mod hierarchy_builder;

// Export only from types (primary source of truth)
pub use types::*;

// Export only non-duplicate items from types_extended
pub use types_extended::{BoundingBox, CoordOrigin, Size, TextCell, Cluster, LayoutPrediction};

pub use parser::DoclingJsonParser;
pub use serializer::MarkdownSerializer;
pub use text_utils::{TextSanitizer, sanitize_text};
pub use page_assembler::{PageAssembler, PageAssemblerOptions};
pub use hierarchy_builder::{HierarchyBuilder, RelationshipBuilder};

