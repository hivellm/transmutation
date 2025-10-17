/// Intermediate document representation (format-agnostic)
/// 
/// This structure is inspired by Docling's DocumentModel
/// It represents the parsed document independent of input/output formats

use crate::error::Result;
use std::path::Path;
use serde::{Serialize, Deserialize};

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

