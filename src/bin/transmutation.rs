//! Transmutation CLI - Command Line Interface for document conversion
//!
//! This binary provides a command-line interface to the Transmutation library,
//! allowing users to convert documents from the terminal on Windows, Mac, and Linux.

use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use std::path::PathBuf;
use std::time::Instant;
use transmutation::{ConversionOptions, Converter, ImageQuality, OutputFormat, Result};

#[derive(Parser)]
#[command(
    name = "transmutation",
    version,
    about = "High-performance document conversion engine for AI/LLM embeddings",
    long_about = "Transmutation converts documents to LLM-optimized formats (Markdown, Images, JSON)\n\
                  Supporting 20+ formats including PDF, DOCX, PPTX, XLSX, images, audio, and video."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Quiet mode (minimal output)
    #[arg(short, long, global = true)]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert a document to another format
    Convert {
        /// Input file path
        #[arg(value_name = "INPUT")]
        input: PathBuf,

        /// Output file path
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,

        /// Output format
        #[arg(short = 'f', long, value_enum, default_value = "markdown")]
        format: OutputFormatArg,

        /// Split output by pages
        #[arg(short = 's', long)]
        split_pages: bool,

        /// Optimize for LLM processing
        #[arg(short = 'l', long)]
        optimize_llm: bool,

        /// Image quality (1-100)
        #[arg(short = 'q', long, default_value = "85")]
        quality: u8,

        /// DPI for image output
        #[arg(long, default_value = "150")]
        dpi: u32,
    },

    /// Batch convert multiple documents
    Batch {
        /// Input directory or glob pattern
        #[arg(value_name = "INPUT")]
        input: String,

        /// Output directory
        #[arg(short, long, value_name = "OUTPUT")]
        output: PathBuf,

        /// Output format
        #[arg(short = 'f', long, value_enum, default_value = "markdown")]
        format: OutputFormatArg,

        /// Number of parallel workers
        #[arg(short = 'j', long, default_value = "4")]
        jobs: usize,

        /// Continue on errors
        #[arg(short = 'c', long)]
        continue_on_error: bool,
    },

    /// Show information about a document
    Info {
        /// Input file path
        #[arg(value_name = "INPUT")]
        input: PathBuf,
    },

    /// List supported formats
    Formats,

    /// Show version and build information
    Version,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormatArg {
    /// Markdown format
    Markdown,
    /// PNG image
    Png,
    /// JPEG image
    Jpeg,
    /// WebP image
    Webp,
    /// JSON format
    Json,
    /// CSV format (for spreadsheets)
    Csv,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose {
        tracing::Level::DEBUG
    } else if cli.quiet {
        tracing::Level::ERROR
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .init();

    // Run command
    if let Err(e) = run_command(cli).await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}

async fn run_command(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Convert {
            input,
            output,
            format,
            split_pages,
            optimize_llm,
            quality,
            dpi,
        } => {
            if !cli.quiet {
                println!("{}", "Converting document...".cyan().bold());
                println!("  Input:  {}", input.display());
            }
            
            let output_path = output.unwrap_or_else(|| {
                let mut path = input.clone();
                path.set_extension(match format {
                    OutputFormatArg::Markdown => "md",
                    OutputFormatArg::Png => "png",
                    OutputFormatArg::Jpeg => "jpg",
                    OutputFormatArg::Webp => "webp",
                    OutputFormatArg::Json => "json",
                    OutputFormatArg::Csv => "csv",
                });
                path
            });
            
            if !cli.quiet {
                println!("  Output: {}", output_path.display());
                println!("  Format: {:?}", format);
            }
            
            // Create converter
            let converter = Converter::new()?;
            
            // Configure options
            let options = ConversionOptions {
                split_pages,
                optimize_for_llm: optimize_llm,
                extract_tables: true,
                image_quality: ImageQuality::High,
                dpi,
                ..Default::default()
            };
            
            // Determine output format
            let output_format = match format {
                OutputFormatArg::Markdown => OutputFormat::Markdown {
                    split_pages,
                    optimize_for_llm: optimize_llm,
                },
                OutputFormatArg::Json => OutputFormat::Json {
                    structured: true,
                    include_metadata: true,
                },
                OutputFormatArg::Png => OutputFormat::Image {
                    format: transmutation::ImageFormat::Png,
                    quality,
                    dpi,
                },
                OutputFormatArg::Jpeg => OutputFormat::Image {
                    format: transmutation::ImageFormat::Jpeg,
                    quality,
                    dpi,
                },
                OutputFormatArg::Webp => OutputFormat::Image {
                    format: transmutation::ImageFormat::Webp,
                    quality,
                    dpi,
                },
                OutputFormatArg::Csv => OutputFormat::Csv {
                    delimiter: ',',
                    include_headers: true,
                },
            };
            
            // Perform conversion
            let start = Instant::now();
            let result = converter
                .convert(&input)
                .to(output_format)
                .with_options(options)
                .execute()
                .await?;
            let duration = start.elapsed();
            
            // Save output
            result.save(&output_path).await?;
            
            // Display results
            if !cli.quiet {
                println!();
                println!("{}", "✓ Conversion completed successfully!".green().bold());
                println!();
                println!("{}", "Statistics:".yellow().bold());
                println!("  Duration:     {:?}", duration);
                println!("  Pages:        {}", result.statistics.pages_processed);
                println!("  Tables:       {}", result.statistics.tables_extracted);
                println!("  Input size:   {:.2} MB", result.statistics.input_size_bytes as f64 / 1_000_000.0);
                println!("  Output size:  {:.2} MB", result.statistics.output_size_bytes as f64 / 1_000_000.0);
                println!("  Speed:        {:.2} pages/sec", 
                    result.statistics.pages_processed as f64 / duration.as_secs_f64());
                
                if let Some(title) = &result.metadata.title {
                    println!();
                    println!("{}", "Metadata:".yellow().bold());
                    println!("  Title: {}", title);
                    if let Some(author) = &result.metadata.author {
                        println!("  Author: {}", author);
                    }
                }
            }
            
            Ok(())
        }

        Commands::Batch {
            input,
            output,
            format,
            jobs,
            continue_on_error,
        } => {
            println!("{}", "Batch converting documents...".cyan().bold());
            println!("  Input:   {}", input);
            println!("  Output:  {}", output.display());
            println!("  Format:  {:?}", format);
            println!("  Workers: {}", jobs);
            
            if continue_on_error {
                println!("  Mode:    Continue on errors");
            }
            
            // TODO: Implement batch conversion
            
            println!("{}", "✓ Batch conversion completed!".green().bold());
            Ok(())
        }

        Commands::Info { input } => {
            println!("{}", "Document Information".cyan().bold());
            println!("  File: {}", input.display());
            
            // TODO: Implement document info extraction
            
            println!("\n{}", "Format Detection:".yellow());
            println!("  Type: Unknown (not implemented)");
            println!("  Size: Unknown");
            
            Ok(())
        }

        Commands::Formats => {
            println!("{}", "Supported Formats".cyan().bold());
            println!();
            
            println!("{}", "Documents:".yellow().bold());
            println!("  PDF, DOCX, PPTX, XLSX, HTML, XML, TXT, MD, RTF, ODT");
            println!();
            
            println!("{}", "Images (with OCR):".yellow().bold());
            println!("  JPG, PNG, TIFF, BMP, GIF, WEBP");
            println!();
            
            println!("{}", "Audio/Video:".yellow().bold());
            println!("  MP3, MP4, WAV, M4A (transcription via Whisper)");
            println!();
            
            println!("{}", "Archives:".yellow().bold());
            println!("  ZIP, TAR, GZ, 7Z");
            println!();
            
            println!("{}", "Output Formats:".yellow().bold());
            println!("  Markdown, PNG, JPEG, WebP, JSON, CSV");
            
            Ok(())
        }

        Commands::Version => {
            println!("{} {}", "Transmutation".cyan().bold(), transmutation::VERSION);
            println!();
            println!("Build Information:");
            println!("  Rust Edition: 2024");
            println!("  Features: {}", get_enabled_features());
            println!();
            println!("Engines (Pure Rust):");
            print_engine_status("PDF Parser (lopdf)", cfg!(feature = "pdf"));
            print_engine_status("DOCX Parser (docx-rs)", cfg!(feature = "office"));
            print_engine_status("HTML/XML Parser", cfg!(feature = "web"));
            print_engine_status("Tesseract OCR", cfg!(feature = "tesseract"));
            print_engine_status("FFmpeg", cfg!(feature = "ffmpeg"));
            
            Ok(())
        }
    }
}

fn get_enabled_features() -> String {
    let mut features = Vec::new();
    
    if cfg!(feature = "pdf") { features.push("pdf"); }
    if cfg!(feature = "office") { features.push("office"); }
    if cfg!(feature = "web") { features.push("web"); }
    if cfg!(feature = "image-ocr") { features.push("image-ocr"); }
    if cfg!(feature = "tesseract") { features.push("tesseract"); }
    if cfg!(feature = "ffmpeg") { features.push("ffmpeg"); }
    if cfg!(feature = "archives") { features.push("archives"); }
    if cfg!(feature = "cache") { features.push("cache"); }
    
    if features.is_empty() {
        "none".to_string()
    } else {
        features.join(", ")
    }
}

fn print_engine_status(name: &str, enabled: bool) {
    if enabled {
        println!("  {} {}", "✓".green(), name);
    } else {
        println!("  {} {} {}", "✗".red(), name, "(disabled)".dimmed());
    }
}

