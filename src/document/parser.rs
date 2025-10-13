/// Parser for docling-parse JSON output to DoclingDocument
use super::types::*;
use crate::error::{Result, TransmutationError};
use serde_json::Value;
use std::collections::HashMap;

pub struct DoclingJsonParser;

impl DoclingJsonParser {
    /// Parse docling-parse JSON to DoclingDocument
    pub fn parse(json_str: &str) -> Result<DoclingDocument> {
        let json: Value = serde_json::from_str(json_str)?;

        let filename = json["info"]["filename"]
            .as_str()
            .unwrap_or("document")
            .to_string();

        let mut doc = DoclingDocument::new(filename);

        // Extract table of contents for heading detection
        let toc = Self::extract_table_of_contents(&json);
        let heading_map: HashMap<String, usize> = toc
            .iter()
            .map(|(title, level)| (title.to_lowercase(), *level))
            .collect();

        // Process each page
        if let Some(pages) = json["pages"].as_array() {
            for page in pages {
                Self::process_page(page, &mut doc, &heading_map)?;
            }
        }

        Ok(doc)
    }

    fn extract_table_of_contents(json: &Value) -> Vec<(String, usize)> {
        let mut toc = Vec::new();

        if let Some(toc_array) = json["annotations"]["table_of_contents"].as_array() {
            Self::extract_toc_recursive(toc_array, &mut toc);
        }

        toc
    }

    fn extract_toc_recursive(entries: &[Value], result: &mut Vec<(String, usize)>) {
        for entry in entries {
            if let (Some(title), Some(level)) =
                (entry["title"].as_str(), entry["level"].as_u64())
            {
                result.push((title.to_string(), level as usize));
            }

            if let Some(children) = entry["children"].as_array() {
                Self::extract_toc_recursive(children, result);
            }
        }
    }

    fn process_page(
        page: &Value,
        doc: &mut DoclingDocument,
        heading_map: &HashMap<String, usize>,
    ) -> Result<()> {
        // Process cells ordered by position (Y descending, X ascending)
        if let Some(cells_obj) = page["original"]["cells"].as_object() {
            if let Some(cell_data) = cells_obj["data"].as_array() {
                Self::process_cells_ordered(cell_data, doc, heading_map)?;
            }
        }

        Ok(())
    }

    fn process_cells_ordered(
        cell_data: &[Value],
        doc: &mut DoclingDocument,
        heading_map: &HashMap<String, usize>,
    ) -> Result<()> {
        // Group cells by Y position (lines) and order by X
        // Cell structure: [x0, y0, x1, y1, ...]
        let mut cells_with_pos: Vec<(f64, f64, f64, String)> = Vec::new();

        for cell in cell_data {
            if let Some(cell_array) = cell.as_array() {
                // cell_array[0] = x0, cell_array[1] = y0, cell_array[2] = x1, cell_array[12] = text
                if let (Some(x0), Some(x1), Some(y), Some(text)) = (
                    cell_array.get(0).and_then(|v| v.as_f64()),
                    cell_array.get(2).and_then(|v| v.as_f64()),
                    cell_array.get(1).and_then(|v| v.as_f64()),
                    cell_array.get(12).and_then(|v| v.as_str()),
                ) {
                    let trimmed = text.trim();
                    // Filter out non-meaningful single characters and whitespace
                    if !trimmed.is_empty() && !(trimmed.len() == 1 && !trimmed.chars().next().unwrap().is_alphanumeric()) {
                        cells_with_pos.push((y, x0, x1, text.to_string()));
                    }
                }
            }
        }

        // Sort by Y (descending, top to bottom), then X0 (ascending, left to right)
        cells_with_pos.sort_by(|a, b| {
            let y_cmp = b.0.partial_cmp(&a.0).unwrap();
            if y_cmp == std::cmp::Ordering::Equal {
                a.1.partial_cmp(&b.1).unwrap()
            } else {
                y_cmp
            }
        });

        // Group into lines based on Y proximity and build paragraphs
        let mut current_paragraph = Vec::new();
        let mut prev_y = f64::MAX;
        let line_threshold = 5.0; // pixels

        for (y, x0, x1, text) in cells_with_pos {
            if (prev_y - y).abs() > line_threshold {
                // New line detected
                if !current_paragraph.is_empty() {
                    let paragraph_text = Self::join_cells(&current_paragraph);
                    if !paragraph_text.is_empty() {
                        Self::process_text_line(&paragraph_text, doc, heading_map);
                    }
                    current_paragraph.clear();
                }
                prev_y = y;
            }

            current_paragraph.push((x0, x1, text));
        }

        // Flush last paragraph
        if !current_paragraph.is_empty() {
            let paragraph_text = Self::join_cells(&current_paragraph);
            if !paragraph_text.is_empty() {
                Self::process_text_line(&paragraph_text, doc, heading_map);
            }
        }

        Ok(())
    }

    fn join_cells(cells: &[(f64, f64, String)]) -> String {
        let mut result = String::new();
        let mut prev_x_end = 0.0;

        for (x0, x1, text) in cells {
            // Add space if there's a gap between cells
            // Gap threshold of ~2 pixels to account for minor positioning differences
            if prev_x_end > 0.0 {
                let gap = x0 - prev_x_end;
                if gap > 2.0 {
                    result.push(' ');
                }
            }

            result.push_str(text);
            prev_x_end = *x1; // Use actual cell end position
        }

        result.trim().to_string()
    }

    fn process_text_line(
        text: &str,
        doc: &mut DoclingDocument,
        heading_map: &HashMap<String, usize>,
    ) {
        // Check if it's a heading
        let text_lower = text.to_lowercase();
        if let Some(&level) = heading_map.get(&text_lower) {
            if level == 0 {
                doc.add_item(DocItem::Title(TextItem {
                    text: text.to_string(),
                    formatting: None,
                    label: DocItemLabel::Title,
                }));
            } else {
                doc.add_item(DocItem::SectionHeader(SectionHeaderItem {
                    text: text.to_string(),
                    level,
                    formatting: None,
                }));
            }
        } else {
            // Regular paragraph
            doc.add_item(DocItem::Paragraph(TextItem {
                text: text.to_string(),
                formatting: None,
                label: DocItemLabel::Paragraph,
            }));
        }
    }

}

