//! Image converter with OCR support
//!
//! Converts image files to text using Tesseract OCR.

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::types::{ConversionOptions, ConversionResult, ConversionOutput, FileFormat, OutputFormat, OutputMetadata};
use crate::Result;
use async_trait::async_trait;
use std::path::Path;
use tokio::fs;

/// Image to text converter (OCR)
pub struct ImageConverter {
    #[cfg(feature = "tesseract")]
    ocr_engine: Option<String>,
}

impl ImageConverter {
    /// Create a new image converter
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "tesseract")]
            ocr_engine: Some("tesseract".to_string()),
        }
    }
    
    /// Perform OCR on an image
    #[cfg(feature = "tesseract")]
    async fn ocr_image(&self, image_path: &Path, language: &str) -> Result<String> {
        use leptess::{LepTess, Variable};
        
        // Initialize Tesseract
        let mut tesseract = LepTess::new(None, language)
            .map_err(|e| crate::TransmutationError::conversion_failed(&format!("Failed to initialize Tesseract: {}", e)))?;
        
        // Set image
        tesseract.set_image(image_path)
            .map_err(|e| crate::TransmutationError::conversion_failed(&format!("Failed to set image: {}", e)))?;
        
        // Get text
        let text = tesseract.get_utf8_text()
            .map_err(|e| crate::TransmutationError::conversion_failed(&format!("OCR failed: {}", e)))?;
        
        Ok(text)
    }
    
    /// Convert image to Markdown
    async fn image_to_markdown(&self, image_path: &Path, language: &str) -> Result<String> {
        #[cfg(feature = "tesseract")]
        {
            let text = self.ocr_image(image_path, language).await?;
            
            let mut markdown = String::new();
            markdown.push_str("# OCR Result\n\n");
            
            // Add paragraphs
            for para in text.split("\n\n") {
                let trimmed = para.trim();
                if !trimmed.is_empty() {
                    markdown.push_str(&format!("{}\n\n", trimmed));
                }
            }
            
            Ok(markdown)
        }
        
        #[cfg(not(feature = "tesseract"))]
        {
            Err(crate::TransmutationError::conversion_failed(
                "OCR feature not enabled. Compile with --features tesseract"
            ))
        }
    }
}

impl Default for ImageConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentConverter for ImageConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![
            FileFormat::Jpeg,
            FileFormat::Png,
            FileFormat::Tiff,
            FileFormat::Bmp,
            FileFormat::Gif,
            FileFormat::Webp,
        ]
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
        eprintln!("🔄 Image OCR (Tesseract)");
        eprintln!("   Image → OCR → {:?}", output_format);
        eprintln!();
        
        let language = "eng";  // Default to English (can be made configurable later)
        
        #[cfg(feature = "tesseract")]
        {
            eprintln!("📸 Running OCR (language: {})...", language);
            
            // Convert image to text
            let markdown = self.image_to_markdown(input, language).await?;
            
            // Convert to requested format
            let output_data = match output_format {
                OutputFormat::Markdown { .. } => {
                    eprintln!("📝 Markdown generated!");
                    markdown.into_bytes()
                },
                OutputFormat::Json { .. } => {
                    eprintln!("📝 Converting to JSON...");
                    let json = serde_json::json!({
                        "ocr": {
                            "text": markdown,
                            "language": language,
                        }
                    });
                    serde_json::to_string_pretty(&json)?.into_bytes()
                },
                _ => {
                    return Err(crate::TransmutationError::UnsupportedFormat(
                        format!("Output format {:?} not supported for images", output_format)
                    ));
                }
            };
            
            let output_size = output_data.len() as u64;
            let input_size = fs::metadata(input).await?.len();
            
            eprintln!("✅ OCR complete!");
            
            Ok(ConversionResult {
                input_path: input.to_path_buf(),
                input_format: crate::utils::file_detect::detect_format(input).await?,
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
                    language: Some(language.to_string()),
                    custom: std::collections::HashMap::new(),
                },
                statistics: crate::types::ConversionStatistics {
                    input_size_bytes: input_size,
                    output_size_bytes: output_size,
                    duration: std::time::Duration::from_secs(0),
                    pages_processed: 1,
                    tables_extracted: 0,
                    images_extracted: 0,
                    cache_hit: false,
                },
            })
        }
        
        #[cfg(not(feature = "tesseract"))]
        {
            Err(crate::TransmutationError::conversion_failed(
                "OCR feature not enabled. Compile with --features image-ocr"
            ))
        }
    }

    fn metadata(&self) -> ConverterMetadata {
        ConverterMetadata {
            name: "Image OCR Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Image to text converter using Tesseract OCR".to_string(),
            external_deps: vec!["tesseract".to_string()],
        }
    }
}
