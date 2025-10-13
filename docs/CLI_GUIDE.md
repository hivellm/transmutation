# Transmutation CLI Guide

## Overview

Transmutation is available in two formats:
1. **CLI** - Command-line tool for Windows, Mac, and Linux
2. **Library** - Rust crate for integration into Rust projects

## Installation

### As CLI Tool

```bash
# Install globally
cargo install transmutation --features cli

# Or build from source
git clone https://github.com/hivellm/transmutation
cd transmutation
cargo build --release --features cli

# Add to PATH or copy to system bin
cp target/release/transmutation /usr/local/bin/
```

### As Library

```toml
# Add to your Cargo.toml
[dependencies]
transmutation = "0.1"
```

## CLI Usage

### Basic Commands

```bash
# Show help
transmutation --help

# Show version and enabled features
transmutation version

# List supported formats
transmutation formats

# Convert a document
transmutation convert input.pdf

# Convert with specific output
transmutation convert input.pdf -o output.md

# Convert with options
transmutation convert input.pdf -f markdown --optimize-llm

# Batch convert directory
transmutation batch ./documents/ -o ./output/

# Show document information
transmutation info document.pdf
```

### Convert Command

```bash
transmutation convert <INPUT> [OPTIONS]

Options:
  -o, --output <FILE>      Output file path
  -f, --format <FORMAT>    Output format [markdown|png|jpeg|webp|json|csv]
  -s, --split-pages        Split output by pages (creates multiple files)
  -d, --output-dir <DIR>   Output directory for split pages/images
  -l, --optimize-llm       Optimize for LLM processing
  -q, --quality <1-100>    Image quality (default: 85)
      --dpi <DPI>          DPI for image output (default: 150)
  -v, --verbose            Enable verbose output
  -q, --quiet              Quiet mode (minimal output)
```

### Examples

#### Convert PDF to Markdown
```bash
# Basic conversion
transmutation convert document.pdf

# With LLM optimization
transmutation convert document.pdf --optimize-llm

# Split into pages (one MD per page) - NEW!
transmutation convert document.pdf --precision --split-pages \
  --output-dir data/pages -o document.md
# Creates: data/pages/document_1.md, document_2.md, ..., document_N.md

# Split into images (one PNG per page) - NEW!
transmutation convert document.pdf --format png --dpi 150 \
  --output-dir data/images -o document.png
# Creates: data/images/document_1.png, document_2.png, ..., document_N.png

# High quality single file
transmutation convert document.pdf -f png --dpi 300 --quality 95
```

#### Convert Office Documents
```bash
# DOCX to Markdown
transmutation convert presentation.docx -o output.md

# PPTX to images (one per slide)
transmutation convert slides.pptx -f png --split-pages

# XLSX to CSV
transmutation convert spreadsheet.xlsx -f csv
```

#### Batch Processing
```bash
# Convert all PDFs in a directory
transmutation batch ./pdfs/ -o ./markdown/

# Parallel processing with 8 workers
transmutation batch ./documents/ -o ./output/ -j 8

# Continue on errors
transmutation batch ./input/ -o ./output/ --continue-on-error

# Specific format
transmutation batch ./images/ -o ./text/ -f markdown
```

#### OCR from Images
```bash
# Single image
transmutation convert scan.png -f markdown

# Batch OCR
transmutation batch ./scans/ -o ./text/ -f markdown
```

#### Audio/Video Transcription
```bash
# Audio file
transmutation convert podcast.mp3 -f markdown

# Video file (extracts audio and transcribes)
transmutation convert video.mp4 -f markdown

# Multiple files
transmutation batch ./audio/ -o ./transcripts/
```

## Library Usage

### Basic Example

```rust
use transmutation::{Converter, OutputFormat, ConversionOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize converter
    let converter = Converter::new()?;
    
    // Convert PDF to Markdown
    let result = converter
        .convert("document.pdf")
        .to(OutputFormat::Markdown { 
            split_pages: true, 
            optimize_for_llm: true 
        })
        .execute()
        .await?;
    
    // Save output
    result.save("output/document.md").await?;
    
    println!("Converted {} pages", result.page_count());
    Ok(())
}
```

### Advanced Example

```rust
use transmutation::{
    Converter, 
    OutputFormat, 
    ConversionOptions,
    ImageQuality,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = Converter::new()?;
    
    // Custom options
    let options = ConversionOptions {
        split_pages: true,
        optimize_for_llm: true,
        max_chunk_size: 2048,
        image_quality: ImageQuality::High,
        dpi: 300,
        ocr_language: "eng+por".to_string(),
        preserve_layout: true,
        extract_tables: true,
        extract_images: true,
        include_metadata: true,
        compression_level: 6,
        remove_headers_footers: true,
        remove_watermarks: false,
        normalize_whitespace: true,
    };
    
    let result = converter
        .convert("document.pdf")
        .to(OutputFormat::Markdown { 
            split_pages: true, 
            optimize_for_llm: true 
        })
        .with_options(options)
        .execute()
        .await?;
    
    result.save("output/document.md").await?;
    
    Ok(())
}
```

### Batch Processing

```rust
use transmutation::{Converter, BatchProcessor, OutputFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = Converter::new()?;
    let batch = BatchProcessor::new(converter);
    
    // Process multiple files
    let results = batch
        .add_files(&["doc1.pdf", "doc2.docx", "doc3.pptx"])
        .to(OutputFormat::Markdown { 
            split_pages: false, 
            optimize_for_llm: true 
        })
        .parallel(4)  // Use 4 workers
        .execute()
        .await?;
    
    for (file, result) in results {
        println!("{}: {} -> {}", 
            file, 
            result.input_size(), 
            result.output_size()
        );
    }
    
    Ok(())
}
```

## Platform Support

### Windows
- ✅ Windows 10/11 (x64)
- ✅ Windows Server 2019+ (x64)
- ⚠️ ARM64 (experimental)

**Installation**:
```powershell
# Using cargo
cargo install transmutation --features cli

# Or download pre-built binary
# https://github.com/hivellm/transmutation/releases
```

### macOS
- ✅ macOS 12+ (Intel)
- ✅ macOS 12+ (Apple Silicon/M1/M2)

**Installation**:
```bash
# Using cargo
cargo install transmutation --features cli

# Using Homebrew (coming soon)
# brew install transmutation
```

### Linux
- ✅ Ubuntu 20.04+ (x64)
- ✅ Debian 11+ (x64)
- ✅ Fedora 35+ (x64)
- ✅ Arch Linux (x64)
- ⚠️ ARM64 (experimental)

**Installation**:
```bash
# Using cargo
cargo install transmutation --features cli

# Ubuntu/Debian (APT package - coming soon)
# sudo apt install transmutation

# Fedora (RPM package - coming soon)
# sudo dnf install transmutation

# Arch Linux (AUR - coming soon)
# yay -S transmutation
```

## Performance Tips

### 1. Use Parallel Processing
```bash
# Use more workers for batch operations
transmutation batch ./docs/ -o ./output/ -j 8
```

### 2. Enable Caching
```bash
# Set cache directory
export TRANSMUTATION_CACHE_DIR=~/.cache/transmutation

# Use Redis for distributed caching
export TRANSMUTATION_CACHE_REDIS=redis://localhost:6379
```

### 3. Optimize for Your Use Case
```bash
# For LLM embeddings (smaller, optimized)
transmutation convert doc.pdf --optimize-llm

# For archival (high quality)
transmutation convert doc.pdf -f png --dpi 300 --quality 95
```

## Troubleshooting

### CLI Not Found
```bash
# Ensure cargo bin is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Or use absolute path
~/.cargo/bin/transmutation --help
```

### Permission Denied
```bash
# Make executable (Linux/Mac)
chmod +x transmutation

# Or use cargo run
cargo run --features cli -- --help
```

### Missing Features
```bash
# Check enabled features
transmutation version

# Rebuild with specific features
cargo build --release --features "cli,pdf,office,image-ocr"
```

## See Also

- [README.md](../README.md) - Project overview
- [ARCHITECTURE.md](../ARCHITECTURE.md) - Technical design
- [API Documentation](https://docs.rs/transmutation) - Full API reference
- [Examples](../examples/) - Code examples

---

**Questions?** Open an issue on [GitHub](https://github.com/hivellm/transmutation/issues) or join our [Discord](https://discord.gg/hivellm).

