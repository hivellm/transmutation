//! DOCX converter implementation

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::types::{ConversionOptions, ConversionResult, FileFormat, OutputFormat};
use crate::Result;
use async_trait::async_trait;
use std::path::Path;

/// DOCX to Markdown converter
pub struct DocxConverter;

impl DocxConverter {
    /// Create a new DOCX converter
    pub fn new() -> Self {
        Self
    }
}

impl Default for DocxConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentConverter for DocxConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![FileFormat::Docx]
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
        unimplemented!("DOCX conversion not yet implemented")
    }

    fn metadata(&self) -> ConverterMetadata {
        ConverterMetadata {
            name: "DOCX Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Pure Rust DOCX to Markdown converter".to_string(),
            external_deps: vec!["docx-rs".to_string()],
        }
    }
}

