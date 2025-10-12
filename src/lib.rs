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
pub mod engines;
pub mod error;
pub mod integration;
pub mod optimization;
pub mod output;
pub mod utils;

pub use error::{Result, TransmutationError};

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
}

impl Default for Converter {
    fn default() -> Self {
        Self::new().expect("Failed to create default converter")
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

