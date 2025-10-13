//! # Transmutation
//!
//! High-performance document conversion engine for AI/LLM embeddings.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use transmutation::{Converter, OutputFormat, ConversionOptions};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize converter
//!     let converter = Converter::new()?;
//!     
//!     // Convert PDF to Markdown
//!     let result = converter
//!         .convert("document.pdf")
//!         .to(OutputFormat::Markdown { split_pages: true, optimize_for_llm: true })
//!         .execute()
//!         .await?;
//!     
//!     // Save output
//!     result.save("output/document.md").await?;
//!     
//!     Ok(())
//! }
//! ```

#![warn(missing_docs, missing_debug_implementations, rust_2024_compatibility)]
#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod converters;
#[cfg(all(feature = "pdf", feature = "docling-ffi"))]
pub mod document;
pub mod engines;
pub mod error;
pub mod integration;
#[cfg(feature = "docling-ffi")]
pub mod ml;
pub mod optimization;
pub mod output;
pub mod pipeline;  // Docling-style flexible export pipeline
pub mod types;
pub mod utils;

pub use converters::{ConverterMetadata, DocumentConverter};
pub use error::{Result, TransmutationError};
pub use types::*;

/// Current version of the Transmutation library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Main converter interface
pub struct Converter {
    config: ConverterConfig,
}

/// Converter configuration
#[derive(Debug, Clone)]
pub struct ConverterConfig {
    /// Enable caching
    pub enable_cache: bool,
    /// Maximum parallel conversions
    pub max_parallel: usize,
    /// Timeout for conversions
    pub timeout: std::time::Duration,
}

impl Default for ConverterConfig {
    fn default() -> Self {
        Self {
            enable_cache: true,
            max_parallel: num_cpus::get(),
            timeout: std::time::Duration::from_secs(300),
        }
    }
}

impl Converter {
    /// Create a new converter with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(ConverterConfig::default())
    }

    /// Create a new converter with custom configuration
    pub fn with_config(config: ConverterConfig) -> Result<Self> {
        tracing::info!("Initializing Transmutation v{}", VERSION);
        Ok(Self { config })
    }

    /// Get the current configuration
    pub fn config(&self) -> &ConverterConfig {
        &self.config
    }

    /// Start a conversion with builder pattern
    pub fn convert<P: AsRef<std::path::Path>>(&self, input: P) -> ConversionBuilder {
        ConversionBuilder::new(input.as_ref().to_path_buf())
    }
}

impl Default for Converter {
    fn default() -> Self {
        Self::new().expect("Failed to create default converter")
    }
}

/// Builder for conversions with fluent API
pub struct ConversionBuilder {
    input: std::path::PathBuf,
    output_format: Option<OutputFormat>,
    options: ConversionOptions,
}

impl ConversionBuilder {
    /// Create a new conversion builder
    pub fn new(input: std::path::PathBuf) -> Self {
        Self {
            input,
            output_format: None,
            options: ConversionOptions::default(),
        }
    }

    /// Set the output format
    pub fn to(mut self, format: OutputFormat) -> Self {
        self.output_format = Some(format);
        self
    }

    /// Set conversion options
    pub fn with_options(mut self, options: ConversionOptions) -> Self {
        self.options = options;
        self
    }

    /// Execute the conversion
    pub async fn execute(self) -> Result<ConversionResult> {
        use crate::utils::detect_format;

        // Detect input format
        let input_format = detect_format(&self.input).await?;

        // Get output format (default to Markdown)
        let output_format = self.output_format.unwrap_or(OutputFormat::Markdown {
            split_pages: false,
            optimize_for_llm: true,
        });

        // Select appropriate converter
        #[cfg(feature = "pdf")]
        if input_format == FileFormat::Pdf {
            use crate::converters::pdf::PdfConverter;
            let converter = PdfConverter::new();
            return converter.convert(&self.input, output_format, self.options).await;
        }

        #[cfg(feature = "office")]
        if input_format == FileFormat::Docx {
            use crate::converters::docx::DocxConverter;
            let converter = DocxConverter::new();
            return converter.convert(&self.input, output_format, self.options).await;
        }

        #[cfg(feature = "office")]
        if input_format == FileFormat::Xlsx {
            use crate::converters::xlsx::XlsxConverter;
            let converter = XlsxConverter::new();
            return converter.convert(&self.input, output_format, self.options).await;
        }

        #[cfg(feature = "office")]
        if input_format == FileFormat::Pptx {
            use crate::converters::pptx::PptxConverter;
            let converter = PptxConverter::new();
            return converter.convert(&self.input, output_format, self.options).await;
        }

        #[cfg(feature = "web")]
        if input_format == FileFormat::Html {
            use crate::converters::html::HtmlConverter;
            let converter = HtmlConverter::new();
            return converter.convert(&self.input, output_format, self.options).await;
        }

        #[cfg(feature = "web")]
        if input_format == FileFormat::Xml {
            use crate::converters::xml::XmlConverter;
            let converter = XmlConverter::new();
            return converter.convert(&self.input, output_format, self.options).await;
        }

        // Format not supported or feature not enabled
        Err(TransmutationError::UnsupportedFormat(format!(
            "Format {:?} is not supported or feature not enabled",
            input_format
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converter_creation() {
        let converter = Converter::new();
        assert!(converter.is_ok());
    }

    #[test]
    fn test_default_config() {
        let config = ConverterConfig::default();
        assert!(config.enable_cache);
        assert!(config.max_parallel > 0);
    }
}

