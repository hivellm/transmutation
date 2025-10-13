//! Core types for Transmutation

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Supported file formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileFormat {
    // Document formats
    Pdf,
    Docx,
    Pptx,
    Xlsx,
    Html,
    Xml,
    Txt,
    Markdown,
    Rtf,
    Odt,
    Csv,
    Tsv,

    // Image formats
    Jpeg,
    Png,
    Tiff,
    Bmp,
    Gif,
    Webp,

    // Audio formats
    Mp3,
    Wav,
    M4a,
    Flac,

    // Video formats
    Mp4,
    Avi,
    Mkv,
    Mov,

    // Archive formats
    Zip,
    Tar,
    TarGz,
    TarBz2,
    SevenZ,

    // Unknown format
    Unknown,
}

impl FileFormat {
    /// Check if format is a document
    pub fn is_document(&self) -> bool {
        matches!(
            self,
            Self::Pdf
                | Self::Docx
                | Self::Pptx
                | Self::Xlsx
                | Self::Html
                | Self::Xml
                | Self::Txt
                | Self::Markdown
                | Self::Rtf
                | Self::Odt
                | Self::Csv
                | Self::Tsv
        )
    }

    /// Check if format is an image
    pub fn is_image(&self) -> bool {
        matches!(
            self,
            Self::Jpeg | Self::Png | Self::Tiff | Self::Bmp | Self::Gif | Self::Webp
        )
    }

    /// Check if format is audio
    pub fn is_audio(&self) -> bool {
        matches!(self, Self::Mp3 | Self::Wav | Self::M4a | Self::Flac)
    }

    /// Check if format is video
    pub fn is_video(&self) -> bool {
        matches!(self, Self::Mp4 | Self::Avi | Self::Mkv | Self::Mov)
    }

    /// Check if format is an archive
    pub fn is_archive(&self) -> bool {
        matches!(
            self,
            Self::Zip | Self::Tar | Self::TarGz | Self::TarBz2 | Self::SevenZ
        )
    }

    /// Get file extension
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Pdf => "pdf",
            Self::Docx => "docx",
            Self::Pptx => "pptx",
            Self::Xlsx => "xlsx",
            Self::Html => "html",
            Self::Xml => "xml",
            Self::Txt => "txt",
            Self::Markdown => "md",
            Self::Rtf => "rtf",
            Self::Odt => "odt",
            Self::Csv => "csv",
            Self::Tsv => "tsv",
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Tiff => "tiff",
            Self::Bmp => "bmp",
            Self::Gif => "gif",
            Self::Webp => "webp",
            Self::Mp3 => "mp3",
            Self::Wav => "wav",
            Self::M4a => "m4a",
            Self::Flac => "flac",
            Self::Mp4 => "mp4",
            Self::Avi => "avi",
            Self::Mkv => "mkv",
            Self::Mov => "mov",
            Self::Zip => "zip",
            Self::Tar => "tar",
            Self::TarGz => "tar.gz",
            Self::TarBz2 => "tar.bz2",
            Self::SevenZ => "7z",
            Self::Unknown => "bin",
        }
    }
}

/// Output format options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OutputFormat {
    /// Markdown output
    Markdown {
        /// Split output by pages
        split_pages: bool,
        /// Optimize for LLM processing
        optimize_for_llm: bool,
    },

    /// Image output
    Image {
        /// Image format
        format: ImageFormat,
        /// Quality (1-100)
        quality: u8,
        /// DPI for rendering
        dpi: u32,
    },

    /// JSON output
    Json {
        /// Include structured data
        structured: bool,
        /// Include metadata
        include_metadata: bool,
    },

    /// CSV output (for spreadsheets)
    Csv {
        /// CSV delimiter
        delimiter: char,
        /// Include headers
        include_headers: bool,
    },

    /// Embedding-ready output (optimized chunks)
    EmbeddingReady {
        /// Maximum chunk size in tokens
        max_chunk_size: usize,
        /// Overlap between chunks in tokens
        overlap: usize,
    },
}

/// Image format for image output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFormat {
    /// PNG format
    Png,
    /// JPEG format
    Jpeg,
    /// WebP format
    Webp,
}

/// Image quality levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageQuality {
    /// Low quality (small size)
    Low,
    /// Medium quality (balanced)
    Medium,
    /// High quality (large size)
    High,
}

/// Conversion options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionOptions {
    // Output control
    /// Split output by pages
    pub split_pages: bool,
    /// Optimize for LLM processing
    pub optimize_for_llm: bool,
    /// Maximum chunk size in tokens
    pub max_chunk_size: usize,

    // Quality settings
    /// Image quality level
    pub image_quality: ImageQuality,
    /// DPI for image output
    pub dpi: u32,
    /// OCR language(s) (e.g., "eng", "eng+por")
    pub ocr_language: String,

    // Processing options
    /// Preserve document layout
    pub preserve_layout: bool,
    /// Extract tables separately
    pub extract_tables: bool,
    /// Extract embedded images
    pub extract_images: bool,
    /// Include document metadata
    pub include_metadata: bool,

    // Optimization
    /// Compression level (0-9)
    pub compression_level: u8,
    /// Remove headers and footers
    pub remove_headers_footers: bool,
    /// Remove watermarks
    pub remove_watermarks: bool,
    /// Normalize whitespace
    pub normalize_whitespace: bool,
    
    // Precision mode
    /// Use high-precision mode (Docling-based layout analysis)
    /// When enabled: ~95% similarity, slower (uses Python/ML)
    /// When disabled (default): ~81% similarity, 250x faster (pure Rust)
    pub use_precision_mode: bool,
    
    /// Use docling-parse C++ FFI for maximum precision (95%+ similarity)
    /// Requires compilation with --features docling-ffi
    pub use_ffi: bool,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            split_pages: false,
            optimize_for_llm: true,
            max_chunk_size: 2048,
            image_quality: ImageQuality::Medium,
            dpi: 150,
            ocr_language: "eng".to_string(),
            preserve_layout: true,
            extract_tables: true,
            extract_images: true,
            include_metadata: true,
            compression_level: 6,
            remove_headers_footers: true,
            remove_watermarks: false,
            normalize_whitespace: true,
            use_precision_mode: false, // Fast mode by default (pure Rust, 250x faster)
            use_ffi: false, // C++ FFI disabled by default
        }
    }
}

/// Result of a conversion operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    /// Input file path
    pub input_path: PathBuf,
    /// Input file format
    pub input_format: FileFormat,
    /// Output format
    pub output_format: OutputFormat,
    /// Converted content
    pub content: Vec<ConversionOutput>,
    /// Document metadata
    pub metadata: DocumentMetadata,
    /// Conversion statistics
    pub statistics: ConversionStatistics,
}

impl ConversionResult {
    /// Get the number of pages/items converted
    pub fn page_count(&self) -> usize {
        self.content.len()
    }

    /// Get input file size in bytes
    pub fn input_size(&self) -> u64 {
        self.statistics.input_size_bytes
    }

    /// Get output size in bytes
    pub fn output_size(&self) -> u64 {
        self.statistics.output_size_bytes
    }

    /// Get conversion duration
    pub fn duration(&self) -> Duration {
        self.statistics.duration
    }

    /// Get chunk count (for embedding-ready output)
    pub fn chunk_count(&self) -> usize {
        self.content.iter().map(|o| o.chunks()).sum()
    }

    /// Save to file(s)
    pub async fn save<P: AsRef<Path>>(&self, output_path: P) -> crate::Result<()> {
        let output_path = output_path.as_ref();

        if self.content.len() == 1 {
            // Single file output
            tokio::fs::write(output_path, &self.content[0].data).await?;
        } else {
            // Multiple files (e.g., split by pages)
            // Create a directory with the output name
            let stem = output_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");
            let parent = output_path.parent().unwrap_or_else(|| Path::new("."));
            let output_dir = parent.join(stem);
            
            // Create directory
            tokio::fs::create_dir_all(&output_dir).await?;

            let ext = output_path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("md");

            // Save each page
            for (i, output) in self.content.iter().enumerate() {
                let file_path = output_dir.join(format!("page_{:04}.{}", i + 1, ext));
                tokio::fs::write(&file_path, &output.data).await?;
            }
            
            // Save metadata JSON
            let metadata_json = serde_json::json!({
                "input": self.input_path.to_string_lossy(),
                "format": format!("{:?}", self.input_format),
                "pages": self.content.len(),
                "metadata": {
                    "title": self.metadata.title,
                    "author": self.metadata.author,
                    "page_count": self.metadata.page_count,
                    "created": self.metadata.created,
                    "modified": self.metadata.modified,
                },
                "statistics": {
                    "input_size_bytes": self.statistics.input_size_bytes,
                    "output_size_bytes": self.statistics.output_size_bytes,
                    "duration_ms": self.statistics.duration.as_millis(),
                    "pages_processed": self.statistics.pages_processed,
                    "tables_extracted": self.statistics.tables_extracted,
                }
            });
            
            let metadata_path = output_dir.join("metadata.json");
            let json_str = serde_json::to_string_pretty(&metadata_json)
                .map_err(|e| crate::TransmutationError::SerializationError(e))?;
            tokio::fs::write(&metadata_path, json_str).await?;
        }

        Ok(())
    }
}

/// Single conversion output (page, slide, or complete document)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionOutput {
    /// Page/slide number (0 for full document)
    pub page_number: usize,
    /// Output data
    pub data: Vec<u8>,
    /// Output metadata
    pub metadata: OutputMetadata,
}

impl ConversionOutput {
    /// Get number of chunks in this output
    pub fn chunks(&self) -> usize {
        self.metadata.chunk_count
    }
}

/// Metadata for a single output
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OutputMetadata {
    /// Size in bytes
    pub size_bytes: u64,
    /// Number of chunks (for embedding-ready output)
    pub chunk_count: usize,
    /// Token count (if calculated)
    pub token_count: Option<usize>,
}

/// Document metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// Document title
    pub title: Option<String>,
    /// Author
    pub author: Option<String>,
    /// Creation date
    pub created: Option<String>,
    /// Modification date
    pub modified: Option<String>,
    /// Page count
    pub page_count: usize,
    /// Language
    pub language: Option<String>,
    /// Custom metadata
    pub custom: std::collections::HashMap<String, String>,
}

/// Conversion statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionStatistics {
    /// Input file size in bytes
    pub input_size_bytes: u64,
    /// Output size in bytes
    pub output_size_bytes: u64,
    /// Conversion duration
    pub duration: Duration,
    /// Number of pages processed
    pub pages_processed: usize,
    /// Number of tables extracted
    pub tables_extracted: usize,
    /// Number of images extracted
    pub images_extracted: usize,
    /// Cache hit
    pub cache_hit: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_format_categories() {
        assert!(FileFormat::Pdf.is_document());
        assert!(FileFormat::Png.is_image());
        assert!(FileFormat::Mp3.is_audio());
        assert!(FileFormat::Mp4.is_video());
        assert!(FileFormat::Zip.is_archive());
    }

    #[test]
    fn test_file_format_extension() {
        assert_eq!(FileFormat::Pdf.extension(), "pdf");
        assert_eq!(FileFormat::Docx.extension(), "docx");
        assert_eq!(FileFormat::Jpeg.extension(), "jpg");
    }

    #[test]
    fn test_default_conversion_options() {
        let opts = ConversionOptions::default();
        assert!(opts.optimize_for_llm);
        assert_eq!(opts.max_chunk_size, 2048);
        assert_eq!(opts.dpi, 150);
    }

    #[test]
    fn test_output_format() {
        let md = OutputFormat::Markdown {
            split_pages: true,
            optimize_for_llm: true,
        };
        assert!(matches!(md, OutputFormat::Markdown { .. }));
    }
}

