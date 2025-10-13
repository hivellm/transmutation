//! Archive converter implementation
//!
//! Extracts and converts documents from archives (ZIP, TAR, 7Z, etc.).

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::types::{ConversionOptions, ConversionResult, ConversionOutput, FileFormat, OutputFormat, OutputMetadata};
use crate::utils::file_detect;
use crate::Result;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tokio::fs;
use zip::ZipArchive;
use std::io::{Cursor, Read};
use std::collections::HashMap;

#[cfg(feature = "archives-extended")]
use tar::Archive as TarArchive;
#[cfg(feature = "archives-extended")]
use flate2::read::GzDecoder;

/// Archive to document converter
pub struct ArchiveConverter;

impl ArchiveConverter {
    /// Create a new archive converter
    pub fn new() -> Self {
        Self
    }
    
    /// List files in ZIP archive
    async fn list_zip_files(&self, archive_path: &Path) -> Result<Vec<(String, u64)>> {
        let data = fs::read(archive_path).await?;
        let cursor = Cursor::new(data);
        let mut archive = ZipArchive::new(cursor)?;
        
        let mut files = Vec::new();
        for i in 0..archive.len() {
            let file = archive.by_index(i)?;
            if !file.is_dir() {
                files.push((file.name().to_string(), file.size()));
            }
        }
        
        Ok(files)
    }
    
    /// List files in TAR archive
    #[cfg(feature = "archives-extended")]
    async fn list_tar_files(&self, archive_path: &Path, is_gzipped: bool) -> Result<Vec<(String, u64)>> {
        let data = fs::read(archive_path).await?;
        let cursor = Cursor::new(data);
        
        let mut files = Vec::new();
        
        if is_gzipped {
            let decoder = GzDecoder::new(cursor);
            let mut archive = TarArchive::new(decoder);
            
            for entry in archive.entries()? {
                let entry = entry?;
                let path = entry.path()?;
                if !entry.header().entry_type().is_dir() {
                    files.push((
                        path.display().to_string(),
                        entry.header().size()?
                    ));
                }
            }
        } else {
            let mut archive = TarArchive::new(cursor);
            
            for entry in archive.entries()? {
                let entry = entry?;
                let path = entry.path()?;
                if !entry.header().entry_type().is_dir() {
                    files.push((
                        path.display().to_string(),
                        entry.header().size()?
                    ));
                }
            }
        }
        
        Ok(files)
    }
    
    /// List files in archive (auto-detect type)
    async fn list_archive_files(&self, archive_path: &Path, format: FileFormat) -> Result<Vec<(String, u64)>> {
        match format {
            FileFormat::Zip => {
                self.list_zip_files(archive_path).await
            },
            #[cfg(feature = "archives-extended")]
            FileFormat::Tar => {
                self.list_tar_files(archive_path, false).await
            },
            #[cfg(feature = "archives-extended")]
            FileFormat::TarGz => {
                self.list_tar_files(archive_path, true).await
            },
            _ => {
                Err(crate::TransmutationError::UnsupportedFormat(
                    format!("Archive format {:?} not yet supported", format)
                ))
            }
        }
    }
    
    /// Generate archive index in Markdown
    fn generate_markdown_index(&self, files: &[(String, u64)], archive_name: &str) -> String {
        let mut markdown = String::new();
        markdown.push_str(&format!("# Archive: {}\n\n", archive_name));
        markdown.push_str(&format!("**Total files**: {}\n\n", files.len()));
        
        // Calculate total size
        let total_size: u64 = files.iter().map(|(_, size)| size).sum();
        markdown.push_str(&format!("**Total size**: {} bytes ({:.2} MB)\n\n", 
            total_size, 
            total_size as f64 / 1_048_576.0
        ));
        
        // Group by extension
        let mut by_extension: HashMap<String, Vec<&(String, u64)>> = HashMap::new();
        for file_info in files {
            let ext = Path::new(&file_info.0)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("no extension")
                .to_lowercase();
            by_extension.entry(ext).or_insert_with(Vec::new).push(file_info);
        }
        
        markdown.push_str("## Files by Type\n\n");
        let mut extensions: Vec<_> = by_extension.keys().collect();
        extensions.sort();
        
        for ext in extensions {
            let files = &by_extension[ext];
            markdown.push_str(&format!("### .{} ({} files)\n\n", ext, files.len()));
            
            for (name, size) in files.iter() {
                markdown.push_str(&format!("- `{}` ({} bytes)\n", name, size));
            }
            markdown.push('\n');
        }
        
        markdown.push_str("## All Files\n\n");
        markdown.push_str("| File | Size (bytes) | Size (MB) |\n");
        markdown.push_str("|------|-------------:|----------:|\n");
        
        for (name, size) in files {
            markdown.push_str(&format!(
                "| `{}` | {} | {:.2} |\n",
                name,
                size,
                *size as f64 / 1_048_576.0
            ));
        }
        
        markdown
    }
    
    /// Generate archive index in JSON
    fn generate_json_index(&self, files: &[(String, u64)], archive_name: &str) -> Result<String> {
        let total_size: u64 = files.iter().map(|(_, size)| size).sum();
        
        // Group by extension
        let mut by_extension: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
        for (name, size) in files {
            let ext = Path::new(name)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("no extension")
                .to_lowercase();
            
            by_extension.entry(ext.clone()).or_insert_with(Vec::new).push(serde_json::json!({
                "name": name,
                "size": size,
                "extension": ext,
            }));
        }
        
        let json = serde_json::json!({
            "archive": {
                "name": archive_name,
                "total_files": files.len(),
                "total_size_bytes": total_size,
                "total_size_mb": total_size as f64 / 1_048_576.0,
            },
            "files_by_type": by_extension,
            "all_files": files.iter().map(|(name, size)| serde_json::json!({
                "name": name,
                "size": size,
            })).collect::<Vec<_>>(),
        });
        
        Ok(serde_json::to_string_pretty(&json)?)
    }
}

impl Default for ArchiveConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentConverter for ArchiveConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![FileFormat::Zip, FileFormat::Tar, FileFormat::SevenZ]
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
        let archive_name = input.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("archive");
        
        // Detect archive format
        let input_format = file_detect::detect_format(input).await?;
        
        eprintln!("🔄 Archive Processing (Pure Rust)");
        eprintln!("   Archive ({:?}) → List Files → {:?}", input_format, output_format);
        eprintln!();
        
        // List files in archive
        let files = self.list_archive_files(input, input_format).await?;
        eprintln!("📦 Found {} files in archive", files.len());
        
        // Convert to requested format
        let output_data = match output_format {
            OutputFormat::Markdown { .. } => {
                eprintln!("📝 Generating Markdown index...");
                let markdown = self.generate_markdown_index(&files, archive_name);
                markdown.into_bytes()
            },
            OutputFormat::Json { .. } => {
                eprintln!("📝 Generating JSON index...");
                let json = self.generate_json_index(&files, archive_name)?;
                json.into_bytes()
            },
            _ => {
                return Err(crate::TransmutationError::UnsupportedFormat(
                    format!("Output format {:?} not supported for archives", output_format)
                ));
            }
        };
        
        let output_size = output_data.len() as u64;
        let input_size = fs::metadata(input).await?.len();
        
        eprintln!("✅ Archive index generated!");
        
        Ok(ConversionResult {
            input_path: input.to_path_buf(),
            input_format,
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
                title: Some(archive_name.to_string()),
                author: None,
                created: None,
                modified: None,
                page_count: 1,
                language: None,
                custom: {
                    let mut custom = std::collections::HashMap::new();
                    custom.insert("file_count".to_string(), files.len().to_string());
                    custom
                },
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

    fn metadata(&self) -> ConverterMetadata {
        ConverterMetadata {
            name: "Archive Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Archive to document index converter (ZIP, TAR, 7Z)".to_string(),
            external_deps: vec![],
        }
    }
}
