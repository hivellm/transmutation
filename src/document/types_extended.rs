/// Extended document types based on docling-core
/// 
/// This module provides complete type definitions for document structure,
/// matching the docling-core Python implementation.
/// 
/// Reference: transmutation/docling-core/docling_core/types/doc/

use serde::{Deserialize, Serialize};

/// Coordinate origin for bounding boxes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordOrigin {
    TopLeft,
    BottomLeft,
}

/// Bounding box with coordinates
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoundingBox {
    pub l: f64,  // left
    pub t: f64,  // top
    pub r: f64,  // right
    pub b: f64,  // bottom
    pub origin: CoordOrigin,
}

impl BoundingBox {
    pub fn new(l: f64, t: f64, r: f64, b: f64, origin: CoordOrigin) -> Self {
        Self { l, t, r, b, origin }
    }
    
    pub fn width(&self) -> f64 {
        (self.r - self.l).abs()
    }
    
    pub fn height(&self) -> f64 {
        (self.b - self.t).abs()
    }
    
    pub fn area(&self) -> f64 {
        self.width() * self.height()
    }
    
    /// Calculate intersection over union (IoU)
    pub fn intersection_over_union(&self, other: &BoundingBox) -> f64 {
        let inter_l = self.l.max(other.l);
        let inter_t = self.t.max(other.t);
        let inter_r = self.r.min(other.r);
        let inter_b = self.b.min(other.b);
        
        if inter_r <= inter_l || inter_b <= inter_t {
            return 0.0;
        }
        
        let inter_area = (inter_r - inter_l) * (inter_b - inter_t);
        let union_area = self.area() + other.area() - inter_area;
        
        if union_area > 0.0 {
            inter_area / union_area
        } else {
            0.0
        }
    }
    
    /// Calculate intersection over self
    pub fn intersection_over_self(&self, other: &BoundingBox) -> f64 {
        let inter_l = self.l.max(other.l);
        let inter_t = self.t.max(other.t);
        let inter_r = self.r.min(other.r);
        let inter_b = self.b.min(other.b);
        
        if inter_r <= inter_l || inter_b <= inter_t {
            return 0.0;
        }
        
        let inter_area = (inter_r - inter_l) * (inter_b - inter_t);
        let self_area = self.area();
        
        if self_area > 0.0 {
            inter_area / self_area
        } else {
            0.0
        }
    }
    
    pub fn as_tuple(&self) -> (f64, f64, f64, f64) {
        (self.l, self.t, self.r, self.b)
    }
}

/// Page size
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

/// Text cell with position and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextCell {
    pub index: usize,
    pub text: String,
    pub bbox: BoundingBox,
    pub font_name: Option<String>,
    pub font_size: Option<f32>,
    pub confidence: f32,
    pub from_ocr: bool,
}

/// Cluster of text cells (detected region)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cluster {
    pub id: usize,
    pub label: DocItemLabel,
    pub bbox: BoundingBox,
    pub cells: Vec<TextCell>,
    pub confidence: f32,
}

/// Layout prediction for a page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutPrediction {
    pub clusters: Vec<Cluster>,
    pub page_size: Size,
}

/// Complete document item labels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DocItemLabel {
    Title,
    DocumentIndex,         // Table of contents
    SectionHeader,
    Paragraph,
    Text,
    Caption,
    Footnote,
    PageHeader,
    PageFooter,
    ListItem,
    Code,
    Formula,
    Table,
    Picture,
    Figure,                // Same as Picture
    CheckboxSelected,
    CheckboxUnselected,
    Reference,
    KeyValueRegion,
    Form,
}

impl DocItemLabel {
    pub fn is_text_element(&self) -> bool {
        matches!(
            self,
            DocItemLabel::Text
                | DocItemLabel::Paragraph
                | DocItemLabel::SectionHeader
                | DocItemLabel::Caption
                | DocItemLabel::Footnote
                | DocItemLabel::PageHeader
                | DocItemLabel::PageFooter
                | DocItemLabel::ListItem
        )
    }
    
    pub fn is_container(&self) -> bool {
        matches!(self, DocItemLabel::Form | DocItemLabel::KeyValueRegion)
    }
}

/// Text formatting attributes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Formatting {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub subscript: bool,
    pub superscript: bool,
}

/// Table cell with structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCellData {
    pub text: String,
    pub row_span: usize,
    pub col_span: usize,
    pub start_row_offset_idx: usize,
    pub end_row_offset_idx: usize,
    pub start_col_offset_idx: usize,
    pub end_col_offset_idx: usize,
    pub column_header: bool,
    pub row_header: bool,
    pub bbox: Option<BoundingBox>,
}

/// Table structure data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    pub num_rows: usize,
    pub num_cols: usize,
    pub cells: Vec<TableCellData>,
}

impl TableData {
    /// Build 2D grid representation
    pub fn to_grid(&self) -> Vec<Vec<TableCellData>> {
        let mut grid = vec![vec![]; self.num_rows];
        
        for cell in &self.cells {
            if cell.start_row_offset_idx < self.num_rows {
                grid[cell.start_row_offset_idx].push(cell.clone());
            }
        }
        
        grid
    }
}

/// Image reference modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageRefMode {
    Placeholder,
    Embedded,
    Referenced,
}

/// Reference to an image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageRef {
    pub uri: String,
    pub mode: ImageRefMode,
    pub mimetype: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// Code language label
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeLanguage {
    Python,
    JavaScript,
    TypeScript,
    Rust,
    Java,
    Cpp,
    C,
    Go,
    Ruby,
    PHP,
    Swift,
    Kotlin,
    Bash,
    SQL,
    HTML,
    CSS,
    JSON,
    XML,
    YAML,
    Markdown,
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bbox_intersection() {
        let bbox1 = BoundingBox::new(0.0, 0.0, 10.0, 10.0, CoordOrigin::TopLeft);
        let bbox2 = BoundingBox::new(5.0, 5.0, 15.0, 15.0, CoordOrigin::TopLeft);
        
        let iou = bbox1.intersection_over_union(&bbox2);
        assert!(iou > 0.0 && iou < 1.0);
    }
    
    #[test]
    fn test_bbox_area() {
        let bbox = BoundingBox::new(0.0, 0.0, 10.0, 20.0, CoordOrigin::TopLeft);
        assert_eq!(bbox.area(), 200.0);
    }
    
    #[test]
    fn test_label_is_text() {
        assert!(DocItemLabel::Paragraph.is_text_element());
        assert!(!DocItemLabel::Table.is_text_element());
    }
}

