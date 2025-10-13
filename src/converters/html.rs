//! HTML converter implementation

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::types::{ConversionOptions, ConversionResult, FileFormat, OutputFormat};
use crate::Result;
use async_trait::async_trait;
use std::path::Path;

/// HTML to Markdown converter
pub struct HtmlConverter;

impl HtmlConverter {
    /// Create a new HTML converter
    pub fn new() -> Self {
        Self
    }
}

impl Default for HtmlConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentConverter for HtmlConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![FileFormat::Html]
    }

    fn output_formats(&self) -> Vec<OutputFormat> {
        vec![OutputFormat::Markdown {
            split_pages: false,
            optimize_for_llm: true,
        }]
    }

    async fn convert(
        &self,
        _input: &Path,
        _output_format: OutputFormat,
        _options: ConversionOptions,
    ) -> Result<ConversionResult> {
        unimplemented!("HTML conversion not yet implemented")
    }

    fn metadata(&self) -> ConverterMetadata {
        ConverterMetadata {
            name: "HTML Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Pure Rust HTML to Markdown converter".to_string(),
            external_deps: vec!["scraper".to_string(), "html5ever".to_string()],
        }
    }
}

