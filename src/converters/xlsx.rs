//! XLSX converter implementation
//! 
//! Direct parsing of XLSX files (ZIP with XML) for multiple output formats:
//! - Markdown (tables)
//! - CSV (raw data)
//! - JSON (structured data)
//! - XML (original structure)

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::types::{ConversionOptions, ConversionResult, ConversionOutput, FileFormat, OutputFormat, OutputMetadata};
use crate::Result;
use async_trait::async_trait;
use std::path::Path;
use tokio::fs;

/// XLSX to multiple formats converter
/// 
/// Uses umya-spreadsheet for direct parsing (pure Rust, no LibreOffice)
pub struct XlsxConverter;

impl XlsxConverter {
    /// Create a new XLSX converter
    pub fn new() -> Self {
        Self
    }
    
    /// Read XLSX file and extract sheets using umya-spreadsheet
    fn read_xlsx(&self, path: &Path) -> Result<umya_spreadsheet::Spreadsheet> {
        eprintln!("📊 Reading XLSX file (umya-spreadsheet)...");
        
        let book = umya_spreadsheet::reader::xlsx::read(path).map_err(|e| {
            crate::TransmutationError::engine_error("xlsx-parser", format!("Failed to read XLSX: {}", e))
        })?;
        
        eprintln!("      ✓ Found {} sheets", book.get_sheet_count());
        Ok(book)
    }
    
    /// Convert XLSX to Markdown tables
    fn to_markdown(&self, book: &umya_spreadsheet::Spreadsheet) -> String {
        let mut markdown = String::new();
        markdown.push_str("# Spreadsheet\n\n");
        
        for (idx, sheet) in book.get_sheet_collection().iter().enumerate() {
            let sheet_name = sheet.get_name();
            markdown.push_str(&format!("## Sheet {}: {}\n\n", idx + 1, sheet_name));
            
            // Get sheet dimensions
            let highest_row = sheet.get_highest_row();
            let highest_col = sheet.get_highest_column();
            
            if highest_row == 0 || highest_col == 0 {
                markdown.push_str("*(Empty sheet)*\n\n");
                continue;
            }
            
            // Build table
            for row in 1..=highest_row {
                if row == 1 {
                    // Header row
                    markdown.push('|');
                    for col in 1..=highest_col {
                        let cell = sheet.get_cell((col, row));
                        let value = cell.map(|c| c.get_value().to_string()).unwrap_or_default();
                        markdown.push_str(&format!(" {} |", value));
                    }
                    markdown.push('\n');
                    
                    // Separator
                    markdown.push('|');
                    for _ in 1..=highest_col {
                        markdown.push_str("---|");
                    }
                    markdown.push('\n');
                } else {
                    // Data rows
                    markdown.push('|');
                    for col in 1..=highest_col {
                        let cell = sheet.get_cell((col, row));
                        let value = cell.map(|c| c.get_value().to_string()).unwrap_or_default();
                        markdown.push_str(&format!(" {} |", value));
                    }
                    markdown.push('\n');
                }
            }
            
            markdown.push_str("\n---\n\n");
        }
        
        markdown
    }
    
    /// Convert XLSX to CSV (first sheet only)
    fn to_csv(&self, book: &umya_spreadsheet::Spreadsheet, delimiter: char) -> String {
        let mut csv = String::new();
        
        // Get first sheet
        if let Some(sheet) = book.get_sheet_collection().first() {
            let highest_row = sheet.get_highest_row();
            let highest_col = sheet.get_highest_column();
            
            for row in 1..=highest_row {
                let mut values = Vec::new();
                for col in 1..=highest_col {
                    let cell = sheet.get_cell((col, row));
                    let value = cell.map(|c| c.get_value().to_string()).unwrap_or_default();
                    
                    // Quote values with commas
                    if value.contains(delimiter) || value.contains('"') {
                        values.push(format!("\"{}\"", value.replace('"', "\"\"")));
                    } else {
                        values.push(value);
                    }
                }
                csv.push_str(&values.join(&delimiter.to_string()));
                csv.push('\n');
            }
        }
        
        csv
    }
    
    /// Convert XLSX to JSON
    fn to_json(&self, book: &umya_spreadsheet::Spreadsheet) -> Result<String> {
        use serde_json::json;
        
        let mut sheets_json = Vec::new();
        
        for sheet in book.get_sheet_collection() {
            let sheet_name = sheet.get_name();
            let highest_row = sheet.get_highest_row();
            let highest_col = sheet.get_highest_column();
            
            let mut rows = Vec::new();
            
            for row in 1..=highest_row {
                let mut row_data = Vec::new();
                for col in 1..=highest_col {
                    let cell = sheet.get_cell((col, row));
                    let value = cell.map(|c| c.get_value().to_string()).unwrap_or_default();
                    row_data.push(value);
                }
                rows.push(row_data);
            }
            
            sheets_json.push(json!({
                "name": sheet_name,
                "rows": rows,
                "row_count": highest_row,
                "col_count": highest_col,
            }));
        }
        
        let result = json!({
            "spreadsheet": {
                "sheets": sheets_json,
                "sheet_count": book.get_sheet_count(),
            }
        });
        
        Ok(serde_json::to_string_pretty(&result)?)
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
        _options: ConversionOptions,
    ) -> Result<ConversionResult> {
        eprintln!("🔄 XLSX Conversion (Pure Rust)");
        eprintln!("   XLSX (ZIP) → XML Parsing → {:?}", output_format);
        eprintln!();
        
        // Read XLSX file
        let book = self.read_xlsx(input)?;
        
        // Convert to requested format
        let output_data = match output_format {
            OutputFormat::Markdown { .. } => {
                eprintln!("📝 Converting to Markdown tables...");
                self.to_markdown(&book).into_bytes()
            },
            OutputFormat::Csv { delimiter, .. } => {
                eprintln!("📝 Converting to CSV (delimiter: '{}')...", delimiter);
                self.to_csv(&book, delimiter).into_bytes()
            },
            OutputFormat::Json { .. } => {
                eprintln!("📝 Converting to JSON...");
                self.to_json(&book)?.into_bytes()
            },
            _ => {
                return Err(crate::TransmutationError::UnsupportedFormat(
                    format!("Output format {:?} not supported for XLSX", output_format)
                ));
            }
        };
        
        let output_size = output_data.len() as u64;
        
        Ok(ConversionResult {
            input_path: input.to_path_buf(),
            input_format: FileFormat::Xlsx,
            output_format,
            content: vec![ConversionOutput {
                page_number: 1,
                data: output_data,
                metadata: OutputMetadata {
                    size_bytes: output_size,
                    chunk_count: book.get_sheet_count(),
                    token_count: None,
                },
            }],
            metadata: crate::types::DocumentMetadata {
                title: None,
                author: None,
                created: None,
                modified: None,
                page_count: book.get_sheet_count(),
                language: None,
                custom: std::collections::HashMap::new(),
            },
            statistics: crate::types::ConversionStatistics {
                input_size_bytes: fs::metadata(input).await?.len(),
                output_size_bytes: output_size,
                duration: std::time::Duration::from_secs(0),
                pages_processed: book.get_sheet_count(),
                tables_extracted: book.get_sheet_count(),
                images_extracted: 0,
                cache_hit: false,
            },
        })
    }

    fn metadata(&self) -> ConverterMetadata {
        ConverterMetadata {
            name: "XLSX Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "XLSX to Markdown/CSV/JSON/XML converter (pure Rust, no LibreOffice needed)".to_string(),
            external_deps: vec![],
        }
    }
}


