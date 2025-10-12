//! Image OCR converter implementation

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::types::{ConversionOptions, ConversionResult, FileFormat, OutputFormat};
use crate::Result;
use async_trait::async_trait;
use std::path::Path;

/// Image to Markdown converter (via OCR)
pub struct ImageConverter;

impl ImageConverter {
    /// Create a new Image converter
    pub fn new() -> Self {
        Self
    }
}

impl Default for ImageConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentConverter for ImageConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![
            FileFormat::Jpeg,
            FileFormat::Png,
            FileFormat::Tiff,
            FileFormat::Bmp,
            FileFormat::Gif,
            FileFormat::Webp,
        ]
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
        unimplemented!("Image OCR conversion not yet implemented")
    }

    fn metadata(&self) -> ConverterMetadata {
        ConverterMetadata {
            name: "Image OCR Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Image to Markdown via OCR".to_string(),
            external_deps: vec!["tesseract".to_string(), "leptess".to_string()],
        }
    }
}

