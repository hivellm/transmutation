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
    
    /// Convert DOCX to Markdown
    #[cfg(feature = "office")]
    async fn convert_to_markdown(
        &self,
        path: &Path,
        _options: &ConversionOptions,
    ) -> Result<Vec<ConversionOutput>> {
        use docx_rs::*;
        
        eprintln!("📄 Reading DOCX file...");
        
        // Read DOCX file
        let file_data = tokio::fs::read(path).await?;
        
        // Parse DOCX
        let docx = read_docx(&file_data)
            .map_err(|e| crate::TransmutationError::engine_error("docx-rs", format!("Failed to parse DOCX: {:?}", e)))?;
        
        eprintln!("✓ DOCX parsed successfully");
        
        // Extract text from document
        let mut markdown = String::new();
        
        // Process document body
        for child in &docx.document.children {
            match child {
                DocumentChild::Paragraph(para) => {
                    let para_text = self.paragraph_to_markdown(para);
                    if !para_text.is_empty() {
                        markdown.push_str(&para_text);
                        markdown.push_str("\n\n");
                    }
                }
                DocumentChild::Table(table) => {
                    let table_md = self.table_to_markdown(table);
                    markdown.push_str(&table_md);
                    markdown.push_str("\n\n");
                }
                _ => {
                    // Handle other types if needed
                }
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
    
    /// Convert paragraph to Markdown
    #[cfg(feature = "office")]
    fn paragraph_to_markdown(&self, para: &Paragraph) -> String {
        let mut text = String::new();
        let mut is_bold = false;
        let mut is_italic = false;
        
        // Extract runs from paragraph
        for child in &para.children {
            if let ParagraphChild::Run(run) = child {
                // Check formatting
                let mut run_text = String::new();
                
                for run_child in &run.children {
                    if let RunChild::Text(t) = run_child {
                        run_text.push_str(&t.text);
                    }
                }
                
                // Apply formatting
                if let Some(rpr) = &run.run_property {
                    is_bold = rpr.bold.is_some();
                    is_italic = rpr.italic.is_some();
                }
                
                if is_bold && is_italic {
                    text.push_str(&format!("***{}***", run_text));
                } else if is_bold {
                    text.push_str(&format!("**{}**", run_text));
                } else if is_italic {
                    text.push_str(&format!("*{}*", run_text));
                } else {
                    text.push_str(&run_text);
                }
            }
        }
        
        // Check if it's a heading
        if let Some(ppr) = &para.property {
            if let Some(style) = &ppr.style {
                if style.val.starts_with("Heading") {
                    // Extract heading level
                    let level = style.val.chars()
                        .last()
                        .and_then(|c| c.to_digit(10))
                        .unwrap_or(1) as usize;
                    
                    let prefix = "#".repeat(level.min(6));
                    return format!("{} {}", prefix, text.trim());
                }
            }
        }
        
        text.trim().to_string()
    }
    
    /// Convert table to Markdown
    #[cfg(feature = "office")]
    fn table_to_markdown(&self, table: &Table) -> String {
        let mut md = String::new();
        let mut rows: Vec<Vec<String>> = Vec::new();
        
        // Extract table data
        for row in &table.rows {
            let mut row_data = Vec::new();
            
            for cell in &row.cells {
                let mut cell_text = String::new();
                
                for child in &cell.children {
                    if let TableCellContent::Paragraph(para) = child {
                        let para_text = self.paragraph_to_markdown(para);
                        if !para_text.is_empty() {
                            if !cell_text.is_empty() {
                                cell_text.push(' ');
                            }
                            cell_text.push_str(&para_text);
                        }
                    }
                }
                
                row_data.push(cell_text);
            }
            
            rows.push(row_data);
        }
        
        // Generate Markdown table
        if rows.is_empty() {
            return String::new();
        }
        
        // Header row
        if let Some(header) = rows.first() {
            md.push_str("| ");
            md.push_str(&header.join(" | "));
            md.push_str(" |\n");
            
            // Separator
            md.push_str("|");
            for _ in 0..header.len() {
                md.push_str(" --- |");
            }
            md.push('\n');
        }
        
        // Data rows
        for row in rows.iter().skip(1) {
            md.push_str("| ");
            md.push_str(&row.join(" | "));
            md.push_str(" |\n");
        }
        
        md
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

