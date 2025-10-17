/// Document processing pipeline (inspired by Docling architecture)
///
/// Separates extraction from export for maximum flexibility:
/// Input → Parser → DocumentStructure → Exporter → Output(s)
pub mod document_structure;
pub mod exporters;

use std::path::Path;

pub use document_structure::DocumentStructure;
pub use exporters::{ChunkingExporter, Exporter, ImageExporter, JsonExporter, MarkdownExporter};

use crate::error::Result;

/// Pipeline for document processing
///
/// Example:
/// ```rust
/// let pipeline = DocumentPipeline::new();
/// let doc = pipeline.parse_pdf("input.pdf")?;
///
/// // Export to multiple formats from same document
/// let md = MarkdownExporter::new().export(&doc)?;
/// let json = JsonExporter::new().export(&doc)?;
/// let images = ImageExporter::new().export_pages(&doc)?;
/// let chunks = ChunkingExporter::new(512).export(&doc)?;
/// ```
pub struct DocumentPipeline {
    // Configuration for parsing
}

impl DocumentPipeline {
    pub fn new() -> Self {
        Self {}
    }

    /// Parse document into intermediate representation
    pub async fn parse(&self, path: &Path) -> Result<DocumentStructure> {
        // TODO: Detect format and route to appropriate parser
        DocumentStructure::from_pdf(path).await
    }
}

impl Default for DocumentPipeline {
    fn default() -> Self {
        Self::new()
    }
}
