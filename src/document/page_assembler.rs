use crate::document::text_utils::{
    TextSanitizer, calculate_section_level, extract_section_number, is_likely_heading,
};
/// Page assembly - convert detected clusters into structured document elements
///
/// Based on docling's page_assemble_model.py
use crate::document::types::*;
use crate::document::types_extended::*;
use crate::error::Result;

/// Page assembler options
#[derive(Debug)]
pub struct PageAssemblerOptions {
    pub enable_text_sanitization: bool,
    pub enable_heading_detection: bool,
    pub enable_list_detection: bool,
    pub merge_adjacent_text: bool,
}

impl Default for PageAssemblerOptions {
    fn default() -> Self {
        Self {
            enable_text_sanitization: true,
            enable_heading_detection: true,
            enable_list_detection: true,
            merge_adjacent_text: true,
        }
    }
}

/// Page assembler - converts layout clusters to document items
#[derive(Debug)]
pub struct PageAssembler {
    options: PageAssemblerOptions,
    sanitizer: TextSanitizer,
}

impl PageAssembler {
    pub fn new(options: PageAssemblerOptions) -> Self {
        Self {
            options,
            sanitizer: TextSanitizer::new(),
        }
    }

    /// Assemble document items from clusters
    pub fn assemble(&self, clusters: &[Cluster]) -> Result<Vec<DocItem>> {
        let mut items = Vec::new();

        for cluster in clusters {
            let doc_items = self.process_cluster(cluster)?;
            items.extend(doc_items);
        }

        // Post-processing: merge adjacent text blocks if enabled
        if self.options.merge_adjacent_text {
            items = self.merge_adjacent_text_items(items)?;
        }

        Ok(items)
    }

    /// Process a single cluster based on its label
    fn process_cluster(&self, cluster: &Cluster) -> Result<Vec<DocItem>> {
        match cluster.label {
            DocItemLabel::Title => self.process_title(cluster),
            DocItemLabel::SectionHeader => self.process_section_header(cluster),
            DocItemLabel::Paragraph | DocItemLabel::Text => self.process_text(cluster),
            DocItemLabel::ListItem => self.process_list_item(cluster),
            DocItemLabel::Caption => self.process_caption(cluster),
            DocItemLabel::Footnote => self.process_footnote(cluster),
            DocItemLabel::PageHeader | DocItemLabel::PageFooter => {
                self.process_header_footer(cluster)
            }
            DocItemLabel::Table => self.process_table(cluster),
            DocItemLabel::Picture | DocItemLabel::Figure => self.process_picture(cluster),
            DocItemLabel::Code => self.process_code(cluster),
            DocItemLabel::Formula => self.process_formula(cluster),
            DocItemLabel::CheckboxSelected | DocItemLabel::CheckboxUnselected => {
                self.process_checkbox(cluster)
            }
        }
    }

    /// Extract and sanitize text from cluster cells
    fn extract_text(&self, cluster: &Cluster) -> String {
        // Sort cells by position (Y then X)
        let mut cells = cluster.cells.clone();
        cells.sort_by(|a, b| {
            let y_cmp = a.bbox.t.partial_cmp(&b.bbox.t).unwrap();
            if y_cmp == std::cmp::Ordering::Equal {
                a.bbox.l.partial_cmp(&b.bbox.l).unwrap()
            } else {
                y_cmp
            }
        });

        // Smart joining: docling-parse returns one character per cell
        // We need to detect word boundaries based on horizontal distance
        let mut text = String::new();
        let mut prev_x_end = 0.0;
        let mut prev_y = 0.0;

        for cell in &cells {
            let gap_x = cell.bbox.l - prev_x_end;
            let gap_y = (cell.bbox.t - prev_y).abs();
            let cell_width = cell.bbox.r - cell.bbox.l;

            // New line if vertical gap is significant
            if prev_y > 0.0 && gap_y > 5.0 {
                if !text.ends_with('\n') {
                    text.push('\n');
                }
            }
            // Add space if horizontal gap is significant (word boundary)
            // Use character width as reference: gap > 50% of char width = word boundary
            else if prev_x_end > 0.0
                && gap_x > (cell_width * 0.3)
                && !text.ends_with(' ')
                && !text.ends_with('\n')
            {
                text.push(' ');
            }

            text.push_str(&cell.text);
            prev_x_end = cell.bbox.r;
            prev_y = cell.bbox.t;
        }

        // Sanitize if enabled
        if self.options.enable_text_sanitization {
            self.sanitizer.sanitize(&text)
        } else {
            text
        }
    }

    /// Process title cluster
    fn process_title(&self, cluster: &Cluster) -> Result<Vec<DocItem>> {
        let text = self.extract_text(cluster);

        Ok(vec![DocItem::Title(TextItem {
            text,
            formatting: None,
            label: DocItemLabel::Title,
        })])
    }

    /// Process section header cluster
    fn process_section_header(&self, cluster: &Cluster) -> Result<Vec<DocItem>> {
        let text = self.extract_text(cluster);

        // Try to extract section number to determine level
        let level = if let Some(section_num) = extract_section_number(&text) {
            calculate_section_level(&section_num)
        } else {
            // Fallback heuristic based on font size or default to 2
            2
        };

        Ok(vec![DocItem::SectionHeader(SectionHeaderItem {
            text,
            level,
            formatting: None,
        })])
    }

    /// Process text/paragraph cluster
    fn process_text(&self, cluster: &Cluster) -> Result<Vec<DocItem>> {
        let text = self.extract_text(cluster);

        // Check if it's actually a heading (ML may misclassify)
        if self.options.enable_heading_detection && is_likely_heading(&text) {
            // Promote to section header
            let level = if let Some(section_num) = extract_section_number(&text) {
                calculate_section_level(&section_num)
            } else {
                2
            };

            Ok(vec![DocItem::SectionHeader(SectionHeaderItem {
                text,
                level,
                formatting: None,
            })])
        } else {
            Ok(vec![DocItem::Paragraph(TextItem {
                text,
                formatting: None,
                label: DocItemLabel::Paragraph,
            })])
        }
    }

    /// Process list item cluster
    fn process_list_item(&self, cluster: &Cluster) -> Result<Vec<DocItem>> {
        let text = self.extract_text(cluster);

        // Detect marker and type
        let (marker, enumerated) = self.detect_list_marker(&text);

        // Remove marker from text
        let text_without_marker = if let Some(m) = &marker {
            text.trim_start_matches(m).trim_start().to_string()
        } else {
            text
        };

        Ok(vec![DocItem::ListItem(ListItemData {
            text: text_without_marker,
            marker: marker.unwrap_or_else(|| "-".to_string()),
            enumerated,
            level: 0, // TODO: Detect nesting level from indentation
        })])
    }

    /// Detect list marker (bullet or number)
    fn detect_list_marker(&self, text: &str) -> (Option<String>, bool) {
        let trimmed = text.trim_start();

        // Bullet markers
        if trimmed.starts_with("- ") || trimmed.starts_with("• ") || trimmed.starts_with("· ") {
            return (Some(trimmed.chars().next().unwrap().to_string()), false);
        }

        // Numbered markers (1., 2., 1), 2), etc.)
        if let Some(pos) = trimmed.find(|c| c == '.' || c == ')') {
            if pos > 0 && trimmed[..pos].chars().all(|c| c.is_numeric()) {
                let marker = &trimmed[..=pos];
                return (Some(marker.to_string()), true);
            }
        }

        (None, false)
    }

    /// Process caption cluster
    fn process_caption(&self, cluster: &Cluster) -> Result<Vec<DocItem>> {
        let text = self.extract_text(cluster);

        Ok(vec![DocItem::Paragraph(TextItem {
            text,
            formatting: Some(Formatting {
                italic: true,
                ..Default::default()
            }),
            label: DocItemLabel::Caption,
        })])
    }

    /// Process footnote cluster
    fn process_footnote(&self, cluster: &Cluster) -> Result<Vec<DocItem>> {
        let text = self.extract_text(cluster);

        Ok(vec![DocItem::Paragraph(TextItem {
            text,
            formatting: None,
            label: DocItemLabel::Footnote,
        })])
    }

    /// Process page header/footer cluster
    fn process_header_footer(&self, cluster: &Cluster) -> Result<Vec<DocItem>> {
        let text = self.extract_text(cluster);

        // Usually skip headers/footers as they're page metadata
        // But can be included if needed
        Ok(vec![DocItem::Paragraph(TextItem {
            text,
            formatting: None,
            label: cluster.label,
        })])
    }

    /// Process table cluster
    fn process_table(&self, cluster: &Cluster) -> Result<Vec<DocItem>> {
        // This is a placeholder - actual table structure comes from TableStructureModel
        // For now, create a simple table from cells

        let text = self.extract_text(cluster);

        // TODO: Use TableStructureModel output to build proper TableData
        // For now, create a minimal table
        let table_data = TableData {
            num_rows: 1,
            num_cols: 1,
            grid: vec![vec![TableCell {
                text,
                row_span: 1,
                col_span: 1,
            }]],
        };

        Ok(vec![DocItem::Table(TableItem {
            data: table_data,
            caption: None,
        })])
    }

    /// Process picture/figure cluster
    fn process_picture(&self, cluster: &Cluster) -> Result<Vec<DocItem>> {
        // Extract any text (OCR or caption)
        let text = if !cluster.cells.is_empty() {
            Some(self.extract_text(cluster))
        } else {
            None
        };

        Ok(vec![DocItem::Picture(PictureItem {
            caption: text,
            placeholder: format!(
                "<!-- Figure at ({}, {}) -->",
                cluster.bbox.l, cluster.bbox.t
            ),
        })])
    }

    /// Process code block cluster
    fn process_code(&self, cluster: &Cluster) -> Result<Vec<DocItem>> {
        let text = self.extract_text(cluster);

        // Try to detect language from first line
        let language = self.detect_code_language(&text);

        Ok(vec![DocItem::Code(CodeItem { text, language })])
    }

    /// Detect programming language from code text
    fn detect_code_language(&self, text: &str) -> Option<String> {
        // Simple heuristics - can be improved
        if text.contains("def ") || text.contains("import ") || text.contains("print(") {
            Some("python".to_string())
        } else if text.contains("function ") || text.contains("const ") || text.contains("let ") {
            Some("javascript".to_string())
        } else if text.contains("fn ") || text.contains("impl ") || text.contains("pub ") {
            Some("rust".to_string())
        } else {
            None
        }
    }

    /// Process formula cluster
    fn process_formula(&self, cluster: &Cluster) -> Result<Vec<DocItem>> {
        let text = self.extract_text(cluster);

        // Detect if inline or block formula based on length/position
        let is_inline = text.len() < 50;

        Ok(vec![DocItem::Formula(FormulaItem { text, is_inline })])
    }

    /// Process checkbox cluster
    fn process_checkbox(&self, cluster: &Cluster) -> Result<Vec<DocItem>> {
        let text = self.extract_text(cluster);
        let checked = cluster.label == DocItemLabel::CheckboxSelected;

        let marker = if checked { "[x]" } else { "[ ]" };

        Ok(vec![DocItem::ListItem(ListItemData {
            text,
            marker: marker.to_string(),
            enumerated: false,
            level: 0,
        })])
    }

    /// Merge adjacent text items into paragraphs
    fn merge_adjacent_text_items(&self, items: Vec<DocItem>) -> Result<Vec<DocItem>> {
        if items.len() < 2 {
            return Ok(items);
        }

        let mut merged = Vec::new();
        let mut current_text: Option<String> = None;
        let mut current_label: Option<DocItemLabel> = None;

        for item in items {
            match item {
                DocItem::Paragraph(ref text_item) if text_item.label == DocItemLabel::Paragraph => {
                    // Accumulate text
                    if let Some(ref mut text) = current_text {
                        text.push(' ');
                        text.push_str(&text_item.text);
                    } else {
                        current_text = Some(text_item.text.clone());
                        current_label = Some(text_item.label);
                    }
                }
                _ => {
                    // Flush accumulated text
                    if let Some(text) = current_text.take() {
                        merged.push(DocItem::Paragraph(TextItem {
                            text,
                            formatting: None,
                            label: current_label.unwrap_or(DocItemLabel::Paragraph),
                        }));
                    }
                    merged.push(item);
                }
            }
        }

        // Flush remaining
        if let Some(text) = current_text {
            merged.push(DocItem::Paragraph(TextItem {
                text,
                formatting: None,
                label: current_label.unwrap_or(DocItemLabel::Paragraph),
            }));
        }

        Ok(merged)
    }
}

impl Default for PageAssembler {
    fn default() -> Self {
        Self::new(PageAssemblerOptions::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_list_marker() {
        let assembler = PageAssembler::default();

        let (marker, enumerated) = assembler.detect_list_marker("- Item");
        assert_eq!(marker, Some("-".to_string()));
        assert!(!enumerated);

        let (marker, enumerated) = assembler.detect_list_marker("1. First");
        assert_eq!(marker, Some("1.".to_string()));
        assert!(enumerated);

        let (_marker, enumerated) = assembler.detect_list_marker("• Bullet");
        assert!(!enumerated);
    }

    #[test]
    fn test_detect_code_language() {
        let assembler = PageAssembler::default();

        assert_eq!(
            assembler.detect_code_language("def main():\n    print('hello')"),
            Some("python".to_string())
        );

        assert_eq!(
            assembler.detect_code_language("function test() { const x = 1; }"),
            Some("javascript".to_string())
        );

        assert_eq!(
            assembler.detect_code_language("fn main() { println!(\"hello\"); }"),
            Some("rust".to_string())
        );
    }
}
