//! Batch processing for multiple documents
//!
//! Provides parallel processing capabilities using Rayon.

use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::{
    ConversionOptions, ConversionResult, Converter, OutputFormat, Result, TransmutationError,
};

/// Batch processor for multiple documents
#[derive(Debug)]
pub struct BatchProcessor {
    files: Vec<PathBuf>,
    output_format: OutputFormat,
    options: ConversionOptions,
    parallel_jobs: usize,
}

impl BatchProcessor {
    /// Create a new batch processor
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            output_format: OutputFormat::Markdown {
                split_pages: false,
                optimize_for_llm: true,
            },
            options: ConversionOptions::default(),
            parallel_jobs: num_cpus::get(),
        }
    }

    /// Add a single file to the batch
    pub fn add_file<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.files.push(path.as_ref().to_path_buf());
        self
    }

    /// Add multiple files to the batch
    pub fn add_files<P: AsRef<Path>>(mut self, paths: &[P]) -> Self {
        for path in paths {
            self.files.push(path.as_ref().to_path_buf());
        }
        self
    }

    /// Set output format for all conversions
    pub fn output_format(mut self, format: OutputFormat) -> Self {
        self.output_format = format;
        self
    }

    /// Set conversion options
    pub fn options(mut self, options: ConversionOptions) -> Self {
        self.options = options;
        self
    }

    /// Set number of parallel jobs
    pub fn parallel(mut self, jobs: usize) -> Self {
        self.parallel_jobs = jobs.max(1);
        self
    }

    /// Execute batch conversion
    pub async fn execute(self) -> Result<BatchResult> {
        let start_time = Instant::now();
        let total_files = self.files.len();

        eprintln!("🚀 Starting batch conversion...");
        eprintln!("   Files: {}", total_files);
        eprintln!("   Concurrent jobs: {}", self.parallel_jobs);
        eprintln!("   Output format: {:?}", self.output_format);
        eprintln!();

        let output_format = self.output_format.clone();
        let options = self.options.clone();

        // Process files concurrently using Tokio
        let mut tasks = Vec::new();

        for file in self.files {
            let output_format = output_format.clone();
            let options = options.clone();

            let task = tokio::spawn(async move {
                let result = match Converter::new() {
                    Ok(converter) => {
                        converter
                            .convert(&file)
                            .to(output_format)
                            .with_options(options)
                            .execute()
                            .await
                    }
                    Err(e) => Err(e),
                };

                (file, result)
            });

            tasks.push(task);
        }

        // Wait for all tasks to complete
        let results = futures::future::join_all(tasks).await;

        let total_time = start_time.elapsed();

        // Collect results
        let mut successes = Vec::new();
        let mut failures = Vec::new();

        for task_result in results {
            match task_result {
                Ok((file, conversion_result)) => match conversion_result {
                    Ok(conversion) => successes.push((file, conversion)),
                    Err(e) => failures.push((file, e)),
                },
                Err(join_error) => {
                    eprintln!("Task join error: {}", join_error);
                }
            }
        }

        eprintln!();
        eprintln!("✅ Batch conversion complete!");
        eprintln!("   Success: {}/{}", successes.len(), total_files);
        eprintln!("   Failed: {}", failures.len());
        eprintln!("   Total time: {:.2}s", total_time.as_secs_f64());

        Ok(BatchResult {
            successes,
            failures,
            total_files,
            total_time,
        })
    }
}

impl Default for BatchProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of batch processing
#[derive(Debug)]
pub struct BatchResult {
    /// Successfully converted files
    pub successes: Vec<(PathBuf, ConversionResult)>,
    /// Failed conversions
    pub failures: Vec<(PathBuf, TransmutationError)>,
    /// Total number of files processed
    pub total_files: usize,
    /// Total processing time
    pub total_time: std::time::Duration,
}

impl BatchResult {
    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_files == 0 {
            0.0
        } else {
            (self.successes.len() as f64 / self.total_files as f64) * 100.0
        }
    }

    /// Get total pages processed
    pub fn total_pages(&self) -> usize {
        self.successes
            .iter()
            .map(|(_, result)| result.metadata.page_count)
            .sum()
    }

    /// Get average processing speed (pages/second)
    pub fn pages_per_second(&self) -> f64 {
        let total_pages = self.total_pages() as f64;
        let total_secs = self.total_time.as_secs_f64();

        if total_secs > 0.0 {
            total_pages / total_secs
        } else {
            0.0
        }
    }

    /// Save all successful conversions to a directory
    pub async fn save_all<P: AsRef<Path>>(&self, output_dir: P) -> Result<()> {
        let output_dir = output_dir.as_ref();
        tokio::fs::create_dir_all(output_dir).await?;

        for (input_path, result) in &self.successes {
            let filename = input_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");

            let extension = match result.output_format {
                OutputFormat::Markdown { .. } => "md",
                OutputFormat::Json { .. } => "json",
                OutputFormat::Image { .. } => "png",
                _ => "txt",
            };

            let output_path = output_dir.join(format!("{}.{}", filename, extension));

            if let Some(output) = result.content.first() {
                tokio::fs::write(&output_path, &output.data).await?;
            }
        }

        Ok(())
    }
}
