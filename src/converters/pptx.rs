//! PPTX converter implementation

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::types::{ConversionOptions, ConversionResult, FileFormat, OutputFormat};
use crate::Result;
use async_trait::async_trait;
use std::path::Path;

/// PPTX to Markdown converter
pub struct PptxConverter;

impl PptxConverter {
    /// Create a new PPTX converter
    pub fn new() -> Self {
        Self
    }
}

impl Default for PptxConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentConverter for PptxConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![FileFormat::Pptx]
    }

    fn output_formats(&self) -> Vec<OutputFormat> {
        vec![OutputFormat::Markdown {
            split_pages: true,
            optimize_for_llm: true,
        }]
    }

    async fn convert(
        &self,
        _input: &Path,
        _output_format: OutputFormat,
        _options: ConversionOptions,
    ) -> Result<ConversionResult> {
        unimplemented!("PPTX conversion not yet implemented")
    }

    fn metadata(&self) -> ConverterMetadata {
        ConverterMetadata {
            name: "PPTX Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Pure Rust PPTX to Markdown converter".to_string(),
            external_deps: vec!["zip".to_string(), "quick-xml".to_string()],
        }
    }
}

