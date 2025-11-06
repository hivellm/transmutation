//! ODT (OpenDocument Text) converter implementation
//!
//! Converts ODT files to Markdown by extracting content from the content.xml file.

#![allow(clippy::unused_self, clippy::uninlined_format_args, clippy::manual_flatten)]

use std::io::{Cursor, Read};
use std::path::Path;

use async_trait::async_trait;
use quick_xml::Reader;
use quick_xml::events::Event;
use tokio::fs;
use zip::ZipArchive;

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::Result;
use crate::types::{
    ConversionOptions, ConversionOutput, ConversionResult, FileFormat, OutputFormat, OutputMetadata,
};

/// ODT to Markdown converter
#[derive(Debug)]
pub struct OdtConverter;

impl OdtConverter {
    /// Create a new ODT converter
    pub fn new() -> Self {
        Self
    }

    /// Extract text from ODT content.xml
    fn extract_text_from_xml(&self, xml: &str) -> String {
        let mut markdown = String::new();
        markdown.push_str("# Document\n\n");

        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        let mut in_paragraph = false;
        let mut in_heading = false;
        let mut heading_level = 1;
        let mut current_text = String::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                    if name == "text:p" {
                        in_paragraph = true;
                        current_text.clear();
                    } else if name == "text:h" {
                        in_heading = true;
                        // Try to get outline-level attribute
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if String::from_utf8_lossy(attr.key.as_ref())
                                    == "text:outline-level"
                                {
                                    if let Ok(value) = String::from_utf8(attr.value.into_owned()) {
                                        heading_level = value.parse().unwrap_or(1);
                                    }
                                }
                            }
                        }
                        current_text.clear();
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_paragraph || in_heading {
                        current_text.push_str(&e.unescape().unwrap_or_default());
                    }
                }
                Ok(Event::End(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                    if name == "text:p" {
                        in_paragraph = false;
                        if !current_text.trim().is_empty() {
                            markdown.push_str(&format!("{}\n\n", current_text.trim()));
                        }
                    } else if name == "text:h" {
                        in_heading = false;
                        if !current_text.trim().is_empty() {
                            let hashes = "#".repeat(heading_level.min(6));
                            markdown.push_str(&format!("{} {}\n\n", hashes, current_text.trim()));
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    eprintln!(
                        "Warning: XML parse error at position {}: {}",
                        reader.buffer_position(),
                        e
                    );
                    break;
                }
                _ => {}
            }
            buf.clear();
        }

        if markdown.trim() == "# Document" {
            markdown.push_str("*No text content found in ODT*\n");
        }

        markdown
    }

    /// Convert ODT to Markdown
    async fn odt_to_markdown(&self, odt_path: &Path) -> Result<String> {
        // Read ODT file (it's a ZIP)
        let data = fs::read(odt_path).await?;
        let cursor = Cursor::new(data);
        let mut archive = ZipArchive::new(cursor)?;

        // Extract content.xml
        let mut content_xml = String::new();
        match archive.by_name("content.xml") {
            Ok(mut file) => {
                file.read_to_string(&mut content_xml)?;
            }
            Err(_) => {
                return Ok("# Error\n\n*Could not find content.xml in ODT file*\n".to_string());
            }
        }

        Ok(self.extract_text_from_xml(&content_xml))
    }
}

impl Default for OdtConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentConverter for OdtConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![FileFormat::Odt]
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
        eprintln!("🔄 ODT Conversion (Pure Rust)");
        eprintln!("   ODT → ZIP → XML → {:?}", output_format);
        eprintln!();

        // Convert ODT to Markdown
        let markdown = self.odt_to_markdown(input).await?;

        // Convert to requested format
        let output_data = match output_format {
            OutputFormat::Markdown { .. } => {
                eprintln!("📝 Markdown extracted!");
                markdown.into_bytes()
            }
            OutputFormat::Json { .. } => {
                eprintln!("📝 Converting to JSON...");
                let json = serde_json::json!({
                    "text": {
                        "content": markdown,
                        "format": "odt",
                    }
                });
                serde_json::to_string_pretty(&json)?.into_bytes()
            }
            _ => {
                return Err(crate::TransmutationError::UnsupportedFormat(format!(
                    "Output format {:?} not supported for ODT",
                    output_format
                )));
            }
        };

        let output_size = output_data.len() as u64;
        let input_size = fs::metadata(input).await?.len();

        eprintln!("✅ ODT conversion complete!");

        Ok(ConversionResult {
            input_path: input.to_path_buf(),
            input_format: FileFormat::Odt,
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
            name: "ODT Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "ODT to Markdown converter (pure Rust, ZIP + XML parsing)".to_string(),
            external_deps: vec![],
        }
    }
}
