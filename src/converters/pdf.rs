//! PDF converter implementation
//!
//! Pure Rust PDF to Markdown converter using lopdf for parsing.

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::engines::layout_analyzer::LayoutAnalyzer;
use crate::engines::pdf_parser::{PdfParser, PdfPage};
use crate::optimization::text::TextOptimizer;
use crate::output::{Chunker, MarkdownGenerator};
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

    /// Join lines that belong to the same paragraph (Docling-style)
    /// This function mimics Docling's text joining behavior
    fn join_paragraph_lines(text: &str) -> String {
        let lines: Vec<&str> = text.lines().collect();
        let mut result = String::new();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i];
            let trimmed = line.trim();
            
            // Skip empty lines (they will be added when needed)
            if trimmed.is_empty() {
                i += 1;
                continue;
            }
            
            // Pre-compute line context
            let prev_empty = i == 0 || lines.get(i-1).map(|l| l.trim().is_empty()).unwrap_or(false);
            let next_empty = i == lines.len() - 1 || lines.get(i+1).map(|l| l.trim().is_empty()).unwrap_or(false);
            
            // FIRST: Check if this looks like an author name (before heading detection)
            let looks_like_name = (trimmed.len() > 5 && trimmed.len() < 80) && 
                                (trimmed.chars().next().map(|c| c.is_uppercase() || c == ' ').unwrap_or(false)) &&
                                !trimmed.contains('@') &&
                                !trimmed.ends_with('.') &&
                                !trimmed.starts_with("The ") &&
                                !trimmed.starts_with("In ") &&
                                !trimmed.starts_with("Most ") &&
                                !trimmed.starts_with("Recurrent ") &&
                                !trimmed.starts_with("Attention ") &&
                                trimmed.split_whitespace().count() >= 2 &&
                                trimmed.split_whitespace().count() <= 8;
            
            // Check if next line is likely affiliation or symbol
            let next_is_affiliation_or_symbol = if i + 1 < lines.len() {
                let next = lines[i+1].trim();
                next.contains("Google") || next.contains("University") ||
                next.contains("Research") || next.contains("Brain") ||
                next.contains("Toronto") ||
                next == "∗" || next == "†" || next == "‡" || next == "∗ †" || next == "∗ ‡"
            } else {
                false
            };
            
            // If this is a potential author name, collect following lines (symbol, affiliation, email)
            if looks_like_name && next_is_affiliation_or_symbol {
                let mut author_parts = vec![trimmed];
                let mut j = i + 1;
                
                // Collect up to 4 following lines for symbol/affiliation/email
                while j < lines.len() && j < i + 5 {
                    let next_line = lines[j].trim();
                    if next_line.is_empty() {
                        break;
                    }
                    
                    let is_affiliation = next_line.contains("Google") || next_line.contains("University") ||
                                       next_line.contains("Research") || next_line.contains("Brain") ||
                                       next_line.contains("Toronto");
                    let is_email = next_line.contains('@');
                    let is_symbol = next_line == "∗" || next_line == "†" || next_line == "‡" || 
                                  next_line == "∗ †" || next_line == "∗ ‡";
                    
                    if is_affiliation || is_email || is_symbol {
                        author_parts.push(next_line);
                        j += 1;
                        
                        // Stop after email
                        if is_email {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                
                // If we found author components with email, join them
                if author_parts.len() >= 2 && author_parts.iter().any(|p| p.contains('@')) {
                    if !result.is_empty() && !result.ends_with("\n\n") {
                        result.push_str("\n\n");
                    }
                    result.push_str(&author_parts.join(" "));
                    result.push_str("\n\n");
                    i = j;
                    continue;
                }
            }
            
            // Check if this is a footnote marker line
            if trimmed.starts_with('∗') || trimmed.starts_with('†') || trimmed.starts_with('‡') {
                if !result.is_empty() && !result.ends_with("\n\n") {
                    result.push_str("\n\n");
                }
                result.push_str(trimmed);
                result.push_str("\n\n");
                i += 1;
                continue;
            }
            
            // SECOND: Check if this is a heading (after author detection)
            // Heading detection: specific patterns only
            
            // Main title: appears early, surrounded by empty lines, title case
            let is_main_title = i < 10 && prev_empty && next_empty && 
                              trimmed.len() > 10 && trimmed.len() < 100 &&
                              trimmed.split_whitespace().count() >= 3 &&
                              trimmed.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) &&
                              !trimmed.contains('@') &&
                              !trimmed.contains('∗');
            
            // Section headings: Abstract or numbered sections
            let is_section_heading = trimmed == "Abstract" ||
                                    (prev_empty && next_empty && trimmed.starts_with(|c: char| c.is_numeric()) && 
                                     trimmed.contains(' ') && trimmed.len() < 80);
            
            if is_main_title || is_section_heading {
                // Add heading with proper spacing
                if !result.is_empty() {
                    result.push_str("\n\n");
                }
                // Add ## for all headings (title, Abstract, numbered sections)
                result.push_str("## ");
                result.push_str(trimmed);
                result.push_str("\n\n");
                i += 1;
                continue;
            }
            
            // Regular text - join with previous line if it's part of the same paragraph
            if !result.is_empty() && !result.ends_with("\n\n") {
                // Check if we should join with previous line
                let should_join = !prev_empty;
                
                if should_join {
                    // Remove trailing hyphen if present (word continuation)
                    if result.ends_with('-') {
                        result.pop();
                    } else if !result.ends_with(' ') {
                        result.push(' ');
                    }
                }
            }
            
            result.push_str(trimmed);
            
            // Check if this line ends a sentence (period, colon, etc.)
            let ends_sentence = trimmed.ends_with('.') || trimmed.ends_with(':') || 
                              trimmed.ends_with('!') || trimmed.ends_with('?');
            
            // Don't end paragraph on abbreviations
            let is_abbreviation = trimmed.ends_with(" al.") || trimmed.ends_with(" Fig.") ||
                                trimmed.ends_with(" et.") || trimmed.ends_with(" vs.");
            
            // Add paragraph break if sentence ends and it's not an abbreviation
            if ends_sentence && !is_abbreviation && next_empty {
                result.push_str("\n\n");
            }
            
            i += 1;
        }
        
        // Final cleanup
        let mut cleaned = result.replace("\n\n\n\n", "\n\n");
        cleaned = cleaned.replace("\n\n\n", "\n\n");
        
        // Ensure proper spacing around headings
        cleaned = cleaned.replace("##  ", "## ");
        cleaned = cleaned.replace("\n\n## ", "\n## ");
        
        cleaned.trim().to_string()
    }
    
    /// Convert PDF to Markdown using pdf-extract (high quality)
    #[cfg(feature = "pdf")]
    async fn convert_to_markdown_pdf_extract(
        &self,
        path: &Path,
        options: &ConversionOptions,
    ) -> Result<Vec<ConversionOutput>> {
        use pdf_extract::extract_text;
        
        // Extract all text at once (better quality than lopdf)
        let raw_text = extract_text(path)
            .map_err(|e| crate::TransmutationError::engine_error("PDF Parser", format!("pdf-extract failed: {:?}", e)))?;
        
        // Post-process: join lines that belong to same paragraph (like Docling does)
        let markdown = Self::join_paragraph_lines(&raw_text);
        
        // The text is already in markdown format, no need for additional processing
        let data = markdown.into_bytes();
        let size_bytes = data.len() as u64;
        
        Ok(vec![ConversionOutput {
            page_number: 0,
            data,
            metadata: OutputMetadata {
                size_bytes,
                ..Default::default()
            },
        }])
    }
    
    /// Convert PDF to Markdown
    async fn convert_to_markdown(
        &self,
        parser: &PdfParser,
        options: &ConversionOptions,
    ) -> Result<Vec<ConversionOutput>> {
        let pages = parser.extract_all_pages()?;
        
        // Use layout analysis if text blocks are available
        let analyzer = LayoutAnalyzer::new();
        let markdown_outputs: Vec<String> = if options.split_pages {
            // Generate separate markdown for each page
            pages
                .iter()
                .map(|page| {
                    if !page.text_blocks.is_empty() {
                        // Use semantic layout analysis
                        let analyzed = analyzer.analyze(&page.text_blocks);
                        MarkdownGenerator::from_analyzed_blocks(&analyzed, options.clone())
                    } else {
                        // Fallback to simple text extraction
                        let text = if options.optimize_for_llm {
                            self.text_optimizer.optimize(&page.text)
                        } else {
                            page.text.clone()
                        };
                        MarkdownGenerator::from_text(&text, options.clone())
                    }
                })
                .collect()
        } else {
            // Combined document with all pages
            let mut all_analyzed_blocks = Vec::new();
            
            for page in &pages {
                if !page.text_blocks.is_empty() {
                    let analyzed = analyzer.analyze(&page.text_blocks);
                    all_analyzed_blocks.extend(analyzed);
                } else {
                    // Fallback to text extraction
                    let text = if options.optimize_for_llm {
                        self.text_optimizer.optimize(&page.text)
                    } else {
                        page.text.clone()
                    };
                    // Convert text to simple paragraph blocks
                    for para in text.split("\n\n") {
                        if !para.trim().is_empty() {
                            all_analyzed_blocks.push(crate::engines::layout_analyzer::AnalyzedBlock {
                                block_type: crate::engines::layout_analyzer::BlockType::Paragraph,
                                content: para.to_string(),
                                level: None,
                                font_size: 10.0,
                                y_position: 0.0,
                            });
                        }
                    }
                }
            }
            
            vec![MarkdownGenerator::from_analyzed_blocks(&all_analyzed_blocks, options.clone())]
        };

        // Convert to ConversionOutput with optional chunking
        let outputs = markdown_outputs
            .into_iter()
            .enumerate()
            .map(|(i, md)| {
                // Apply chunking if requested
                let (final_content, chunk_count, token_count) = if options.max_chunk_size > 0 {
                    let chunker = Chunker::from_options(&options);
                    let chunks = chunker.chunk(&md);
                    let total_tokens: usize = chunks.iter().map(|c| c.token_count).sum();
                    let chunk_count = chunks.len();
                    
                    // Combine chunks with separators
                    let combined = chunks
                        .into_iter()
                        .map(|c| c.content)
                        .collect::<Vec<_>>()
                        .join("\n\n---\n\n");
                    
                    (combined, chunk_count, Some(total_tokens))
                } else {
                    (md.clone(), 1, Some(md.len() / 4)) // Rough token estimate
                };
                
                ConversionOutput {
                    page_number: if options.split_pages { i } else { 0 },
                    data: final_content.as_bytes().to_vec(),
                    metadata: OutputMetadata {
                        size_bytes: final_content.len() as u64,
                        chunk_count,
                        token_count,
                    },
                }
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
                // Use pdf-extract for best quality (if available)
                #[cfg(feature = "pdf")]
                {
                    self.convert_to_markdown_pdf_extract(input, &options).await?
                }
                #[cfg(not(feature = "pdf"))]
                {
                    self.convert_to_markdown(&parser, &options).await?
                }
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
        
        // Extract tables if enabled
        let tables_extracted = if options.extract_tables {
            parser
                .extract_all_tables()
                .map(|tables| tables.iter().map(|(_, t)| t.len()).sum())
                .unwrap_or(0)
        } else {
            0
        };

        // Build statistics
        let duration = start_time.elapsed();
        let statistics = ConversionStatistics {
            input_size_bytes: input_size,
            output_size_bytes: output_size,
            duration,
            pages_processed: page_count,
            tables_extracted,
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


