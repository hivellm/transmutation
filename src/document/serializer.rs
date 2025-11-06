//! Markdown serializer for DoclingDocument
//! Reimplementation of docling-core's markdown serializer in Rust

#![allow(missing_docs)]

use once_cell::sync::Lazy;
use regex::Regex;

use super::types::*;
use crate::error::Result;

/// Pattern for detecting markdown special characters
static MD_ESCAPE_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"([\\`*_{}[\]()#+\-.!|])").unwrap());

/// Pattern for detecting URLs
static URL_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"https?://[^\s]+").unwrap());

pub struct MarkdownSerializer {
    indent: usize,
    escape_underscores: bool,
    escape_special_chars: bool,
    enable_tables: bool,
    enable_images: bool,
}

impl Default for MarkdownSerializer {
    fn default() -> Self {
        Self {
            indent: 4,
            escape_underscores: true,
            escape_special_chars: true,
            enable_tables: true,
            enable_images: true,
        }
    }
}

impl MarkdownSerializer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_indent(mut self, indent: usize) -> Self {
        self.indent = indent;
        self
    }

    pub fn with_escape_special_chars(mut self, enable: bool) -> Self {
        self.escape_special_chars = enable;
        self
    }

    pub fn with_tables(mut self, enable: bool) -> Self {
        self.enable_tables = enable;
        self
    }

    pub fn with_images(mut self, enable: bool) -> Self {
        self.enable_images = enable;
        self
    }

    pub fn serialize(&self, doc: &DoclingDocument) -> Result<String> {
        let mut parts = Vec::new();

        for item in &doc.items {
            if let Some(text) = self.serialize_item(item) {
                parts.push(text);
            }
        }

        let mut output = parts.join("\n\n");

        // Clean up excessive newlines
        while output.contains("\n\n\n") {
            output = output.replace("\n\n\n", "\n\n");
        }

        Ok(output.trim().to_string())
    }

    fn serialize_item(&self, item: &DocItem) -> Option<String> {
        match item {
            DocItem::Title(text_item) => Some(self.serialize_title(text_item)),
            DocItem::SectionHeader(header) => Some(self.serialize_section_header(header)),
            DocItem::Paragraph(text_item) => Some(self.serialize_paragraph(text_item)),
            DocItem::ListItem(list_item) => Some(self.serialize_list_item(list_item)),
            DocItem::Table(table) => Some(self.serialize_table(table)),
            DocItem::Picture(picture) => Some(self.serialize_picture(picture)),
            DocItem::Code(code) => Some(self.serialize_code(code)),
            DocItem::Formula(formula) => Some(self.serialize_formula(formula)),
        }
    }

    fn serialize_title(&self, item: &TextItem) -> String {
        let text = self.apply_formatting(&item.text, item.formatting.as_ref());
        format!("# {}", text)
    }

    fn serialize_section_header(&self, item: &SectionHeaderItem) -> String {
        let text = self.apply_formatting(&item.text, item.formatting.as_ref());
        let hashes = "#".repeat(item.level + 1);
        format!("{} {}", hashes, text)
    }

    fn serialize_paragraph(&self, item: &TextItem) -> String {
        let mut text = item.text.clone();

        // Handle checkboxes
        text = match item.label {
            DocItemLabel::CheckboxSelected => format!("- [x] {}", text),
            DocItemLabel::CheckboxUnselected => format!("- [ ] {}", text),
            _ => text,
        };

        self.apply_formatting(&text, item.formatting.as_ref())
    }

    fn serialize_list_item(&self, item: &ListItemData) -> String {
        let indent_str = " ".repeat(item.level * self.indent);
        let marker = if item.enumerated {
            "1.".to_string()
        } else {
            item.marker.clone()
        };

        format!("{}{} {}", indent_str, marker, item.text)
    }

    fn serialize_table(&self, table: &TableItem) -> String {
        let mut output = String::new();

        // Add caption if present
        if let Some(caption) = &table.caption {
            output.push_str(caption);
            output.push_str("\n\n");
        }

        // Serialize table using GitHub-flavored markdown
        if table.data.grid.is_empty() {
            return output;
        }

        // Header row
        let header = &table.data.grid[0];
        output.push('|');
        for cell in header {
            output.push(' ');
            output.push_str(&cell.text.replace('\n', " "));
            output.push_str(" |");
        }
        output.push('\n');

        // Separator row
        output.push('|');
        for _ in header {
            output.push_str(" --- |");
        }
        output.push('\n');

        // Data rows
        for row in &table.data.grid[1..] {
            output.push('|');
            for cell in row {
                output.push(' ');
                output.push_str(&cell.text.replace('\n', " "));
                output.push_str(" |");
            }
            output.push('\n');
        }

        output.trim_end().to_string()
    }

    fn serialize_picture(&self, picture: &PictureItem) -> String {
        let mut output = String::new();

        if let Some(caption) = &picture.caption {
            output.push_str(caption);
            output.push_str("\n\n");
        }

        output.push_str(&picture.placeholder);
        output
    }

    fn serialize_code(&self, code: &CodeItem) -> String {
        if let Some(lang) = &code.language {
            format!("```{}\n{}\n```", lang, code.text)
        } else {
            format!("```\n{}\n```", code.text)
        }
    }

    fn serialize_formula(&self, formula: &FormulaItem) -> String {
        if formula.is_inline {
            format!("${}$", formula.text)
        } else {
            format!("$${}$$", formula.text)
        }
    }

    fn apply_formatting(&self, text: &str, formatting: Option<&Formatting>) -> String {
        let mut result = self.escape_markdown_chars(text);

        // Apply formatting
        if let Some(fmt) = formatting {
            // Apply in order: bold, italic, strikethrough
            // For combined formatting: ***text*** = bold + italic
            if fmt.bold && fmt.italic {
                result = format!("***{}***", result);
            } else if fmt.bold {
                result = format!("**{}**", result);
            } else if fmt.italic {
                result = format!("*{}*", result);
            }

            if fmt.underline {
                // Markdown doesn't have native underline, use HTML
                result = format!("<u>{}</u>", result);
            }
        }

        result
    }

    // Note: Extended formatting (strikethrough, subscript, superscript) will be added later
    // when the Formatting struct is expanded to include these fields

    /// Escape markdown special characters
    fn escape_markdown_chars(&self, text: &str) -> String {
        if !self.escape_special_chars {
            return text.to_string();
        }

        // Don't escape inside URLs
        if URL_PATTERN.is_match(text) {
            return text.to_string();
        }

        // Don't escape if already in code block
        if text.starts_with('`') && text.ends_with('`') {
            return text.to_string();
        }

        // Escape special markdown characters
        let mut result = text.to_string();

        // Only escape underscores if not in links
        if self.escape_underscores && !text.contains("](") {
            result = result.replace('_', r"\_");
        }

        // Escape other special chars selectively
        result = result.replace('*', r"\*");
        result = result.replace('[', r"\[");
        result = result.replace(']', r"\]");

        result
    }

    /// Format text with hyperlink
    fn format_link(&self, text: &str, url: &str) -> String {
        format!("[{}]({})", text, url)
    }

    /// Format inline code
    fn format_inline_code(&self, text: &str) -> String {
        format!("`{}`", text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_title() {
        let serializer = MarkdownSerializer::new();
        let item = TextItem {
            text: "Test Title".to_string(),
            formatting: None,
            label: DocItemLabel::Title,
        };

        let result = serializer.serialize_title(&item);
        assert_eq!(result, "# Test Title");
    }

    #[test]
    fn test_serialize_section_header() {
        let serializer = MarkdownSerializer::new();
        let item = SectionHeaderItem {
            text: "Section".to_string(),
            level: 1,
            formatting: None,
        };

        let result = serializer.serialize_section_header(&item);
        assert_eq!(result, "## Section");
    }

    #[test]
    fn test_apply_formatting() {
        let serializer = MarkdownSerializer::new();

        let bold = Formatting {
            bold: true,
            italic: false,
            underline: false,
        };
        assert_eq!(serializer.apply_formatting("text", Some(&bold)), "**text**");

        let italic = Formatting {
            bold: false,
            italic: true,
            underline: false,
        };
        assert_eq!(serializer.apply_formatting("text", Some(&italic)), "*text*");

        let both = Formatting {
            bold: true,
            italic: true,
            underline: false,
        };
        assert_eq!(
            serializer.apply_formatting("text", Some(&both)),
            "*   *   **text****"
        );
    }
}
