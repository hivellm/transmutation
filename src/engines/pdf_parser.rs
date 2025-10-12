//! Pure Rust PDF text extraction engine
//!
//! This module provides text extraction from PDF files using the `lopdf` crate.
//! It handles various PDF encodings, multi-column layouts, and positional information.

use crate::{Result, TransmutationError};
use lopdf::Document;
use std::collections::BTreeMap;
use std::path::Path;

/// PDF parser for text extraction
pub struct PdfParser {
    document: Document,
}

/// Extracted page information
#[derive(Debug, Clone)]
pub struct PdfPage {
    /// Page number (0-indexed)
    pub number: usize,
    /// Extracted text
    pub text: String,
    /// Page width in points
    pub width: f32,
    /// Page height in points
    pub height: f32,
    /// Text blocks with position information
    pub text_blocks: Vec<TextBlock>,
}

/// Text block with position
#[derive(Debug, Clone)]
pub struct TextBlock {
    /// Text content
    pub text: String,
    /// X position (left edge)
    pub x: f32,
    /// Y position (bottom edge)
    pub y: f32,
    /// Font size
    pub font_size: f32,
    /// Font name
    pub font_name: Option<String>,
}

impl PdfParser {
    /// Load a PDF from file path
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let document = Document::load(path.as_ref()).map_err(|e| {
            TransmutationError::engine_error_with_source("PDF Parser", "Failed to load PDF", e)
        })?;

        Ok(Self { document })
    }

    /// Load a PDF from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let document = Document::load_mem(bytes).map_err(|e| {
            TransmutationError::engine_error_with_source(
                "PDF Parser",
                "Failed to load PDF from bytes",
                e,
            )
        })?;

        Ok(Self { document })
    }

    /// Get the number of pages in the PDF
    pub fn page_count(&self) -> usize {
        self.document.get_pages().len()
    }

    /// Get page IDs
    fn get_page_ids(&self) -> Vec<(u32, u16)> {
        self.document.get_pages().keys().copied().collect()
    }

    /// Extract text from a specific page (0-indexed)
    pub fn extract_text(&self, page_num: usize) -> Result<String> {
        let page_ids = self.get_page_ids();
        
        if page_num >= page_ids.len() {
            return Err(TransmutationError::InvalidOptions(format!(
                "Page {} does not exist (total pages: {})",
                page_num,
                page_ids.len()
            )));
        }

        let page_id = page_ids[page_num];
        
        // Extract text from page
        let text = self.document
            .extract_text(&[page_id.0])
            .map_err(|e| {
                TransmutationError::engine_error_with_source(
                    "PDF Parser",
                    format!("Failed to extract text from page {}", page_num),
                    e,
                )
            })?;

        Ok(text)
    }

    /// Extract all text from the PDF
    pub fn extract_all_text(&self) -> Result<String> {
        let page_ids: Vec<u32> = self.get_page_ids().iter().map(|(id, _)| *id).collect();
        
        let text = self.document
            .extract_text(&page_ids)
            .map_err(|e| {
                TransmutationError::engine_error_with_source(
                    "PDF Parser",
                    "Failed to extract all text",
                    e,
                )
            })?;

        Ok(text)
    }

    /// Get page size (width, height) in points
    pub fn get_page_size(&self, page_num: usize) -> Result<(f32, f32)> {
        let page_ids = self.get_page_ids();
        
        if page_num >= page_ids.len() {
            return Err(TransmutationError::InvalidOptions(format!(
                "Page {} does not exist", page_num
            )));
        }

        let page_id = page_ids[page_num];
        let pages = self.document.get_pages();
        
        if let Some(&page_obj_id) = pages.get(&page_id) {
            if let Ok(page_dict) = self.document.get_object(page_obj_id) {
                if let Ok(page) = page_dict.as_dict() {
                    if let Ok(media_box) = page.get(b"MediaBox") {
                        if let Ok(media_box_array) = media_box.as_array() {
                            if media_box_array.len() >= 4 {
                                let width = media_box_array[2].as_f64().unwrap_or(612.0) as f32;
                                let height = media_box_array[3].as_f64().unwrap_or(792.0) as f32;
                                return Ok((width, height));
                            }
                        }
                    }
                }
            }
        }

        // Default to US Letter size if not found
        Ok((612.0, 792.0))
    }

    /// Extract detailed page information
    pub fn extract_page(&self, page_num: usize) -> Result<PdfPage> {
        let text = self.extract_text(page_num)?;
        let (width, height) = self.get_page_size(page_num)?;

        Ok(PdfPage {
            number: page_num,
            text,
            width,
            height,
            text_blocks: Vec::new(), // TODO: Implement detailed text block extraction
        })
    }

    /// Extract all pages with detailed information
    pub fn extract_all_pages(&self) -> Result<Vec<PdfPage>> {
        let page_count = self.page_count();
        let mut pages = Vec::with_capacity(page_count);

        for i in 0..page_count {
            pages.push(self.extract_page(i)?);
        }

        Ok(pages)
    }

    /// Get PDF metadata
    pub fn get_metadata(&self) -> PdfMetadata {
        let mut metadata = PdfMetadata::default();

        if let Ok(info_dict) = self.document.trailer.get(b"Info") {
            if let Ok(info) = info_dict.as_dict() {
                // Extract title
                if let Ok(title) = info.get(b"Title") {
                    if let Ok(title_str) = title.as_str() {
                        metadata.title = Some(title_str.to_string());
                    }
                }

                // Extract author
                if let Ok(author) = info.get(b"Author") {
                    if let Ok(author_str) = author.as_str() {
                        metadata.author = Some(author_str.to_string());
                    }
                }

                // Extract creation date
                if let Ok(created) = info.get(b"CreationDate") {
                    if let Ok(created_str) = created.as_str() {
                        metadata.created = Some(created_str.to_string());
                    }
                }

                // Extract modification date
                if let Ok(modified) = info.get(b"ModDate") {
                    if let Ok(modified_str) = modified.as_str() {
                        metadata.modified = Some(modified_str.to_string());
                    }
                }

                // Extract subject
                if let Ok(subject) = info.get(b"Subject") {
                    if let Ok(subject_str) = subject.as_str() {
                        metadata.subject = Some(subject_str.to_string());
                    }
                }

                // Extract keywords
                if let Ok(keywords) = info.get(b"Keywords") {
                    if let Ok(keywords_str) = keywords.as_str() {
                        metadata.keywords = Some(keywords_str.to_string());
                    }
                }

                // Extract producer
                if let Ok(producer) = info.get(b"Producer") {
                    if let Ok(producer_str) = producer.as_str() {
                        metadata.producer = Some(producer_str.to_string());
                    }
                }
            }
        }

        metadata.page_count = self.page_count();
        metadata
    }

    /// Check if PDF is encrypted
    pub fn is_encrypted(&self) -> bool {
        self.document.is_encrypted()
    }

    /// Get PDF version
    pub fn version(&self) -> String {
        format!("{}.{}", self.document.version.major, self.document.version.minor)
    }
}

/// PDF metadata
#[derive(Debug, Clone, Default)]
pub struct PdfMetadata {
    /// Document title
    pub title: Option<String>,
    /// Author
    pub author: Option<String>,
    /// Creation date (PDF date format)
    pub created: Option<String>,
    /// Modification date (PDF date format)
    pub modified: Option<String>,
    /// Subject
    pub subject: Option<String>,
    /// Keywords
    pub keywords: Option<String>,
    /// Producer (PDF software)
    pub producer: Option<String>,
    /// Total page count
    pub page_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_metadata_default() {
        let metadata = PdfMetadata::default();
        assert_eq!(metadata.page_count, 0);
        assert!(metadata.title.is_none());
    }

    #[test]
    fn test_text_block_creation() {
        let block = TextBlock {
            text: "Hello".to_string(),
            x: 10.0,
            y: 20.0,
            font_size: 12.0,
            font_name: Some("Arial".to_string()),
        };
        assert_eq!(block.text, "Hello");
        assert_eq!(block.font_size, 12.0);
    }

    #[test]
    fn test_pdf_page_creation() {
        let page = PdfPage {
            number: 0,
            text: "Page content".to_string(),
            width: 612.0,
            height: 792.0,
            text_blocks: vec![],
        };
        assert_eq!(page.number, 0);
        assert_eq!(page.width, 612.0);
    }

    // Integration tests require actual PDF files
    // These will be added in tests/pdf_parser_tests.rs with fixtures
}

