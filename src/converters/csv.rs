//! CSV/TSV converter implementation
//!
//! Converts CSV/TSV files to Markdown tables and JSON.

#![allow(clippy::uninlined_format_args)]

use std::path::Path;

use async_trait::async_trait;
use tokio::fs;

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::Result;
use crate::types::{
    ConversionOptions, ConversionOutput, ConversionResult, FileFormat, OutputFormat, OutputMetadata,
};

/// CSV/TSV to Markdown converter
#[derive(Debug)]
pub struct CsvConverter {
    delimiter: char,
}

impl CsvConverter {
    /// Create a new CSV converter
    pub fn new() -> Self {
        Self { delimiter: ',' }
    }

    /// Create a TSV converter
    pub fn new_tsv() -> Self {
        Self { delimiter: '\t' }
    }

    /// Parse CSV and convert to Markdown table
    fn csv_to_markdown(&self, csv: &str) -> String {
        let lines: Vec<&str> = csv.lines().filter(|l| !l.trim().is_empty()).collect();

        if lines.is_empty() {
            return "# Empty File\n".to_string();
        }

        let mut markdown = String::new();
        markdown.push_str("# Data Table\n\n");

        for (idx, line) in lines.iter().enumerate() {
            let cells: Vec<&str> = line.split(self.delimiter).collect();

            // Header row
            if idx == 0 {
                markdown.push('|');
                for cell in &cells {
                    markdown.push_str(&format!(" {} |", cell.trim()));
                }
                markdown.push('\n');

                // Separator
                markdown.push('|');
                for _ in &cells {
                    markdown.push_str("---|");
                }
                markdown.push('\n');
            } else {
                // Data rows
                markdown.push('|');
                for cell in &cells {
                    markdown.push_str(&format!(" {} |", cell.trim()));
                }
                markdown.push('\n');
            }
        }

        markdown.push('\n');
        markdown
    }

    /// Parse CSV and convert to JSON
    fn csv_to_json(&self, csv: &str) -> Result<String> {
        let lines: Vec<&str> = csv.lines().filter(|l| !l.trim().is_empty()).collect();

        if lines.is_empty() {
            return Ok(serde_json::json!({"data": []}).to_string());
        }

        // First line is headers
        let headers: Vec<String> = lines[0]
            .split(self.delimiter)
            .map(|h| h.trim().to_string())
            .collect();

        // Remaining lines are data
        let mut rows = Vec::new();
        for line in &lines[1..] {
            let cells: Vec<&str> = line.split(self.delimiter).collect();
            let mut row = serde_json::Map::new();

            for (idx, cell) in cells.iter().enumerate() {
                if idx < headers.len() {
                    row.insert(
                        headers[idx].clone(),
                        serde_json::Value::String(cell.trim().to_string()),
                    );
                }
            }
            rows.push(row);
        }

        let result = serde_json::json!({
            "data": rows,
            "row_count": rows.len(),
            "column_count": headers.len(),
            "headers": headers,
        });

        Ok(serde_json::to_string_pretty(&result)?)
    }
}

impl Default for CsvConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentConverter for CsvConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![FileFormat::Csv, FileFormat::Tsv]
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
        _options: ConversionOptions,
    ) -> Result<ConversionResult> {
        eprintln!("🔄 CSV/TSV Conversion (Pure Rust)");
        eprintln!("   CSV → Parsing → {:?}", output_format);
        eprintln!();

        // Read CSV file
        let csv_content = fs::read_to_string(input).await?;

        // Convert to requested format
        let output_data = match output_format {
            OutputFormat::Markdown { .. } => {
                eprintln!("📝 Converting to Markdown table...");
                let markdown = self.csv_to_markdown(&csv_content);
                markdown.into_bytes()
            }
            OutputFormat::Json { .. } => {
                eprintln!("📝 Converting to JSON...");
                let json = self.csv_to_json(&csv_content)?;
                json.into_bytes()
            }
            _ => {
                return Err(crate::TransmutationError::UnsupportedFormat(format!(
                    "Output format {:?} not supported for CSV",
                    output_format
                )));
            }
        };

        let output_size = output_data.len() as u64;
        let input_size = fs::metadata(input).await?.len();

        eprintln!("✅ CSV conversion complete!");

        Ok(ConversionResult {
            input_path: input.to_path_buf(),
            input_format: if self.delimiter == ',' {
                FileFormat::Csv
            } else {
                FileFormat::Tsv
            },
            output_format,
            content: vec![ConversionOutput {
                page_number: 1,
                data: output_data,
                metadata: OutputMetadata {
                    size_bytes: output_size,
                    chunk_count: 1,
                    token_count: None,
                },
            }],
            metadata: crate::types::DocumentMetadata {
                title: None,
                author: None,
                created: None,
                modified: None,
                page_count: 1,
                language: None,
                custom: std::collections::HashMap::new(),
            },
            statistics: crate::types::ConversionStatistics {
                input_size_bytes: input_size,
                output_size_bytes: output_size,
                duration: std::time::Duration::from_secs(0),
                pages_processed: 1,
                tables_extracted: 1,
                images_extracted: 0,
                cache_hit: false,
            },
        })
    }

    fn metadata(&self) -> ConverterMetadata {
        ConverterMetadata {
            name: "CSV/TSV Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "CSV/TSV to Markdown tables and JSON (pure Rust)".to_string(),
            external_deps: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_converter_creation() {
        let converter = CsvConverter::new();
        let formats = converter.supported_formats();
        assert!(formats.contains(&FileFormat::Csv));
        assert!(formats.contains(&FileFormat::Tsv));
    }

    #[test]
    fn test_csv_to_markdown_basic() {
        let converter = CsvConverter::new();
        let csv = "Name,Age\nAlice,30\nBob,25";
        let result = converter.csv_to_markdown(csv);
        assert!(result.contains("Name"));
        assert!(result.contains("Alice"));
    }

    #[test]
    fn test_csv_converter_metadata() {
        let converter = CsvConverter::new();
        let meta = converter.metadata();
        assert_eq!(meta.name, "CSV/TSV Converter");
    }
}
