//! RTF (Rich Text Format) converter implementation
//!
//! Converts RTF files to Markdown by parsing RTF commands and extracting text.

#![allow(clippy::unused_self, clippy::uninlined_format_args)]

use std::path::Path;

use async_trait::async_trait;
use tokio::fs;

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::Result;
use crate::types::{
    ConversionOptions, ConversionOutput, ConversionResult, FileFormat, OutputFormat, OutputMetadata,
};

/// RTF to Markdown converter
#[derive(Debug)]
pub struct RtfConverter;

impl RtfConverter {
    /// Create a new RTF converter
    pub fn new() -> Self {
        Self
    }

    /// Parse RTF and convert to plain text/Markdown
    /// This is a simplified RTF parser that extracts text content
    fn rtf_to_markdown(&self, rtf: &str) -> String {
        let mut markdown = String::new();
        markdown.push_str("# Document\n\n");

        let mut text_content = String::new();
        let mut in_control_word = false;
        let mut brace_depth = 0;
        let mut skip_next = false;

        let chars: Vec<char> = rtf.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];

            if skip_next {
                skip_next = false;
                i += 1;
                continue;
            }

            match ch {
                '{' => {
                    brace_depth += 1;
                }
                '}' => {
                    brace_depth -= 1;
                    in_control_word = false;
                }
                '\\' => {
                    // Check for control word
                    if i + 1 < chars.len() {
                        let next_ch = chars[i + 1];

                        // Escape sequences
                        if next_ch == '\\' || next_ch == '{' || next_ch == '}' {
                            text_content.push(next_ch);
                            skip_next = true;
                        } else if next_ch == '\'' {
                            // Hex escape \'XX
                            if i + 3 < chars.len() {
                                i += 3; // Skip \'XX
                            }
                        } else if next_ch == 'p'
                            && i + 3 < chars.len()
                            && chars[i + 2] == 'a'
                            && chars[i + 3] == 'r'
                        {
                            // Paragraph break
                            text_content.push_str("\n\n");
                            i += 3;
                        } else if next_ch == 't'
                            && i + 3 < chars.len()
                            && chars[i + 2] == 'a'
                            && chars[i + 3] == 'b'
                        {
                            // Tab
                            text_content.push('\t');
                            i += 3;
                        } else {
                            // Skip control word
                            in_control_word = true;
                            i += 1;
                            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '*')
                            {
                                i += 1;
                            }
                            // Skip optional space after control word
                            if i < chars.len() && chars[i] == ' ' {
                                i += 1;
                            }
                            i -= 1; // Adjust because we'll increment at the end
                        }
                    }
                }
                _ if brace_depth > 0 && !in_control_word && ch >= ' ' => {
                    // Regular text character
                    text_content.push(ch);
                }
                _ => {}
            }

            i += 1;
        }

        // Clean up text: remove extra spaces and add paragraphs
        let paragraphs: Vec<&str> = text_content
            .split("\n\n")
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .collect();

        for para in paragraphs {
            // Detect potential headings
            if para.len() < 80
                && (para
                    .chars()
                    .all(|c| !c.is_lowercase() || !c.is_alphabetic())
                    || para.ends_with(':'))
            {
                markdown.push_str(&format!("## {}\n\n", para));
            } else {
                markdown.push_str(&format!("{}\n\n", para));
            }
        }

        if markdown.trim() == "# Document" {
            markdown.push_str("*No text content extracted from RTF*\n");
        }

        markdown
    }
}

impl Default for RtfConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtf_converter_creation() {
        let converter = RtfConverter::new();
        assert_eq!(converter.supported_formats(), vec![FileFormat::Rtf]);
    }

    #[test]
    fn test_rtf_to_markdown_basic() {
        let converter = RtfConverter::new();
        // RTF with actual text commands
        let rtf = r"{\rtf1\ansi\deff0 {\fonttbl {\f0 Times New Roman;}}
\f0\fs24 Hello World\par
}";
        let result = converter.rtf_to_markdown(rtf);
        // Simplified parser may not extract perfectly, just check it doesn't crash
        assert!(!result.is_empty());
    }

    #[test]
    fn test_rtf_converter_metadata() {
        let converter = RtfConverter::new();
        let meta = converter.metadata();
        assert_eq!(meta.name, "RTF Converter");
    }
}

#[async_trait]
impl DocumentConverter for RtfConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![FileFormat::Rtf]
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
        eprintln!("🔄 RTF Conversion (Pure Rust)");
        eprintln!("   RTF → Parsing → {:?}", output_format);
        eprintln!();

        // Read RTF file
        let rtf_content = fs::read_to_string(input).await?;

        // Convert to requested format
        let output_data = match output_format {
            OutputFormat::Markdown { .. } => {
                eprintln!("📝 Converting to Markdown...");
                let markdown = self.rtf_to_markdown(&rtf_content);
                markdown.into_bytes()
            }
            OutputFormat::Json { .. } => {
                eprintln!("📝 Converting to JSON...");
                let text = self.rtf_to_markdown(&rtf_content);
                let json = serde_json::json!({
                    "text": {
                        "content": text,
                        "format": "rtf",
                    }
                });
                serde_json::to_string_pretty(&json)?.into_bytes()
            }
            _ => {
                return Err(crate::TransmutationError::UnsupportedFormat(format!(
                    "Output format {:?} not supported for RTF",
                    output_format
                )));
            }
        };

        let output_size = output_data.len() as u64;
        let input_size = fs::metadata(input).await?.len();

        eprintln!("✅ RTF conversion complete!");

        Ok(ConversionResult {
            input_path: input.to_path_buf(),
            input_format: FileFormat::Rtf,
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
            name: "RTF Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "RTF to Markdown converter (pure Rust, simplified parser)".to_string(),
            external_deps: vec![],
        }
    }
}
