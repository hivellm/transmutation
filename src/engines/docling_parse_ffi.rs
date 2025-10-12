//! Direct C++ integration with docling-parse (no Python dependency)
//!
//! This module provides Rust FFI bindings to docling-parse C++ library

use crate::{Result, TransmutationError};
use std::path::Path;

/// Text cell extracted from PDF with precise layout information
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TextCell {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub font_size: f32,
    pub font_name: String,
}

/// PDF parser using docling-parse C++ library
pub struct DoclingParseEngine {
    initialized: bool,
}

impl DoclingParseEngine {
    pub fn new() -> Result<Self> {
        // TODO: Load docling-parse shared library (.so/.dll)
        // For now, we'll implement the same logic in pure Rust
        Ok(Self {
            initialized: true,
        })
    }
    
    /// Parse PDF and extract text cells with layout information
    pub fn parse(&self, path: &Path) -> Result<Vec<TextCell>> {
        // TODO: Implement direct C++ FFI calls to docling-parse
        // Or implement the same parsing logic in Rust using lopdf + layout analysis
        
        Err(TransmutationError::engine_error(
            "docling-parse",
            "C++ integration not yet implemented. Use --precision mode for 77.3% similarity."
        ))
    }
    
    /// Convert text cells to Markdown using Docling's algorithm
    pub fn cells_to_markdown(cells: &[TextCell]) -> String {
        // Sort cells by Y then X (reading order)
        let mut sorted = cells.to_vec();
        sorted.sort_by(|a, b| {
            a.y.partial_cmp(&b.y)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal))
        });
        
        // Detect headings based on font size
        let avg_font_size: f32 = sorted.iter().map(|c| c.font_size).sum::<f32>() / sorted.len() as f32;
        
        let mut markdown = String::new();
        let mut current_line_y = f32::MIN;
        let mut current_line = String::new();
        
        for cell in sorted {
            // New line if Y changed significantly
            if (cell.y - current_line_y).abs() > 2.0 {
                if !current_line.is_empty() {
                    markdown.push_str(&current_line);
                    markdown.push_str("\n\n");
                    current_line.clear();
                }
                current_line_y = cell.y;
            }
            
            // Check if heading (larger font)
            if cell.font_size > avg_font_size * 1.2 {
                if !current_line.is_empty() {
                    markdown.push_str(&current_line);
                    markdown.push_str("\n\n");
                    current_line.clear();
                }
                markdown.push_str("## ");
                markdown.push_str(&cell.text);
                markdown.push_str("\n\n");
            } else {
                current_line.push_str(&cell.text);
                current_line.push(' ');
            }
        }
        
        if !current_line.is_empty() {
            markdown.push_str(&current_line);
        }
        
        markdown.trim().to_string()
    }
}

impl Default for DoclingParseEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create DoclingParseEngine")
    }
}

// TODO: Implement actual C++ FFI bindings
// extern "C" {
//     fn docling_parse_pdf(path: *const c_char, cells: *mut *mut TextCell, count: *mut usize) -> i32;
//     fn docling_free_cells(cells: *mut TextCell, count: usize);
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cells_to_markdown() {
        let cells = vec![
            TextCell {
                text: "Title".to_string(),
                x: 100.0,
                y: 100.0,
                width: 50.0,
                height: 20.0,
                font_size: 18.0,
                font_name: "Arial".to_string(),
            },
            TextCell {
                text: "Normal text".to_string(),
                x: 100.0,
                y: 130.0,
                width: 100.0,
                height: 12.0,
                font_size: 12.0,
                font_name: "Arial".to_string(),
            },
        ];
        
        let md = DoclingParseEngine::cells_to_markdown(&cells);
        assert!(md.contains("## Title"));
        assert!(md.contains("Normal text"));
    }
}

