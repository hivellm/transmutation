//! PPTX converter implementation
//!
//! Two modes:
//! 1. **Text extraction**: Direct XML parsing from PPTX (ZIP) for clean text
//! 2. **Image export**: PPTX → PDF → Images (via LibreOffice) for visual slides

use super::traits::{ConverterMetadata, DocumentConverter};
use super::pdf::PdfConverter;
use crate::types::{ConversionOptions, ConversionResult, ConversionOutput, FileFormat, OutputFormat, OutputMetadata};
use crate::Result;
use async_trait::async_trait;
use std::path::Path;
use std::process::Command;
use std::io::Read;
use tokio::fs;

/// PPTX to Markdown converter
///
/// Uses LibreOffice to convert PPTX → PDF (one slide = one page), then processes as PDF
pub struct PptxConverter {
    pdf_converter: PdfConverter,
}

impl PptxConverter {
    /// Create a new PPTX converter
    pub fn new() -> Self {
        Self {
            pdf_converter: PdfConverter::new(),
        }
    }
    
    /// Extract text directly from PPTX XML (better quality than PDF route)
    fn extract_text_from_pptx(&self, path: &Path) -> Result<Vec<String>> {
        use zip::ZipArchive;
        use std::fs::File;
        
        eprintln!("📝 Extracting text from PPTX (Direct XML parsing)...");
        
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file).map_err(|e| {
            crate::TransmutationError::engine_error("zip", format!("Failed to open PPTX as ZIP: {}", e))
        })?;
        let mut slides = Vec::new();
        
        // Find all slide XML files
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| {
                crate::TransmutationError::engine_error("zip", format!("Failed to read file from PPTX: {}", e))
            })?;
            let name = file.name().to_string();
            
            // Process slide files: ppt/slides/slide*.xml
            if name.starts_with("ppt/slides/slide") && name.ends_with(".xml") {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                
                // Extract text from XML (simple approach - remove all tags)
                let text = self.extract_text_from_xml(&content);
                if !text.trim().is_empty() {
                    slides.push(text);
                }
            }
        }
        
        // Sort slides by number
        slides.sort();
        
        eprintln!("      ✓ Extracted text from {} slides", slides.len());
        Ok(slides)
    }
    
    /// Extract text content from XML (removes tags, keeps text)
    fn extract_text_from_xml(&self, xml: &str) -> String {
        use quick_xml::Reader;
        use quick_xml::events::Event;
        
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);
        
        let mut text_parts = Vec::new();
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Text(e)) => {
                    if let Ok(txt) = e.unescape() {
                        let content = txt.trim();
                        if !content.is_empty() {
                            text_parts.push(content.to_string());
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
            buf.clear();
        }
        
        text_parts.join(" ")
    }
    
    /// Convert PPTX to PDF using LibreOffice (for image export only)
    async fn pptx_to_pdf(&self, path: &Path) -> Result<std::path::PathBuf> {
        eprintln!("📊 Converting PPTX to PDF (LibreOffice)...");
        
        let temp_dir = std::env::temp_dir().join(format!("transmutation_pptx_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).await?;
        
        // Detect OS and use appropriate LibreOffice command
        let (libreoffice_cmd, install_msg) = if cfg!(target_os = "windows") {
            ("soffice.exe", "Install LibreOffice from https://www.libreoffice.org/download/")
        } else if cfg!(target_os = "macos") {
            ("/Applications/LibreOffice.app/Contents/MacOS/soffice", "Install: brew install libreoffice")
        } else {
            ("libreoffice", "Install: sudo apt install libreoffice")
        };
        
        let output = Command::new(libreoffice_cmd)
            .arg("--headless")
            .arg("--convert-to")
            .arg("pdf")
            .arg("--outdir")
            .arg(&temp_dir)
            .arg(path)
            .output()
            .map_err(|e| crate::TransmutationError::engine_error(
                "libreoffice",
                format!("Failed to run LibreOffice: {}.\n{}", e, install_msg)
            ))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let _ = fs::remove_dir_all(&temp_dir).await;
            return Err(crate::TransmutationError::engine_error(
                "libreoffice",
                format!("LibreOffice failed: {}", stderr)
            ));
        }
        
        // Find generated PDF (LibreOffice converts each slide to a page)
        let filename = path.file_stem().and_then(|s| s.to_str()).unwrap_or("presentation");
        let pdf_path = temp_dir.join(format!("{}.pdf", filename));
        
        if !pdf_path.exists() {
            let _ = fs::remove_dir_all(&temp_dir).await;
            return Err(crate::TransmutationError::engine_error(
                "libreoffice",
                "PDF not generated by LibreOffice".to_string()
            ));
        }
        
        let pdf_size = pdf_path.metadata()?.len();
        eprintln!("      ✓ PDF: {} KB ({} slides as pages)", pdf_size / 1024, filename);
        
        Ok(pdf_path)
    }
}

impl Default for PptxConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentConverter for PptxConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![FileFormat::Pptx]
    }

    fn output_formats(&self) -> Vec<OutputFormat> {
        vec![
            OutputFormat::Markdown {
                split_pages: true,  // Split by slide
                optimize_for_llm: true,
            },
            OutputFormat::Image {
                format: crate::types::ImageFormat::Png,
                quality: 85,
                dpi: 150,
            },
        ]
    }

    async fn convert(
        &self,
        input: &Path,
        output_format: OutputFormat,
        options: ConversionOptions,
    ) -> Result<ConversionResult> {
        // For images, use LibreOffice → PDF → Images
        // For Markdown, use direct XML parsing for better quality
        match output_format {
            OutputFormat::Image { .. } => {
                eprintln!("🔄 PPTX → Images Pipeline");
                eprintln!("   PPTX → PDF → Images (via LibreOffice)");
                eprintln!();
                
                // Use PDF pipeline for images
                let pdf_path = self.pptx_to_pdf(input).await?;
                let result = self.pdf_converter.convert(&pdf_path, output_format, options).await?;
                
                // Cleanup
                let temp_dir = pdf_path.parent().unwrap();
                let _ = fs::remove_dir_all(temp_dir).await;
                
                eprintln!("✅ PPTX → Images complete ({} slides)!", result.content.len());
                Ok(result)
            },
            
            OutputFormat::Markdown { split_pages, .. } => {
                eprintln!("🔄 PPTX → Markdown Pipeline");
                eprintln!("   PPTX (ZIP) → XML Parsing → Clean Text");
                eprintln!();
                
                // Extract text directly from XML
                let slides = self.extract_text_from_pptx(input)?;
                
                if slides.is_empty() {
                    return Err(crate::TransmutationError::engine_error(
                        "pptx-parser",
                        "No text content found in PPTX"
                    ));
                }
                
                let mut outputs = Vec::new();
                
                if split_pages {
                    // One file per slide
                    for (idx, slide_text) in slides.iter().enumerate() {
                        let markdown = format!("# Slide {}\n\n{}\n", idx + 1, slide_text);
                        outputs.push(ConversionOutput {
                            page_number: idx + 1,
                            data: markdown.as_bytes().to_vec(),
                            metadata: OutputMetadata {
                                size_bytes: markdown.len() as u64,
                                chunk_count: 1,
                                token_count: None,
                            },
                        });
                    }
                } else {
                    // Single file with all slides
                    let mut markdown = String::new();
                    markdown.push_str("# Presentation\n\n");
                    
                    for (idx, slide_text) in slides.iter().enumerate() {
                        markdown.push_str(&format!("## Slide {}\n\n{}\n\n---\n\n", idx + 1, slide_text));
                    }
                    
                    outputs.push(ConversionOutput {
                        page_number: 1,
                        data: markdown.as_bytes().to_vec(),
                        metadata: OutputMetadata {
                            size_bytes: markdown.len() as u64,
                            chunk_count: slides.len(),
                            token_count: None,
                        },
                    });
                }
                
                eprintln!("✅ PPTX → Markdown complete ({} slides)!", slides.len());
                
                let total_size: u64 = outputs.iter().map(|o| o.metadata.size_bytes).sum();
                
                Ok(ConversionResult {
                    input_path: input.to_path_buf(),
                    input_format: FileFormat::Pptx,
                    output_format,
                    content: outputs,
                    metadata: crate::types::DocumentMetadata {
                        title: None,
                        author: None,
                        created: None,
                        modified: None,
                        page_count: slides.len(),
                        language: None,
                        custom: std::collections::HashMap::new(),
                    },
                    statistics: crate::types::ConversionStatistics {
                        input_size_bytes: fs::metadata(input).await?.len(),
                        output_size_bytes: total_size,
                        duration: std::time::Duration::from_secs(0),
                        pages_processed: slides.len(),
                        tables_extracted: 0,
                        images_extracted: 0,
                        cache_hit: false,
                    },
                })
            },
            
            _ => {
                // Fallback: use PDF pipeline
                eprintln!("⚠️  Using fallback PDF pipeline for {:?}", output_format);
                let pdf_path = self.pptx_to_pdf(input).await?;
                let result = self.pdf_converter.convert(&pdf_path, output_format, options).await?;
                
                let temp_dir = pdf_path.parent().unwrap();
                let _ = fs::remove_dir_all(temp_dir).await;
                
                Ok(result)
            }
        }
    }

    fn metadata(&self) -> ConverterMetadata {
        ConverterMetadata {
            name: "PPTX Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "PPTX converter: Direct XML parsing (Markdown) or LibreOffice pipeline (Images)".to_string(),
            external_deps: vec!["LibreOffice (images only)".to_string()],
        }
    }
}

