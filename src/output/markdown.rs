//! Markdown generation and formatting
//!
//! This module converts extracted text to clean, LLM-optimized Markdown format.

use crate::types::ConversionOptions;

/// Markdown generator
pub struct MarkdownGenerator {
    options: ConversionOptions,
    buffer: String,
}

impl MarkdownGenerator {
    /// Create a new Markdown generator
    pub fn new(options: ConversionOptions) -> Self {
        Self {
            options,
            buffer: String::new(),
        }
    }

    /// Generate Markdown from plain text
    pub fn from_text(text: &str, options: ConversionOptions) -> String {
        let mut generator = Self::new(options);
        generator.add_text(text);
        
        if generator.options.optimize_for_llm {
            generator.optimize_for_llm();
        }
        
        generator.buffer
    }

    /// Generate Markdown from multiple pages
    pub fn from_pages(pages: &[(usize, String)], options: ConversionOptions) -> Vec<String> {
        if options.split_pages {
            // Generate separate Markdown for each page
            pages
                .iter()
                .map(|(page_num, text)| {
                    let mut generator = Self::new(options.clone());
                    generator.add_heading(1, &format!("Page {}", page_num + 1));
                    generator.add_text(text);
                    
                    if options.optimize_for_llm {
                        generator.optimize_for_llm();
                    }
                    
                    generator.buffer
                })
                .collect()
        } else {
            // Combined document with page breaks
            let mut generator = Self::new(options.clone());
            
            for (i, (page_num, text)) in pages.iter().enumerate() {
                if i > 0 {
                    generator.add_page_break();
                }
                generator.add_heading(2, &format!("Page {}", page_num + 1));
                generator.add_text(text);
            }
            
            if generator.options.optimize_for_llm {
                generator.optimize_for_llm();
            }
            
            vec![generator.buffer]
        }
    }

    /// Add plain text to buffer
    pub fn add_text(&mut self, text: &str) {
        self.buffer.push_str(text);
        self.buffer.push('\n');
    }

    /// Add a heading
    pub fn add_heading(&mut self, level: usize, text: &str) {
        let level = level.min(6); // Markdown supports up to 6 levels
        self.buffer.push_str(&"#".repeat(level));
        self.buffer.push(' ');
        self.buffer.push_str(text);
        self.buffer.push_str("\n\n");
    }

    /// Add a table
    pub fn add_table(&mut self, rows: &[Vec<String>]) {
        if rows.is_empty() {
            return;
        }

        // Header row
        let header = &rows[0];
        self.buffer.push('|');
        for cell in header {
            self.buffer.push(' ');
            self.buffer.push_str(cell);
            self.buffer.push_str(" |");
        }
        self.buffer.push('\n');

        // Separator
        self.buffer.push('|');
        for _ in header {
            self.buffer.push_str(" --- |");
        }
        self.buffer.push('\n');

        // Data rows
        for row in &rows[1..] {
            self.buffer.push('|');
            for (i, cell) in row.iter().enumerate() {
                self.buffer.push(' ');
                if i < header.len() {
                    self.buffer.push_str(cell);
                }
                self.buffer.push_str(" |");
            }
            self.buffer.push('\n');
        }

        self.buffer.push('\n');
    }

    /// Add a code block
    pub fn add_code_block(&mut self, code: &str, language: Option<&str>) {
        self.buffer.push_str("```");
        if let Some(lang) = language {
            self.buffer.push_str(lang);
        }
        self.buffer.push('\n');
        self.buffer.push_str(code);
        self.buffer.push_str("\n```\n\n");
    }

    /// Add a page break (for combined documents)
    fn add_page_break(&mut self) {
        self.buffer.push_str("\n\n---\n\n");
    }

    /// Optimize Markdown for LLM processing
    fn optimize_for_llm(&mut self) {
        // Normalize whitespace
        if self.options.normalize_whitespace {
            self.normalize_whitespace();
        }

        // Remove excessive newlines (more than 2 consecutive)
        let re = regex::Regex::new(r"\n{3,}").unwrap();
        self.buffer = re.replace_all(&self.buffer, "\n\n").to_string();

        // Trim trailing whitespace from each line
        self.buffer = self
            .buffer
            .lines()
            .map(|line| line.trim_end())
            .collect::<Vec<_>>()
            .join("\n");

        // Ensure document ends with single newline
        self.buffer = self.buffer.trim_end().to_string();
        self.buffer.push('\n');
    }

    /// Normalize whitespace
    fn normalize_whitespace(&mut self) {
        // Replace multiple spaces with single space
        let re = regex::Regex::new(r" {2,}").unwrap();
        self.buffer = re.replace_all(&self.buffer, " ").to_string();

        // Replace tabs with spaces
        self.buffer = self.buffer.replace('\t', "    ");
    }

    /// Get the generated Markdown
    pub fn into_string(self) -> String {
        self.buffer
    }
}

impl Default for MarkdownGenerator {
    fn default() -> Self {
        Self::new(ConversionOptions::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading_generation() {
        let mut md = MarkdownGenerator::default();
        md.add_heading(1, "Title");
        assert_eq!(md.buffer, "# Title\n\n");
    }

    #[test]
    fn test_table_generation() {
        let mut md = MarkdownGenerator::default();
        let table = vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ];
        md.add_table(&table);
        
        assert!(md.buffer.contains("| Name |"));
        assert!(md.buffer.contains("| --- |"));
        assert!(md.buffer.contains("| Alice |"));
    }

    #[test]
    fn test_code_block() {
        let mut md = MarkdownGenerator::default();
        md.add_code_block("fn main() {}", Some("rust"));
        assert!(md.buffer.contains("```rust"));
        assert!(md.buffer.contains("fn main() {}"));
    }

    #[test]
    fn test_whitespace_normalization() {
        let opts = ConversionOptions {
            normalize_whitespace: true,
            optimize_for_llm: true,
            ..Default::default()
        };
        
        let text = "Hello    world\n\n\n\nTest";
        let result = MarkdownGenerator::from_text(text, opts);
        
        assert!(!result.contains("    ")); // No multiple spaces
        assert!(!result.contains("\n\n\n")); // No triple newlines
    }

    #[test]
    fn test_from_pages_split() {
        let pages = vec![
            (0, "Page 1 content".to_string()),
            (1, "Page 2 content".to_string()),
        ];
        
        let opts = ConversionOptions {
            split_pages: true,
            ..Default::default()
        };
        
        let result = MarkdownGenerator::from_pages(&pages, opts);
        assert_eq!(result.len(), 2);
        assert!(result[0].contains("Page 1"));
        assert!(result[1].contains("Page 2"));
    }

    #[test]
    fn test_from_pages_combined() {
        let pages = vec![
            (0, "Page 1 content".to_string()),
            (1, "Page 2 content".to_string()),
        ];
        
        let opts = ConversionOptions {
            split_pages: false,
            ..Default::default()
        };
        
        let result = MarkdownGenerator::from_pages(&pages, opts);
        assert_eq!(result.len(), 1);
        assert!(result[0].contains("Page 1"));
        assert!(result[0].contains("Page 2"));
        assert!(result[0].contains("---")); // Page break
    }
}

