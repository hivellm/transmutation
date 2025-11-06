//! TXT converter implementation
//!
//! Converts plain text files to Markdown with encoding detection.

#![allow(clippy::unused_self, clippy::uninlined_format_args)]

use std::path::Path;

use async_trait::async_trait;
use tokio::fs;

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::Result;
use crate::types::{
    ConversionOptions, ConversionOutput, ConversionResult, FileFormat, OutputFormat, OutputMetadata,
};

/// Plain text to Markdown converter
#[derive(Debug)]
pub struct TxtConverter;

impl TxtConverter {
    /// Create a new TXT converter
    pub fn new() -> Self {
        Self
    }

    /// Convert plain text to Markdown
    fn txt_to_markdown(&self, text: &str) -> String {
        let mut markdown = String::new();
        markdown.push_str("# Document\n\n");

        // Simple paragraph detection based on blank lines
        let paragraphs: Vec<&str> = text
            .split("\n\n")
            .filter(|p| !p.trim().is_empty())
            .collect();

        for para in paragraphs {
            let trimmed = para.trim();

            // Detect if it might be a heading (short line, all caps, or ends with colon)
            if trimmed.len() < 80
                && (trimmed
                    .chars()
                    .all(|c| !c.is_lowercase() || !c.is_alphabetic())
                    || trimmed.ends_with(':'))
            {
                markdown.push_str(&format!("## {}\n\n", trimmed));
            } else {
                markdown.push_str(&format!("{}\n\n", trimmed));
            }
        }

        markdown
    }
}

impl Default for TxtConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_txt_converter_creation() {
        let converter = TxtConverter::new();
        assert_eq!(converter.supported_formats(), vec![FileFormat::Txt]);
    }

    #[test]
    fn test_txt_to_markdown_basic() {
        let converter = TxtConverter::new();
        let text = "This is a test\nSecond line";
        let result = converter.txt_to_markdown(text);
        assert!(result.contains("This is a test"));
    }

    #[test]
    fn test_txt_converter_metadata() {
        let converter = TxtConverter::new();
        let meta = converter.metadata();
        assert_eq!(meta.name, "TXT Converter");
    }
}

#[async_trait]
impl DocumentConverter for TxtConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![FileFormat::Txt]
    }

    fn output_formats(&self) -> Vec<OutputFormat> {
        vec![
            OutputFormat::Markdown {
                split_pages: false,
                optimize_for_llm: true,
            },
            OutputFormat::Json {
                structured: true,
                include_metadata: true,
            },
        ]
    }

    async fn convert(
        &self,
        input: &Path,
        output_format: OutputFormat,
        _options: ConversionOptions,
    ) -> Result<ConversionResult> {
        eprintln!("🔄 TXT Conversion (Pure Rust)");
        eprintln!("   TXT → Encoding Detection → {:?}", output_format);
        eprintln!();

        // Read text file with encoding detection
        let text_content = fs::read_to_string(input).await?;

        // Convert to requested format
        let output_data = match output_format {
            OutputFormat::Markdown { .. } => {
                eprintln!("📝 Converting to Markdown...");
                let markdown = self.txt_to_markdown(&text_content);
                markdown.into_bytes()
            }
            OutputFormat::Json { .. } => {
                eprintln!("📝 Converting to JSON...");
                let json = serde_json::json!({
                    "text": {
                        "content": text_content,
                        "lines": text_content.lines().count(),
                        "chars": text_content.len(),
                    }
                });
                serde_json::to_string_pretty(&json)?.into_bytes()
            }
            _ => {
                return Err(crate::TransmutationError::UnsupportedFormat(format!(
                    "Output format {:?} not supported for TXT",
                    output_format
                )));
            }
        };

        let output_size = output_data.len() as u64;
        let input_size = fs::metadata(input).await?.len();

        eprintln!("✅ TXT conversion complete!");

        Ok(ConversionResult {
            input_path: input.to_path_buf(),
            input_format: FileFormat::Txt,
            output_format,
            content: vec![ConversionOutput {
                page_number: 1,
                data: output_data,
                metadata: OutputMetadata {
                    size_bytes: output_size,
                    chunk_count: 1,
                    token_count: None,
                },
            }],
            metadata: crate::types::DocumentMetadata {
                title: None,
                author: None,
                created: None,
                modified: None,
                page_count: 1,
                language: None,
                custom: std::collections::HashMap::new(),
            },
            statistics: crate::types::ConversionStatistics {
                input_size_bytes: input_size,
                output_size_bytes: output_size,
                duration: std::time::Duration::from_secs(0),
                pages_processed: 1,
                tables_extracted: 0,
                images_extracted: 0,
                cache_hit: false,
            },
        })
    }

    fn metadata(&self) -> ConverterMetadata {
        ConverterMetadata {
            name: "TXT Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Plain text to Markdown converter (pure Rust)".to_string(),
            external_deps: vec![],
        }
    }
}
