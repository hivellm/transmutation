//! PDF converter implementation
//!
//! Pure Rust PDF to Markdown converter using lopdf for parsing.

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::engines::pdf_parser::{PdfParser, PdfPage};
use crate::optimization::text::TextOptimizer;
use crate::output::markdown::MarkdownGenerator;
use crate::types::{
    ConversionOptions, ConversionOutput, ConversionResult, ConversionStatistics,
    DocumentMetadata, FileFormat, OutputFormat, OutputMetadata,
};
use crate::Result;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// PDF to Markdown/Image/JSON converter
pub struct PdfConverter {
    text_optimizer: TextOptimizer,
}

impl PdfConverter {
    /// Create a new PDF converter
    pub fn new() -> Self {
        Self {
            text_optimizer: TextOptimizer::new(),
        }
    }

    /// Convert PDF to Markdown
    async fn convert_to_markdown(
        &self,
        parser: &PdfParser,
        options: &ConversionOptions,
    ) -> Result<Vec<ConversionOutput>> {
        let pages = parser.extract_all_pages()?;
        
        // Extract page texts
        let page_texts: Vec<(usize, String)> = pages
            .iter()
            .map(|page| {
                let optimized = if options.optimize_for_llm {
                    self.text_optimizer.optimize(&page.text)
                } else {
                    page.text.clone()
                };
                (page.number, optimized)
            })
            .collect();

        // Generate Markdown
        let markdown_outputs = MarkdownGenerator::from_pages(&page_texts, options.clone());

        // Convert to ConversionOutput
        let outputs = markdown_outputs
            .into_iter()
            .enumerate()
            .map(|(i, md)| ConversionOutput {
                page_number: if options.split_pages { i } else { 0 },
                data: md.as_bytes().to_vec(),
                metadata: OutputMetadata {
                    size_bytes: md.len() as u64,
                    chunk_count: 1, // TODO: Implement actual chunking
                    token_count: None, // TODO: Implement token counting
                },
            })
            .collect();

        Ok(outputs)
    }

    /// Convert PDF to JSON
    async fn convert_to_json(
        &self,
        parser: &PdfParser,
        options: &ConversionOptions,
    ) -> Result<Vec<ConversionOutput>> {
        let pages = parser.extract_all_pages()?;
        let metadata = parser.get_metadata();

        // Create JSON structure
        let json_data = serde_json::json!({
            "format": "pdf",
            "metadata": {
                "title": metadata.title,
                "author": metadata.author,
                "created": metadata.created,
                "modified": metadata.modified,
                "page_count": metadata.page_count,
            },
            "pages": pages.iter().map(|page| serde_json::json!({
                "number": page.number,
                "text": if options.optimize_for_llm {
                    self.text_optimizer.optimize(&page.text)
                } else {
                    page.text.clone()
                },
                "width": page.width,
                "height": page.height,
            })).collect::<Vec<_>>(),
        });

        let json_string = if options.include_metadata {
            serde_json::to_string_pretty(&json_data)
        } else {
            serde_json::to_string(&json_data)
        }
        .map_err(|e| crate::TransmutationError::SerializationError(e))?;

        Ok(vec![ConversionOutput {
            page_number: 0,
            data: json_string.as_bytes().to_vec(),
            metadata: OutputMetadata {
                size_bytes: json_string.len() as u64,
                chunk_count: pages.len(),
                token_count: None,
            },
        }])
    }

    /// Build document metadata from PDF
    fn build_metadata(&self, parser: &PdfParser) -> DocumentMetadata {
        let pdf_meta = parser.get_metadata();
        
        DocumentMetadata {
            title: pdf_meta.title,
            author: pdf_meta.author,
            created: pdf_meta.created,
            modified: pdf_meta.modified,
            page_count: pdf_meta.page_count,
            language: None, // TODO: Implement language detection
            custom: std::collections::HashMap::new(),
        }
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
        input: &Path,
        output_format: OutputFormat,
        options: ConversionOptions,
    ) -> Result<ConversionResult> {
        let start_time = Instant::now();
        
        // Load PDF
        let parser = PdfParser::load(input)?;
        
        // Get input file size
        let input_size = tokio::fs::metadata(input).await?.len();
        
        // Convert based on output format
        let content = match output_format {
            OutputFormat::Markdown { .. } => {
                self.convert_to_markdown(&parser, &options).await?
            }
            OutputFormat::Json { .. } => {
                self.convert_to_json(&parser, &options).await?
            }
            _ => {
                return Err(crate::TransmutationError::InvalidOptions(
                    format!("Unsupported output format for PDF: {:?}", output_format)
                ));
            }
        };

        // Calculate output size
        let output_size: u64 = content.iter().map(|c| c.metadata.size_bytes).sum();
        
        // Build metadata
        let metadata = self.build_metadata(&parser);
        let page_count = parser.page_count();
        
        // Build statistics
        let duration = start_time.elapsed();
        let statistics = ConversionStatistics {
            input_size_bytes: input_size,
            output_size_bytes: output_size,
            duration,
            pages_processed: page_count,
            tables_extracted: 0, // TODO: Implement table extraction
            images_extracted: 0, // TODO: Implement image extraction
            cache_hit: false,
        };

        Ok(ConversionResult {
            input_path: PathBuf::from(input),
            input_format: FileFormat::Pdf,
            output_format,
            content,
            metadata,
            statistics,
        })
    }

    fn metadata(&self) -> ConverterMetadata {
        ConverterMetadata {
            name: "PDF Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Pure Rust PDF to Markdown/JSON converter using lopdf".to_string(),
            external_deps: vec!["lopdf".to_string()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_converter_creation() {
        let converter = PdfConverter::new();
        assert_eq!(converter.supported_formats(), vec![FileFormat::Pdf]);
    }

    #[test]
    fn test_pdf_converter_metadata() {
        let converter = PdfConverter::new();
        let meta = converter.metadata();
        assert_eq!(meta.name, "PDF Converter");
        assert!(!meta.external_deps.is_empty());
    }

    // Integration tests with real PDFs will be in tests/pdf_tests.rs
}


