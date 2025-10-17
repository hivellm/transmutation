//! Converter trait definitions

use std::path::Path;

use async_trait::async_trait;

use crate::Result;
use crate::types::{ConversionOptions, ConversionResult, FileFormat, OutputFormat};

/// Main trait for document converters
#[async_trait]
pub trait DocumentConverter: Send + Sync {
    /// Get the input formats supported by this converter
    fn supported_formats(&self) -> Vec<FileFormat>;

    /// Get the output formats supported by this converter
    fn output_formats(&self) -> Vec<OutputFormat>;

    /// Check if this converter can handle the given format
    fn can_convert(&self, format: FileFormat) -> bool {
        self.supported_formats().contains(&format)
    }

    /// Convert a document
    async fn convert(
        &self,
        input: &Path,
        output_format: OutputFormat,
        options: ConversionOptions,
    ) -> Result<ConversionResult>;

    /// Get converter metadata
    fn metadata(&self) -> ConverterMetadata;
}

/// Metadata about a converter
#[derive(Debug, Clone)]
pub struct ConverterMetadata {
    /// Converter name
    pub name: String,
    /// Version
    pub version: String,
    /// Description
    pub description: String,
    /// Requires external dependencies
    pub external_deps: Vec<String>,
}

impl Default for ConverterMetadata {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            version: "0.0.0".to_string(),
            description: String::new(),
            external_deps: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converter_metadata_default() {
        let meta = ConverterMetadata::default();
        assert_eq!(meta.name, "Unknown");
        assert!(meta.external_deps.is_empty());
    }
}
