//! PDF converter implementation

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::types::{ConversionOptions, ConversionResult, FileFormat, OutputFormat};
use crate::Result;
use async_trait::async_trait;
use std::path::Path;

/// PDF to Markdown/Image converter
pub struct PdfConverter;

impl PdfConverter {
    /// Create a new PDF converter
    pub fn new() -> Self {
        Self
    }
}

impl Default for PdfConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentConverter for PdfConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![FileFormat::Pdf]
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
        _input: &Path,
        _output_format: OutputFormat,
        _options: ConversionOptions,
    ) -> Result<ConversionResult> {
        // TODO: Implement PDF conversion
        unimplemented!("PDF conversion not yet implemented")
    }

    fn metadata(&self) -> ConverterMetadata {
        ConverterMetadata {
            name: "PDF Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Pure Rust PDF to Markdown/JSON converter".to_string(),
            external_deps: vec!["lopdf".to_string()],
        }
    }
}

