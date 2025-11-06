//! Cell matching algorithm for table structure
//!
//! Matches predicted table cells with extracted text cells using IoU

#![allow(missing_docs, clippy::unused_self)]

use crate::document::types_extended::{BoundingBox, TextCell};
use crate::error::Result;
use crate::ml::table_structure_model::TableCell;

/// Cell matcher for associating text with table structure
#[derive(Debug)]
pub struct CellMatcher {
    iou_threshold: f64,
}

impl CellMatcher {
    pub fn new() -> Self {
        Self {
            iou_threshold: 0.3, // Minimum IoU to consider a match
        }
    }

    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.iou_threshold = threshold;
        self
    }

    /// Match text cells to table cells
    ///
    /// For each table cell, finds all text cells that overlap with it
    /// and concatenates their text in reading order
    pub fn match_cells(
        &self,
        table_cells: &[TableCell],
        text_cells: &[TextCell],
    ) -> Result<Vec<MatchedCell>> {
        let mut matched = Vec::new();

        for table_cell in table_cells {
            let table_bbox = self.table_cell_to_bbox(table_cell);

            // Find all text cells that overlap with this table cell
            let mut matching_texts = Vec::new();

            for text_cell in text_cells {
                let iou = table_bbox.intersection_over_union(&text_cell.bbox);

                if iou >= self.iou_threshold {
                    matching_texts.push((text_cell, iou));
                }
            }

            // Sort by position (top-to-bottom, left-to-right) then by IoU
            matching_texts.sort_by(|a, b| {
                let y_cmp = a.0.bbox.t.partial_cmp(&b.0.bbox.t).unwrap();
                if y_cmp == std::cmp::Ordering::Equal {
                    a.0.bbox.l.partial_cmp(&b.0.bbox.l).unwrap()
                } else {
                    y_cmp
                }
            });

            // Concatenate text from matching cells
            let text = matching_texts
                .iter()
                .map(|(cell, _)| cell.text.trim())
                .filter(|t| !t.is_empty())
                .collect::<Vec<_>>()
                .join(" ");

            matched.push(MatchedCell {
                row: table_cell.row,
                col: table_cell.col,
                row_span: table_cell.row_span,
                col_span: table_cell.col_span,
                text,
                is_header: table_cell.is_header,
                confidence: self.calculate_match_confidence(&matching_texts),
            });
        }

        Ok(matched)
    }

    /// Convert TableCell bbox tuple to BoundingBox
    fn table_cell_to_bbox(&self, cell: &TableCell) -> BoundingBox {
        let (x0, y0, x1, y1) = cell.bbox;
        BoundingBox::new(
            f64::from(x0),
            f64::from(y0),
            f64::from(x1),
            f64::from(y1),
            crate::document::types_extended::CoordOrigin::TopLeft,
        )
    }

    /// Calculate confidence of cell matching
    fn calculate_match_confidence(&self, matches: &[(&TextCell, f64)]) -> f32 {
        if matches.is_empty() {
            return 0.0;
        }

        // Average IoU of all matches
        let avg_iou: f64 = matches.iter().map(|(_, iou)| iou).sum::<f64>() / matches.len() as f64;

        // Weight by number of matches (more text cells = higher confidence)
        let match_count_factor = (matches.len() as f64).min(3.0) / 3.0;

        (avg_iou * 0.7 + match_count_factor * 0.3) as f32
    }
}

impl Default for CellMatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Cell with matched text content
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct MatchedCell {
    pub row: usize,
    pub col: usize,
    pub row_span: usize,
    pub col_span: usize,
    pub text: String,
    pub is_header: bool,
    pub confidence: f32,
}

impl MatchedCell {
    /// Convert to TableData grid format
    pub fn to_table_data(cells: Vec<MatchedCell>) -> crate::document::types::TableData {
        if cells.is_empty() {
            return crate::document::types::TableData {
                num_rows: 0,
                num_cols: 0,
                grid: Vec::new(),
            };
        }

        // Find dimensions
        let num_rows = cells.iter().map(|c| c.row + c.row_span).max().unwrap_or(0);
        let num_cols = cells.iter().map(|c| c.col + c.col_span).max().unwrap_or(0);

        // Build grid
        let mut grid = vec![Vec::new(); num_rows];

        for cell in cells {
            if cell.row < num_rows {
                grid[cell.row].push(crate::document::types::TableCell {
                    text: cell.text,
                    row_span: cell.row_span,
                    col_span: cell.col_span,
                });
            }
        }

        crate::document::types::TableData {
            num_rows,
            num_cols,
            grid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::types_extended::CoordOrigin;

    #[test]
    fn test_cell_matcher_basic() {
        let matcher = CellMatcher::new();

        let table_cells = vec![TableCell {
            row: 0,
            col: 0,
            row_span: 1,
            col_span: 1,
            bbox: (0.0, 0.0, 10.0, 10.0),
            is_header: true,
        }];

        let text_cells = vec![TextCell {
            index: 0,
            text: "Cell A".to_string(),
            bbox: BoundingBox::new(1.0, 1.0, 9.0, 9.0, CoordOrigin::TopLeft),
            font_name: None,
            font_size: None,
            confidence: 1.0,
            from_ocr: false,
        }];

        let matched = matcher.match_cells(&table_cells, &text_cells).unwrap();

        assert_eq!(matched.len(), 1);
        assert_eq!(matched[0].text, "Cell A");
        assert!(matched[0].is_header);
    }

    #[test]
    fn test_cell_matcher_multiple_texts() {
        let matcher = CellMatcher::new();

        let table_cells = vec![TableCell {
            row: 0,
            col: 0,
            row_span: 1,
            col_span: 1,
            bbox: (0.0, 0.0, 20.0, 10.0),
            is_header: false,
        }];

        let text_cells = vec![
            TextCell {
                index: 0,
                text: "Part 1".to_string(),
                bbox: BoundingBox::new(1.0, 1.0, 8.0, 9.0, CoordOrigin::TopLeft),
                font_name: None,
                font_size: None,
                confidence: 1.0,
                from_ocr: false,
            },
            TextCell {
                index: 1,
                text: "Part 2".to_string(),
                bbox: BoundingBox::new(12.0, 1.0, 18.0, 9.0, CoordOrigin::TopLeft),
                font_name: None,
                font_size: None,
                confidence: 1.0,
                from_ocr: false,
            },
        ];

        let matched = matcher.match_cells(&table_cells, &text_cells).unwrap();

        assert_eq!(matched.len(), 1);
        assert_eq!(matched[0].text, "Part 1 Part 2");
    }

    #[test]
    fn test_to_table_data() {
        let cells = vec![
            MatchedCell {
                row: 0,
                col: 0,
                row_span: 1,
                col_span: 1,
                text: "A".to_string(),
                is_header: true,
                confidence: 0.9,
            },
            MatchedCell {
                row: 0,
                col: 1,
                row_span: 1,
                col_span: 1,
                text: "B".to_string(),
                is_header: true,
                confidence: 0.9,
            },
            MatchedCell {
                row: 1,
                col: 0,
                row_span: 1,
                col_span: 1,
                text: "1".to_string(),
                is_header: false,
                confidence: 0.8,
            },
        ];

        let table_data = MatchedCell::to_table_data(cells);

        assert_eq!(table_data.num_rows, 2);
        assert_eq!(table_data.num_cols, 2);
        assert_eq!(table_data.grid[0].len(), 2); // First row has 2 cells
        assert_eq!(table_data.grid[0][0].text, "A");
        assert_eq!(table_data.grid[0][1].text, "B");
    }
}
