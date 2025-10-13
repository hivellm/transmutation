//! DOCX converter implementation
//! 
//! Converts Microsoft Word documents (.docx) to Markdown with precision quality

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::types::{
    ConversionOptions, ConversionOutput, ConversionResult, ConversionStatistics,
    DocumentMetadata, FileFormat, OutputFormat, OutputMetadata,
};
use crate::Result;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// DOCX to Markdown converter
pub struct DocxConverter;

impl DocxConverter {
    /// Create a new DOCX converter
    pub fn new() -> Self {
        Self
    }
    
    /// Convert DOCX to Markdown using simple text extraction
    /// Uses docx-rs for parsing Word documents
    #[cfg(feature = "office")]
    async fn convert_to_markdown(
        &self,
        path: &Path,
        _options: &ConversionOptions,
    ) -> Result<Vec<ConversionOutput>> {
        eprintln!("📄 Reading DOCX file with docx-rs...");
        
        // Read DOCX file
        let file_data = tokio::fs::read(path).await?;
        
        // Parse DOCX using docx-rs
        let docx = docx_rs::read_docx(&file_data)
            .map_err(|e| crate::TransmutationError::engine_error("docx-rs", format!("Failed to parse DOCX: {:?}", e)))?;
        
        eprintln!("✓ DOCX parsed successfully");
        
        // Extract text from document body
        let mut markdown = String::new();
        
        // Simple text extraction from paragraphs
        for child in &docx.document.children {
            let text = self.extract_text_from_child(child);
            if !text.is_empty() {
                markdown.push_str(&text);
                markdown.push_str("\n\n");
            }
        }
        
        // Clean up excessive newlines
        while markdown.contains("\n\n\n") {
            markdown = markdown.replace("\n\n\n", "\n\n");
        }
        
        let markdown = markdown.trim().to_string();
        
        eprintln!("✓ Converted to Markdown: {} chars", markdown.len());
        
        let token_count = markdown.len() / 4;
        let data = markdown.into_bytes();
        let size_bytes = data.len() as u64;
        
        Ok(vec![ConversionOutput {
            page_number: 0,
            data,
            metadata: OutputMetadata {
                size_bytes,
                chunk_count: 1,
                token_count: Some(token_count),
            },
        }])
    }
    
    /// Extract text from document child (simplified approach)
    #[cfg(feature = "office")]
    fn extract_text_from_child(&self, child: &docx_rs::DocumentChild) -> String {
        use docx_rs::DocumentChild;
        
        match child {
            DocumentChild::Paragraph(para) => {
                self.extract_paragraph_text(para)
            }
            DocumentChild::Table(table) => {
                self.extract_table_text(table)
            }
            _ => String::new(),
        }
    }
    
    /// Extract text from paragraph
    #[cfg(feature = "office")]
    fn extract_paragraph_text(&self, para: &docx_rs::Paragraph) -> String {
        use docx_rs::ParagraphChild;
        
        let mut text = String::new();
        
        for child in &para.children {
            if let ParagraphChild::Run(run) = child {
                for run_child in &run.children {
                    if let docx_rs::RunChild::Text(t) = run_child {
                        text.push_str(&t.text);
                    }
                }
            }
        }
        
        text.trim().to_string()
    }
    
    /// Extract text from table (simplified - just get text, skip table structure for now)
    #[cfg(feature = "office")]
    fn extract_table_text(&self, _table: &docx_rs::Table) -> String {
        // TODO: Implement proper table extraction
        // For now, just indicate a table was present
        String::from("[Table content]")
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
        input: &Path,
        output_format: OutputFormat,
        options: ConversionOptions,
    ) -> Result<ConversionResult> {
        let start_time = Instant::now();
        
        // Get input file size
        let input_size = tokio::fs::metadata(input).await?.len();
        
        // Convert based on output format
        let content = match output_format {
            OutputFormat::Markdown { .. } => {
                #[cfg(feature = "office")]
                {
                    self.convert_to_markdown(input, &options).await?
                }
                #[cfg(not(feature = "office"))]
                {
                    return Err(crate::TransmutationError::InvalidOptions(
                        "DOCX conversion requires office feature".to_string()
                    ));
                }
            }
            _ => {
                return Err(crate::TransmutationError::InvalidOptions(
                    format!("Unsupported output format for DOCX: {:?}", output_format)
                ));
            }
        };
        
        // Calculate output size
        let output_size: u64 = content.iter().map(|c| c.metadata.size_bytes).sum();
        
        // Build metadata
        let metadata = DocumentMetadata {
            title: None, // TODO: Extract from DOCX properties
            author: None,
            created: None,
            modified: None,
            page_count: 1, // DOCX doesn't have strict pages
            language: None,
            custom: std::collections::HashMap::new(),
        };
        
        // Build statistics
        let duration = start_time.elapsed();
        let statistics = ConversionStatistics {
            input_size_bytes: input_size,
            output_size_bytes: output_size,
            duration,
            pages_processed: 1,
            tables_extracted: 0, // TODO: Count tables
            images_extracted: 0,
            cache_hit: false,
        };
        
        Ok(ConversionResult {
            input_path: PathBuf::from(input),
            input_format: FileFormat::Docx,
            output_format,
            content,
            metadata,
            statistics,
        })
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

