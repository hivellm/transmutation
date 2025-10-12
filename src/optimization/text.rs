//! Text optimization and cleanup
//!
//! This module provides text processing utilities for cleaning and optimizing
//! extracted text for LLM consumption.

use regex::Regex;

/// Text optimizer for cleaning and normalizing extracted text
pub struct TextOptimizer {
    remove_headers_footers: bool,
    normalize_whitespace: bool,
    remove_hyphenation: bool,
}

impl TextOptimizer {
    /// Create a new text optimizer
    pub fn new() -> Self {
        Self {
            remove_headers_footers: true,
            normalize_whitespace: true,
            remove_hyphenation: true,
        }
    }

    /// Configure whether to remove headers/footers
    pub fn with_header_footer_removal(mut self, enable: bool) -> Self {
        self.remove_headers_footers = enable;
        self
    }

    /// Configure whitespace normalization
    pub fn with_whitespace_normalization(mut self, enable: bool) -> Self {
        self.normalize_whitespace = enable;
        self
    }

    /// Configure hyphenation removal
    pub fn with_hyphenation_removal(mut self, enable: bool) -> Self {
        self.remove_hyphenation = enable;
        self
    }

    /// Optimize text for LLM processing
    pub fn optimize(&self, text: &str) -> String {
        let mut result = text.to_string();

        // Step 1: Remove hyphenation at line breaks
        if self.remove_hyphenation {
            result = self.remove_hyphenation_impl(&result);
        }

        // Step 2: Normalize whitespace
        if self.normalize_whitespace {
            result = self.normalize_whitespace_impl(&result);
        }

        // Step 3: Remove headers/footers (heuristic-based)
        if self.remove_headers_footers {
            result = self.remove_headers_footers_impl(&result);
        }

        // Step 4: Smart paragraph detection
        result = self.detect_paragraphs(&result);

        result
    }

    /// Remove hyphenation at end of lines
    fn remove_hyphenation_impl(&self, text: &str) -> String {
        // Pattern: word- followed by newline and continuation
        let re = Regex::new(r"(\w+)-\s*\n\s*(\w+)").unwrap();
        re.replace_all(text, "$1$2").to_string()
    }

    /// Normalize whitespace
    fn normalize_whitespace_impl(&self, text: &str) -> String {
        let mut result = text.to_string();

        // Replace multiple spaces with single space (but not at line start)
        let re = Regex::new(r"([^\n]) {2,}").unwrap();
        result = re.replace_all(&result, "$1 ").to_string();

        // Replace tabs with spaces
        result = result.replace('\t', "    ");

        // Normalize line endings
        result = result.replace("\r\n", "\n");
        result = result.replace('\r', "\n");

        result
    }

    /// Remove headers and footers (heuristic)
    fn remove_headers_footers_impl(&self, text: &str) -> String {
        let lines: Vec<&str> = text.lines().collect();
        
        if lines.len() < 10 {
            return text.to_string(); // Too short to have headers/footers
        }

        // Detect repeated patterns at top/bottom
        // This is a simple heuristic - can be improved
        let mut filtered_lines = Vec::new();
        
        for line in lines {
            // Skip likely page numbers (just digits)
            if line.trim().chars().all(|c| c.is_ascii_digit()) {
                continue;
            }
            
            // Skip very short lines at boundaries (likely headers/footers)
            if line.trim().len() < 5 {
                continue;
            }
            
            filtered_lines.push(line);
        }

        filtered_lines.join("\n")
    }

    /// Detect and format paragraphs
    fn detect_paragraphs(&self, text: &str) -> String {
        let lines: Vec<&str> = text.lines().collect();
        let mut result = Vec::new();
        let mut current_paragraph = Vec::new();

        for line in lines {
            let trimmed = line.trim();
            
            if trimmed.is_empty() {
                // Empty line - end current paragraph
                if !current_paragraph.is_empty() {
                    result.push(current_paragraph.join(" "));
                    result.push(String::new()); // Add blank line
                    current_paragraph.clear();
                }
            } else {
                // Check if this line starts a new paragraph (indentation, etc.)
                if Self::is_paragraph_start(trimmed) && !current_paragraph.is_empty() {
                    result.push(current_paragraph.join(" "));
                    current_paragraph.clear();
                }
                
                current_paragraph.push(trimmed.to_string());
            }
        }

        // Add final paragraph
        if !current_paragraph.is_empty() {
            result.push(current_paragraph.join(" "));
        }

        result.join("\n")
    }

    /// Check if a line starts a new paragraph
    fn is_paragraph_start(line: &str) -> bool {
        // Starts with capital letter after period
        // Or starts with list marker
        // Or is a heading
        line.starts_with("• ")
            || line.starts_with("- ")
            || line.starts_with("* ")
            || line.starts_with(|c: char| c.is_ascii_digit() && line.contains(". "))
            || line.starts_with('#')
    }
}

impl Default for TextOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Remove excessive whitespace from text
pub fn remove_excessive_whitespace(text: &str) -> String {
    let re = Regex::new(r"\s{3,}").unwrap();
    re.replace_all(text, "  ").to_string()
}

/// Normalize line breaks
pub fn normalize_line_breaks(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

/// Remove page numbers
pub fn remove_page_numbers(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    lines
        .into_iter()
        .filter(|line| {
            let trimmed = line.trim();
            // Skip lines that are just numbers
            !trimmed.chars().all(|c| c.is_ascii_digit())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyphenation_removal() {
        let optimizer = TextOptimizer::new();
        let text = "This is a long-\nword that continues.";
        let result = optimizer.remove_hyphenation_impl(text);
        assert_eq!(result, "This is a longword that continues.");
    }

    #[test]
    fn test_whitespace_normalization() {
        let optimizer = TextOptimizer::new();
        let text = "Hello    world\t\ttab";
        let result = optimizer.normalize_whitespace_impl(text);
        assert!(!result.contains("    "));
        assert!(!result.contains('\t'));
    }

    #[test]
    fn test_remove_excessive_whitespace() {
        let text = "Hello     world";
        let result = remove_excessive_whitespace(text);
        assert_eq!(result, "Hello  world");
    }

    #[test]
    fn test_normalize_line_breaks() {
        let text = "Line 1\r\nLine 2\rLine 3\nLine 4";
        let result = normalize_line_breaks(text);
        assert_eq!(result, "Line 1\nLine 2\nLine 3\nLine 4");
    }

    #[test]
    fn test_full_optimization() {
        let optimizer = TextOptimizer::new();
        let text = "This  is  a  test-\ntext  with\n\n\n\nmultiple  issues.";
        let result = optimizer.optimize(text);
        
        // Should have normalized whitespace
        assert!(!result.contains("  "));
        // Should have removed hyphenation
        assert!(!result.contains("-\n"));
    }
}

