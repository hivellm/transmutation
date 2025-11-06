//! Intermediate document representation (format-agnostic)
//!
//! This structure is inspired by Docling's DocumentModel
//! It represents the parsed document independent of input/output formats

#![allow(missing_docs)]

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::Result;

/// Universal document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentStructure {
    /// Document title
    pub title: Option<String>,

    /// Author information
    pub author: Option<String>,

    /// Pages
    pub pages: Vec<PageStructure>,

    /// Document-level metadata
    pub metadata: DocumentMetadata,
}

/// Page structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageStructure {
    /// Page number (1-indexed)
    pub number: usize,

    /// Page dimensions (points)
    pub width: f32,
    pub height: f32,

    /// Content blocks
    pub blocks: Vec<ContentBlock>,

    /// Raw text for this page
    pub raw_text: String,
}

/// Content block (text, table, image, etc)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    Text {
        text: String,
        style: TextStyle,
        bbox: Option<BoundingBox>,
    },
    Heading {
        text: String,
        level: usize,
        bbox: Option<BoundingBox>,
    },
    List {
        items: Vec<String>,
        ordered: bool,
        bbox: Option<BoundingBox>,
    },
    Table {
        rows: Vec<Vec<String>>,
        bbox: Option<BoundingBox>,
    },
    Image {
        data: Option<Vec<u8>>,
        alt_text: Option<String>,
        bbox: Option<BoundingBox>,
    },
    Formula {
        latex: String,
        bbox: Option<BoundingBox>,
    },
}

/// Text styling
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextStyle {
    pub bold: bool,
    pub italic: bool,
    pub font_size: Option<f32>,
    pub font_family: Option<String>,
}

/// Bounding box
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Document metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentMetadata {
    pub created: Option<String>,
    pub modified: Option<String>,
    pub language: Option<String>,
    pub page_count: usize,
}

impl DocumentStructure {
    /// Parse from PDF (placeholder)
    pub async fn from_pdf(_path: &Path) -> Result<Self> {
        // TODO: Implement PDF parsing to DocumentStructure
        todo!("PDF parsing to DocumentStructure not yet implemented")
    }

    /// Get total text content
    pub fn full_text(&self) -> String {
        self.pages
            .iter()
            .map(|p| p.raw_text.as_str())
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    /// Get page by number (1-indexed)
    pub fn get_page(&self, number: usize) -> Option<&PageStructure> {
        self.pages.iter().find(|p| p.number == number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_structure_creation() {
        let doc = DocumentStructure {
            title: Some("Test Doc".to_string()),
            author: Some("Test Author".to_string()),
            pages: vec![],
            metadata: DocumentMetadata::default(),
        };
        assert_eq!(doc.title, Some("Test Doc".to_string()));
        assert_eq!(doc.author, Some("Test Author".to_string()));
    }

    #[test]
    fn test_page_structure_creation() {
        let page = PageStructure {
            number: 1,
            width: 612.0,
            height: 792.0,
            blocks: vec![],
            raw_text: "Test text".to_string(),
        };
        assert_eq!(page.number, 1);
        assert_eq!(page.raw_text, "Test text");
    }

    #[test]
    fn test_full_text() {
        let page1 = PageStructure {
            number: 1,
            width: 612.0,
            height: 792.0,
            blocks: vec![],
            raw_text: "Page 1".to_string(),
        };
        let page2 = PageStructure {
            number: 2,
            width: 612.0,
            height: 792.0,
            blocks: vec![],
            raw_text: "Page 2".to_string(),
        };
        let doc = DocumentStructure {
            title: None,
            author: None,
            pages: vec![page1, page2],
            metadata: DocumentMetadata::default(),
        };
        let full = doc.full_text();
        assert!(full.contains("Page 1"));
        assert!(full.contains("Page 2"));
    }

    #[test]
    fn test_get_page() {
        let page1 = PageStructure {
            number: 1,
            width: 612.0,
            height: 792.0,
            blocks: vec![],
            raw_text: "Page 1".to_string(),
        };
        let doc = DocumentStructure {
            title: None,
            author: None,
            pages: vec![page1],
            metadata: DocumentMetadata::default(),
        };
        assert!(doc.get_page(1).is_some());
        assert!(doc.get_page(2).is_none());
    }

    #[test]
    fn test_text_style_default() {
        let style = TextStyle::default();
        assert!(!style.bold);
        assert!(!style.italic);
    }

    #[test]
    fn test_bounding_box_creation() {
        let bbox = BoundingBox {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 50.0,
        };
        assert_eq!(bbox.width, 100.0);
        assert_eq!(bbox.height, 50.0);
    }

    #[test]
    fn test_content_block_text() {
        let block = ContentBlock::Text {
            text: "Test".to_string(),
            style: TextStyle::default(),
            bbox: None,
        };
        match block {
            ContentBlock::Text { text, .. } => assert_eq!(text, "Test"),
            _ => panic!("Expected Text block"),
        }
    }
}
