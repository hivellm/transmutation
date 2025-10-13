//! XLSX converter implementation

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::types::{ConversionOptions, ConversionResult, FileFormat, OutputFormat};
use crate::Result;
use async_trait::async_trait;
use std::path::Path;

/// XLSX to Markdown/CSV converter
pub struct XlsxConverter;

impl XlsxConverter {
    /// Create a new XLSX converter
    pub fn new() -> Self {
        Self
    }
}

impl Default for XlsxConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentConverter for XlsxConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![FileFormat::Xlsx]
    }

    fn output_formats(&self) -> Vec<OutputFormat> {
        vec![
            OutputFormat::Markdown {
                split_pages: false,
                optimize_for_llm: true,
            },
            OutputFormat::Csv {
                delimiter: ',',
                include_headers: true,
            },
        ]
    }

    async fn convert(
        &self,
        _input: &Path,
        _output_format: OutputFormat,
        _options: ConversionOptions,
    ) -> Result<ConversionResult> {
        unimplemented!("XLSX conversion not yet implemented")
    }

    fn metadata(&self) -> ConverterMetadata {
        ConverterMetadata {
            name: "XLSX Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Pure Rust XLSX to Markdown/CSV converter".to_string(),
            external_deps: vec!["umya-spreadsheet".to_string()],
        }
    }
}

