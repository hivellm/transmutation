//! XML converter implementation
//!
//! Converts XML to JSON (structured) and Markdown (text content).
//! Uses quick-xml for fast, memory-efficient parsing.

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::types::{ConversionOptions, ConversionResult, ConversionOutput, FileFormat, OutputFormat, OutputMetadata};
use crate::Result;
use async_trait::async_trait;
use std::path::Path;
use tokio::fs;

/// XML to Markdown/JSON converter
pub struct XmlConverter;

impl XmlConverter {
    /// Create a new XML converter
    pub fn new() -> Self {
        Self
    }
    
    /// Convert XML to JSON using quick-xml
    fn xml_to_json(&self, xml: &str) -> Result<String> {
        use quick_xml::de::from_str;
        use serde_json::Value;
        
        // Parse XML to generic Value
        let value: Value = from_str(xml).map_err(|e| {
            crate::TransmutationError::engine_error("xml-parser", format!("Failed to parse XML: {}", e))
        })?;
        
        // Convert to pretty JSON
        Ok(serde_json::to_string_pretty(&value)?)
    }
    
    /// Convert XML to Markdown (extract text content)
    fn xml_to_markdown(&self, xml: &str) -> Result<String> {
        use quick_xml::Reader;
        use quick_xml::events::Event;
        
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);
        
        let mut markdown = String::new();
        markdown.push_str("# XML Document\n\n");
        
        let mut current_element = String::new();
        let mut text_parts = Vec::new();
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    current_element = String::from_utf8_lossy(e.name().as_ref()).to_string();
                }
                Ok(Event::Text(e)) => {
                    if let Ok(text) = e.unescape() {
                        let content = text.trim();
                        if !content.is_empty() && !current_element.is_empty() {
                            text_parts.push(format!("**{}**: {}", current_element, content));
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(crate::TransmutationError::engine_error(
                        "xml-parser",
                        format!("XML parse error: {}", e)
                    ));
                }
                _ => {}
            }
            buf.clear();
        }
        
        markdown.push_str(&text_parts.join("\n\n"));
        markdown.push_str("\n");
        
        Ok(markdown)
    }
}

impl Default for XmlConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentConverter for XmlConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![FileFormat::Xml]
    }

    fn output_formats(&self) -> Vec<OutputFormat> {
        vec![
            OutputFormat::Markdown {
                split_pages: false,
                optimize_for_llm: true,
            },
            OutputFormat::Json {
                structured: true,
                include_metadata: false,
            },
        ]
    }

    async fn convert(
        &self,
        input: &Path,
        output_format: OutputFormat,
        _options: ConversionOptions,
    ) -> Result<ConversionResult> {
        eprintln!("🔄 XML Conversion (Pure Rust)");
        eprintln!("   XML → Parsing → {:?}", output_format);
        eprintln!();
        
        // Read XML file
        let xml_content = fs::read_to_string(input).await?;
        
        // Convert to requested format
        let output_data = match output_format {
            OutputFormat::Markdown { .. } => {
                eprintln!("📝 Converting to Markdown...");
                let markdown = self.xml_to_markdown(&xml_content)?;
                markdown.into_bytes()
            },
            OutputFormat::Json { .. } => {
                eprintln!("📝 Converting to JSON...");
                let json = self.xml_to_json(&xml_content)?;
                json.into_bytes()
            },
            _ => {
                return Err(crate::TransmutationError::UnsupportedFormat(
                    format!("Output format {:?} not supported for XML", output_format)
                ));
            }
        };
        
        let output_size = output_data.len() as u64;
        let input_size = fs::metadata(input).await?.len();
        
        eprintln!("✅ XML conversion complete!");
        
        Ok(ConversionResult {
            input_path: input.to_path_buf(),
            input_format: FileFormat::Xml,
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
            name: "XML Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "XML to Markdown/JSON converter (pure Rust)".to_string(),
            external_deps: vec![],
        }
    }
}


