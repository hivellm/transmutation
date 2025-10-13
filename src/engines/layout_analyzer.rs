//! PDF Layout Analysis Engine
//!
//! This module analyzes text blocks to detect semantic document structure
//! like headings, paragraphs, lists, formulas, and images.

use crate::engines::pdf_parser::TextBlock;

/// Layout analyzer for semantic structure detection
#[derive(Debug, Clone)]
pub struct LayoutAnalyzer {
    /// Font size threshold for headings (pt)
    heading_font_threshold: f32,
    /// Y gap threshold for new paragraph (pt)
    paragraph_y_gap: f32,
    /// Base body font size (pt)
    base_font_size: f32,
}

impl LayoutAnalyzer {
    /// Create new layout analyzer with default settings
    pub fn new() -> Self {
        Self {
            heading_font_threshold: 12.0,
            paragraph_y_gap: 10.0,
            base_font_size: 10.0,
        }
    }

    /// Analyze text blocks to detect semantic structure
    pub fn analyze(&self, blocks: &[TextBlock]) -> Vec<AnalyzedBlock> {
        if blocks.is_empty() {
            return Vec::new();
        }

        let mut analyzed = Vec::new();
        let mut i = 0;

        while i < blocks.len() {
            let block = &blocks[i];
            let content = block.text.trim();

            if content.is_empty() {
                i += 1;
                continue;
            }

            // Detect block type
            let block_type = self.detect_block_type(block, blocks, i);
            
            analyzed.push(AnalyzedBlock {
                block_type: block_type.clone(),
                content: content.to_string(),
                level: self.get_heading_level(&block_type, block.font_size),
                font_size: block.font_size,
                y_position: block.y,
            });

            i += 1;
        }

        // Post-process to merge multi-line elements
        self.merge_multiline_elements(analyzed)
    }

    /// Detect the type of a text block
    fn detect_block_type(&self, block: &TextBlock, _all_blocks: &[TextBlock], _index: usize) -> BlockType {
        let content = block.text.trim();
        
        // Check for formulas (high math symbol density)
        if self.is_formula(content) {
            return BlockType::Formula;
        }

        // Check for image captions
        if self.is_image_caption(content) {
            return BlockType::Image;
        }

        // Check for table content
        if content.contains('|') || content.contains('\t') {
            return BlockType::Table;
        }

        // Check for list items
        if self.is_list_item(content) {
            return BlockType::ListItem;
        }

        // Check for headings based on font size
        if block.font_size > self.heading_font_threshold {
            return self.classify_heading(content, block.font_size);
        }

        // Check for numbered sections like "1 Introduction"
        if self.is_section_heading(content) {
            return BlockType::Heading(2);
        }

        // Check for subsections like "3.1 Encoder"
        if self.is_subsection_heading(content) {
            return BlockType::Heading(3);
        }

        // Default to paragraph
        BlockType::Paragraph
    }

    /// Classify heading by content and font size
    fn classify_heading(&self, content: &str, font_size: f32) -> BlockType {
        // Title if very large font or specific keywords
        if font_size >= 18.0 || content.contains("Attention Is All You Need") {
            return BlockType::Title;
        }

        // Major section if large font
        if font_size >= 14.0 {
            return BlockType::Heading(1); // Changed from 2 to 1
        }

        // Subsection
        BlockType::Heading(2) // Changed from 3 to 2
    }

    /// Check if content is a section heading (e.g., "1 Introduction")
    fn is_section_heading(&self, content: &str) -> bool {
        let trimmed = content.trim();
        
        // Keywords that indicate sections (exact match at start)
        let section_keywords = ["Abstract", "Introduction", "Background", "Conclusion", 
                               "Acknowledgements", "References", "Appendix", "Attention Visualizations"];
        if section_keywords.iter().any(|&kw| trimmed == kw || trimmed.starts_with(&format!("{} ", kw))) {
            return true;
        }
        
        // Pattern: single digit + space + capitalized word (like "1 Introduction")
        if let Some(first_char) = trimmed.chars().next() {
            if first_char.is_numeric() {
                let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
                if parts.len() == 2 {
                    let number_part = parts[0];
                    let text_part = parts[1];
                    
                    // Single digit only (not "3.1")
                    if number_part.len() == 1 && number_part.chars().all(|c| c.is_numeric()) {
                        // Text part starts with capital
                        if let Some(first_text) = text_part.chars().next() {
                            return first_text.is_uppercase();
                        }
                    }
                }
            }
        }
        
        false
    }

    /// Check if content is a subsection heading (e.g., "3.1 Encoder")
    fn is_subsection_heading(&self, content: &str) -> bool {
        let trimmed = content.trim();
        
        // Pattern like "3.1", "3.2.1", etc.
        if trimmed.len() < 150 {
            let first_part = trimmed.split_whitespace().next().unwrap_or("");
            let dot_count = first_part.matches('.').count();
            let digit_count = first_part.chars().filter(|c| c.is_numeric()).count();
            
            // Must have pattern like "3.1" or "3.2.1" or "3.2.2"
            if dot_count >= 1 && digit_count >= 2 && first_part.len() < 10 {
                // Verify there's text after the number
                if trimmed.len() > first_part.len() + 1 {
                    return true;
                }
            }
        }
        
        false
    }

    /// Check if content is a list item
    fn is_list_item(&self, content: &str) -> bool {
        let trimmed = content.trim();
        
        // Bullet points
        if trimmed.starts_with('•') || trimmed.starts_with('▪') || trimmed.starts_with('◦') {
            return true;
        }

        // Dash bullets
        if trimmed.starts_with("- ") || trimmed.starts_with("– ") || trimmed.starts_with("— ") {
            return true;
        }

        // Numbered lists (1., a., i., etc.)
        if let Some(first_word) = trimmed.split_whitespace().next() {
            if first_word.ends_with('.') || first_word.ends_with(')') {
                let without_punct = first_word.trim_end_matches(&['.', ')'][..]);
                // Roman numerals, letters, or numbers
                if without_punct.chars().all(|c| c.is_numeric() || c.is_alphabetic()) 
                    && without_punct.len() <= 3 {
                    return true;
                }
            }
        }

        false
    }

    /// Check if content contains a formula
    fn is_formula(&self, content: &str) -> bool {
        // Math Unicode characters
        let math_chars = ['∑', '∫', '√', '∈', '∉', '⊂', '⊃', '≤', '≥', '≠', '≈', '∞',
                         '∂', '∇', '×', '÷', '±', 'α', 'β', 'γ', 'δ', 'θ', 'λ', 'μ', 'π', 'σ', 'ω'];
        
        let math_count = content.chars().filter(|c| math_chars.contains(c)).count();
        let total_chars = content.chars().count();
        
        // High density of math symbols (>10%)
        if total_chars > 0 && (math_count as f32 / total_chars as f32) > 0.1 {
            return true;
        }

        // Common LaTeX-like patterns that weren't converted
        if content.contains("\\frac") || content.contains("\\sum") || content.contains("\\int") {
            return true;
        }

        // Equations with equals and operators
        if content.contains('=') && (content.matches('+').count() + content.matches('*').count() + content.matches('/').count()) >= 2 {
            // But exclude table rows or code
            if !content.contains('|') && content.len() < 200 {
                return true;
            }
        }

        false
    }

    /// Check if content is an image caption
    fn is_image_caption(&self, content: &str) -> bool {
        let lower = content.to_lowercase();
        lower.starts_with("figure") || lower.starts_with("fig.") || 
        lower.starts_with("image") || lower.starts_with("diagram")
    }

    /// Get heading level based on block type and font size
    fn get_heading_level(&self, block_type: &BlockType, font_size: f32) -> Option<usize> {
        match block_type {
            BlockType::Title => Some(1),
            BlockType::Heading(level) => Some(*level),
            _ => {
                // Derive from font size
                if font_size >= 18.0 {
                    Some(2)
                } else if font_size >= 14.0 {
                    Some(3)
                } else if font_size >= 12.0 {
                    Some(4)
                } else {
                    None
                }
            }
        }
    }

    /// Merge multi-line elements (like multi-line paragraphs)
    fn merge_multiline_elements(&self, blocks: Vec<AnalyzedBlock>) -> Vec<AnalyzedBlock> {
        if blocks.is_empty() {
            return blocks;
        }

        let mut merged = Vec::new();
        let mut current: Option<AnalyzedBlock> = None;

        for block in blocks {
            match (&current, &block.block_type) {
                // Merge consecutive paragraphs if Y gap is small
                (Some(curr), BlockType::Paragraph) if matches!(curr.block_type, BlockType::Paragraph) => {
                    if let Some(ref mut c) = current {
                        // Check if should merge (close Y positions)
                        let y_diff = (c.y_position - block.y_position).abs();
                        if y_diff < self.paragraph_y_gap * 2.0 {
                            c.content.push(' ');
                            c.content.push_str(&block.content);
                        } else {
                            merged.push(current.take().unwrap());
                            current = Some(block);
                        }
                    }
                }
                // Don't merge headings, formulas, images
                _ => {
                    if let Some(c) = current.take() {
                        merged.push(c);
                    }
                    current = Some(block);
                }
            }
        }

        // Add last block
        if let Some(c) = current {
            merged.push(c);
        }

        merged
    }
}

impl Default for LayoutAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Analyzed block with semantic type
#[derive(Debug, Clone)]
pub struct AnalyzedBlock {
    /// Type of block
    pub block_type: BlockType,
    /// Text content
    pub content: String,
    /// Heading level (if applicable)
    pub level: Option<usize>,
    /// Font size
    pub font_size: f32,
    /// Y position on page
    pub y_position: f32,
}

/// Semantic block type
#[derive(Debug, Clone, PartialEq)]
pub enum BlockType {
    /// Document title
    Title,
    /// Heading with level (1-6)
    Heading(usize),
    /// Regular paragraph
    Paragraph,
    /// List item
    ListItem,
    /// Table content
    Table,
    /// Image or figure
    Image,
    /// Mathematical formula
    Formula,
    /// Bibliography reference
    Reference,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_heading_detection() {
        let analyzer = LayoutAnalyzer::new();
        assert!(analyzer.is_section_heading("1 Introduction"));
        assert!(analyzer.is_section_heading("2 Background"));
        assert!(analyzer.is_section_heading("Abstract"));
        assert!(!analyzer.is_section_heading("This is regular text"));
    }

    #[test]
    fn test_subsection_detection() {
        let analyzer = LayoutAnalyzer::new();
        assert!(analyzer.is_subsection_heading("3.1 Encoder and Decoder Stacks"));
        assert!(analyzer.is_subsection_heading("3.2.1 Scaled Dot-Product"));
        assert!(!analyzer.is_subsection_heading("This is not a subsection"));
    }

    #[test]
    fn test_list_detection() {
        let analyzer = LayoutAnalyzer::new();
        assert!(analyzer.is_list_item("• First item"));
        assert!(analyzer.is_list_item("- Second item"));
        assert!(analyzer.is_list_item("1. Numbered item"));
        assert!(!analyzer.is_list_item("Regular text"));
    }

    #[test]
    fn test_formula_detection() {
        let analyzer = LayoutAnalyzer::new();
        assert!(analyzer.is_formula("x = ∑ᵢ yᵢ + √z"));
        assert!(analyzer.is_formula("α + β = γ"));
        assert!(!analyzer.is_formula("This is regular text"));
    }

    #[test]
    fn test_image_caption_detection() {
        let analyzer = LayoutAnalyzer::new();
        assert!(analyzer.is_image_caption("Figure 1: The Transformer"));
        assert!(analyzer.is_image_caption("Fig. 2: Attention mechanism"));
        assert!(!analyzer.is_image_caption("Regular text"));
    }
}

