//! Video converter with audio extraction and transcription
//!
//! Converts video files to text by extracting audio and using Whisper ASR.

use std::path::{Path, PathBuf};
use std::process::Command;

use async_trait::async_trait;
use tempfile::NamedTempFile;
use tokio::fs;

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::Result;
use crate::types::{
    ConversionOptions, ConversionOutput, ConversionResult, FileFormat, OutputFormat, OutputMetadata,
};

/// Video to text converter (FFmpeg + Whisper)
pub struct VideoConverter;

impl VideoConverter {
    /// Create a new video converter
    pub fn new() -> Self {
        Self
    }

    /// Check if ffmpeg is available
    fn check_ffmpeg() -> bool {
        Command::new("ffmpeg").arg("-version").output().is_ok()
    }

    /// Check if whisper is available
    fn check_whisper() -> bool {
        // Try whisper in PATH
        if Command::new("whisper").arg("--help").output().is_ok() {
            return true;
        }

        // Try common installation paths
        let paths = vec![
            format!(
                "{}/.local/bin/whisper",
                std::env::var("HOME").unwrap_or_default()
            ),
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
            format!(
                "{}/.local/bin/whisper",
                std::env::var("HOME").unwrap_or_default()
            ),
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

    /// Extract audio from video using FFmpeg
    async fn extract_audio(&self, video_path: &Path) -> Result<PathBuf> {
        if !Self::check_ffmpeg() {
            return Err(crate::TransmutationError::conversion_failed(
                "FFmpeg not found. Install: sudo apt-get install ffmpeg",
            ));
        }

        // Create temporary audio file
        let temp_audio = NamedTempFile::new().map_err(|e| {
            crate::TransmutationError::conversion_failed(&format!(
                "Failed to create temp file: {}",
                e
            ))
        })?;

        let audio_path = temp_audio.path().with_extension("wav");

        eprintln!("🎬 Extracting audio with FFmpeg...");

        // Extract audio to WAV format
        let output = Command::new("ffmpeg")
            .arg("-i")
            .arg(video_path)
            .arg("-vn") // No video
            .arg("-acodec")
            .arg("pcm_s16le") // WAV format
            .arg("-ar")
            .arg("16000") // 16kHz sample rate (Whisper default)
            .arg("-ac")
            .arg("1") // Mono
            .arg("-y") // Overwrite
            .arg(&audio_path)
            .output()
            .map_err(|e| {
                crate::TransmutationError::conversion_failed(&format!(
                    "FFmpeg execution failed: {}",
                    e
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(crate::TransmutationError::conversion_failed(&format!(
                "FFmpeg failed: {}",
                stderr
            )));
        }

        Ok(audio_path)
    }

    /// Transcribe audio using Whisper
    async fn transcribe_audio(&self, audio_path: &Path, language: Option<&str>) -> Result<String> {
        if !Self::check_whisper() {
            return Err(crate::TransmutationError::conversion_failed(
                "Whisper not found. Install: pip install openai-whisper",
            ));
        }

        eprintln!("🎤 Running Whisper transcription...");

        let whisper_cmd = Self::get_whisper_cmd();
        let mut cmd = Command::new(&whisper_cmd);
        cmd.arg(audio_path);
        cmd.arg("--model").arg("base"); // Use base model
        cmd.arg("--output_format").arg("txt");
        cmd.arg("--output_dir").arg("/tmp");

        if let Some(lang) = language {
            cmd.arg("--language").arg(lang);
        }

        let output = cmd.output().map_err(|e| {
            crate::TransmutationError::conversion_failed(&format!(
                "Whisper execution failed: {}",
                e
            ))
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(crate::TransmutationError::conversion_failed(&format!(
                "Whisper failed: {}",
                stderr
            )));
        }

        // Read transcription
        let audio_name = audio_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("audio");
        let transcript_path = format!("/tmp/{}.txt", audio_name);

        let transcript = tokio::fs::read_to_string(&transcript_path)
            .await
            .map_err(|e| {
                crate::TransmutationError::conversion_failed(&format!(
                    "Failed to read transcript: {}",
                    e
                ))
            })?;

        // Clean up
        let _ = tokio::fs::remove_file(&transcript_path).await;

        Ok(transcript)
    }

    /// Convert video to Markdown
    async fn video_to_markdown(&self, video_path: &Path, language: Option<&str>) -> Result<String> {
        // Extract audio
        let audio_path = self.extract_audio(video_path).await?;

        // Transcribe
        let transcript = self.transcribe_audio(&audio_path, language).await?;

        // Clean up audio file
        let _ = tokio::fs::remove_file(&audio_path).await;

        let mut markdown = String::new();
        markdown.push_str("# Video Transcription\n\n");

        if let Some(lang) = language {
            markdown.push_str(&format!("**Language**: {}\n\n", lang));
        }

        markdown.push_str("## Transcript\n\n");
        markdown.push_str(&transcript);
        markdown.push('\n');

        Ok(markdown)
    }
}

impl Default for VideoConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentConverter for VideoConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![
            FileFormat::Mp4,
            FileFormat::Avi,
            FileFormat::Mkv,
            FileFormat::Mov,
            FileFormat::Webm,
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
        eprintln!("🔄 Video Transcription (FFmpeg + Whisper)");
        eprintln!("   Video → Audio → Whisper → {:?}", output_format);
        eprintln!();

        let language = None; // Auto-detect

        // Convert video to text
        let markdown = self.video_to_markdown(input, language).await?;

        // Convert to requested format
        let output_data = match output_format {
            OutputFormat::Markdown { .. } => {
                eprintln!("✅ Transcription complete!");
                markdown.into_bytes()
            }
            OutputFormat::Json { .. } => {
                eprintln!("📝 Converting to JSON...");
                let json = serde_json::json!({
                    "transcription": {
                        "text": markdown,
                        "language": language.unwrap_or("auto"),
                        "source": "video",
                    }
                });
                serde_json::to_string_pretty(&json)?.into_bytes()
            }
            _ => {
                return Err(crate::TransmutationError::UnsupportedFormat(format!(
                    "Output format {:?} not supported for video",
                    output_format
                )));
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
            name: "Video Transcription Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Video to text converter using FFmpeg + Whisper ASR".to_string(),
            external_deps: vec!["ffmpeg".to_string(), "whisper".to_string()],
        }
    }
}
