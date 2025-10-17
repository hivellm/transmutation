//! Audio converter with Whisper transcription
//!
//! Converts audio files to text using Whisper ASR.

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::types::{ConversionOptions, ConversionResult, ConversionOutput, FileFormat, OutputFormat, OutputMetadata};
use crate::Result;
use async_trait::async_trait;
use std::path::Path;
use std::process::Command;
use tokio::fs;

/// Audio to text converter (Whisper ASR)
pub struct AudioConverter;

impl AudioConverter {
    /// Create a new audio converter
    pub fn new() -> Self {
        Self
    }
    
    /// Check if whisper CLI is available
    fn check_whisper() -> bool {
        // Try whisper in PATH
        if Command::new("whisper").arg("--help").output().is_ok() {
            return true;
        }
        
        // Try common installation paths
        let paths = vec![
            format!("{}/.local/bin/whisper", std::env::var("HOME").unwrap_or_default()),
            "/usr/local/bin/whisper".to_string(),
            "/usr/bin/whisper".to_string(),
        ];
        
        for path in paths {
            if std::path::Path::new(&path).exists() {
                return true;
            }
        }
        
        false
    }
    
    /// Get whisper command path
    fn get_whisper_cmd() -> String {
        // Try common paths
        let paths = vec![
            format!("{}/.local/bin/whisper", std::env::var("HOME").unwrap_or_default()),
            "/usr/local/bin/whisper".to_string(),
            "/usr/bin/whisper".to_string(),
            "whisper".to_string(),
        ];
        
        for path in &paths {
            if std::path::Path::new(path).exists() || path == "whisper" {
                return path.clone();
            }
        }
        
        "whisper".to_string()
    }
    
    /// Transcribe audio using Whisper
    async fn transcribe_audio(&self, audio_path: &Path, language: Option<&str>) -> Result<String> {
        if !Self::check_whisper() {
            return Err(crate::TransmutationError::conversion_failed(
                "Whisper not found. Install: pip install openai-whisper (or pipx install openai-whisper)"
            ));
        }
        
        // Use Whisper CLI for transcription
        let whisper_cmd = Self::get_whisper_cmd();
        let mut cmd = Command::new(&whisper_cmd);
        cmd.arg(audio_path);
        cmd.arg("--model").arg("base");  // Use base model (fast, good quality)
        cmd.arg("--output_format").arg("txt");
        cmd.arg("--output_dir").arg("/tmp");
        
        if let Some(lang) = language {
            cmd.arg("--language").arg(lang);
        }
        
        eprintln!("📝 Running Whisper transcription...");
        let output = cmd.output()
            .map_err(|e| crate::TransmutationError::conversion_failed(&format!("Whisper execution failed: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(crate::TransmutationError::conversion_failed(&format!("Whisper failed: {}", stderr)));
        }
        
        // Read transcription from output file
        let audio_name = audio_path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("audio");
        let transcript_path = format!("/tmp/{}.txt", audio_name);
        
        let transcript = tokio::fs::read_to_string(&transcript_path).await
            .map_err(|e| crate::TransmutationError::conversion_failed(&format!("Failed to read transcript: {}", e)))?;
        
        // Clean up temp file
        let _ = tokio::fs::remove_file(&transcript_path).await;
        
        Ok(transcript)
    }
    
    /// Convert audio to Markdown
    async fn audio_to_markdown(&self, audio_path: &Path, language: Option<&str>) -> Result<String> {
        let transcript = self.transcribe_audio(audio_path, language).await?;
        
        let mut markdown = String::new();
        markdown.push_str("# Audio Transcription\n\n");
        
        if let Some(lang) = language {
            markdown.push_str(&format!("**Language**: {}\n\n", lang));
        }
        
        markdown.push_str("## Transcript\n\n");
        markdown.push_str(&transcript);
        markdown.push('\n');
        
        Ok(markdown)
    }
}

impl Default for AudioConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentConverter for AudioConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![
            FileFormat::Mp3,
            FileFormat::Wav,
            FileFormat::M4a,
            FileFormat::Ogg,
            FileFormat::Flac,
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
        _options: ConversionOptions,
    ) -> Result<ConversionResult> {
        eprintln!("🔄 Audio Transcription (Whisper)");
        eprintln!("   Audio → Whisper → {:?}", output_format);
        eprintln!();
        
        let language = None;  // Auto-detect (can be made configurable)
        
        // Convert audio to text
        let markdown = self.audio_to_markdown(input, language).await?;
        
        // Convert to requested format
        let output_data = match output_format {
            OutputFormat::Markdown { .. } => {
                eprintln!("✅ Transcription complete!");
                markdown.into_bytes()
            },
            OutputFormat::Json { .. } => {
                eprintln!("📝 Converting to JSON...");
                let json = serde_json::json!({
                    "transcription": {
                        "text": markdown,
                        "language": language.unwrap_or("auto"),
                    }
                });
                serde_json::to_string_pretty(&json)?.into_bytes()
            },
            _ => {
                return Err(crate::TransmutationError::UnsupportedFormat(
                    format!("Output format {:?} not supported for audio", output_format)
                ));
            }
        };
        
        let output_size = output_data.len() as u64;
        let input_size = fs::metadata(input).await?.len();
        
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
                language: language.map(|s| s.to_string()),
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

    fn metadata(&self) -> ConverterMetadata {
        ConverterMetadata {
            name: "Audio Transcription Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Audio to text converter using Whisper ASR".to_string(),
            external_deps: vec!["whisper".to_string(), "ffmpeg".to_string()],
        }
    }
}

