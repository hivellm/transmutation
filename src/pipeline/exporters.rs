//! Export layer - convert DocumentStructure to various output formats
//!
//! This follows Docling's architecture: parse once, export many ways

#![allow(dead_code, missing_docs)]

use super::document_structure::DocumentStructure;
use crate::error::Result;

/// Base trait for all exporters
pub trait Exporter {
    type Output;

    fn export(&self, doc: &DocumentStructure) -> Result<Self::Output>;
}

/// Markdown exporter
#[derive(Debug)]
pub struct MarkdownExporter {
    split_pages: bool,
    optimize_for_llm: bool,
}

impl MarkdownExporter {
    pub fn new() -> Self {
        Self {
            split_pages: false,
            optimize_for_llm: false,
        }
    }

    pub fn with_split_pages(mut self, split: bool) -> Self {
        self.split_pages = split;
        self
    }

    pub fn with_llm_optimization(mut self, optimize: bool) -> Self {
        self.optimize_for_llm = optimize;
        self
    }
}

impl Exporter for MarkdownExporter {
    type Output = Vec<String>; // One string per page if split, or single string

    fn export(&self, doc: &DocumentStructure) -> Result<Self::Output> {
        if self.split_pages {
            // Export each page separately
            Ok(doc.pages.iter().map(|p| p.raw_text.clone()).collect())
        } else {
            // Export as single document
            Ok(vec![doc.full_text()])
        }
    }
}

/// JSON exporter
#[derive(Debug)]
pub struct JsonExporter {
    include_metadata: bool,
    structured: bool,
}

impl JsonExporter {
    pub fn new() -> Self {
        Self {
            include_metadata: true,
            structured: true,
        }
    }
}

impl Exporter for JsonExporter {
    type Output = String;

    fn export(&self, doc: &DocumentStructure) -> Result<Self::Output> {
        Ok(serde_json::to_string_pretty(doc)?)
    }
}

/// Image exporter (renders pages to images)
#[derive(Debug)]
pub struct ImageExporter {
    dpi: u32,
    format: ImageFormat,
    quality: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Webp,
}

impl ImageExporter {
    pub fn new() -> Self {
        Self {
            dpi: 150,
            format: ImageFormat::Png,
            quality: 90,
        }
    }

    pub fn with_dpi(mut self, dpi: u32) -> Self {
        self.dpi = dpi;
        self
    }

    pub fn with_format(mut self, format: ImageFormat) -> Self {
        self.format = format;
        self
    }
}

impl Exporter for ImageExporter {
    type Output = Vec<Vec<u8>>; // One image per page

    fn export(&self, _doc: &DocumentStructure) -> Result<Self::Output> {
        // TODO: Implement PDF rendering to images
        todo!("Image export not yet implemented - requires pdfium-render integration")
    }
}

/// Chunking exporter (splits document into chunks for embeddings)
#[derive(Debug)]
pub struct ChunkingExporter {
    chunk_size: usize,
    chunk_overlap: usize,
}

impl ChunkingExporter {
    pub fn new(chunk_size: usize) -> Self {
        Self {
            chunk_size,
            chunk_overlap: chunk_size / 10, // 10% overlap default
        }
    }

    pub fn with_overlap(mut self, overlap: usize) -> Self {
        self.chunk_overlap = overlap;
        self
    }
}

impl Exporter for ChunkingExporter {
    type Output = Vec<String>; // Chunks

    fn export(&self, doc: &DocumentStructure) -> Result<Self::Output> {
        let full_text = doc.full_text();
        let mut chunks = Vec::new();
        let mut start = 0;

        while start < full_text.len() {
            let end = (start + self.chunk_size).min(full_text.len());
            let chunk = full_text[start..end].to_string();
            chunks.push(chunk);

            if end >= full_text.len() {
                break;
            }

            start = end - self.chunk_overlap;
        }

        Ok(chunks)
    }
}

impl Default for MarkdownExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for JsonExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ImageExporter {
    fn default() -> Self {
        Self::new()
    }
}
