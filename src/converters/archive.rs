//! Archive extraction and conversion

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::types::{ConversionOptions, ConversionResult, FileFormat, OutputFormat};
use crate::Result;
use async_trait::async_trait;
use std::path::Path;

/// Archive to Markdown converter
pub struct ArchiveConverter;

impl ArchiveConverter {
    /// Create a new Archive converter
    pub fn new() -> Self {
        Self
    }
}

impl Default for ArchiveConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentConverter for ArchiveConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![
            FileFormat::Zip,
            FileFormat::Tar,
            FileFormat::TarGz,
            FileFormat::TarBz2,
            FileFormat::SevenZ,
        ]
    }

    fn output_formats(&self) -> Vec<OutputFormat> {
        vec![OutputFormat::Json {
            structured: true,
            include_metadata: true,
        }]
    }

    async fn convert(
        &self,
        _input: &Path,
        _output_format: OutputFormat,
        _options: ConversionOptions,
    ) -> Result<ConversionResult> {
        unimplemented!("Archive conversion not yet implemented")
    }

    fn metadata(&self) -> ConverterMetadata {
        ConverterMetadata {
            name: "Archive Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Extract and convert archive contents".to_string(),
            external_deps: vec!["zip".to_string(), "tar".to_string()],
        }
    }
}

