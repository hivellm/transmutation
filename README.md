# Transmutation

**High-performance document conversion engine for AI/LLM embeddings**

Transmutation is a Rust-based document conversion module designed to transform various file formats into optimized text and image outputs suitable for LLM processing and vector embeddings. Built as a core component of the HiveLLM Vectorizer ecosystem, it leverages [Docling](https://github.com/docling-project) for advanced document understanding.

## 🎯 Project Goals

- Convert documents to LLM-friendly formats (Markdown, Images)
- Optimize output for embedding generation (text and multimodal)
- Maintain maximum quality with minimum size
- High-performance Rust implementation
- Seamless integration with HiveLLM Vectorizer

## 📋 Supported Formats

### Document Formats

| Input Format | Output Options | Status |
|-------------|----------------|---------|
| **PDF** | Image per page, Markdown (per page/full), JSON | 🔄 Planned |
| **DOCX** | Image per page, Markdown (per page/full), JSON | 🔄 Planned |
| **PPTX** | Image per slide, Markdown (per slide/full), JSON | 🔄 Planned |
| **XLSX** | Markdown, CSV, JSON | 🔄 Planned |
| **HTML** | Image, Markdown, JSON | 🔄 Planned |
| **XML** | Markdown, JSON | 🔄 Planned |
| **TXT** | Markdown, JSON | 🔄 Planned |
| **MD** | Markdown (normalized), JSON | 🔄 Planned |
| **RTF** | Markdown, JSON | 🔄 Planned |
| **ODT** | Markdown, Image per page, JSON | 🔄 Planned |
| **CSV/TSV** | Markdown tables, JSON | 🔄 Planned |

### Image Formats (OCR/ASR)

| Input Format | Output Options | OCR Engine | Status |
|-------------|----------------|------------|---------|
| **JPG/JPEG** | Markdown (OCR), JSON | Tesseract/Whisper | 🔄 Planned |
| **PNG** | Markdown (OCR), JSON | Tesseract/Whisper | 🔄 Planned |
| **TIFF/TIF** | Markdown (OCR), JSON | Tesseract/Whisper | 🔄 Planned |
| **BMP** | Markdown (OCR), JSON | Tesseract/Whisper | 🔄 Planned |
| **GIF** | Markdown (OCR), JSON | Tesseract/Whisper | 🔄 Planned |
| **WEBP** | Markdown (OCR), JSON | Tesseract/Whisper | 🔄 Planned |

### Audio/Video Formats

| Input Format | Output Options | Engine | Status |
|-------------|----------------|---------|---------|
| **MP3** | Markdown (transcription), JSON | Whisper | 🔄 Planned |
| **MP4** | Markdown (transcription), Images (keyframes), JSON | Whisper/FFmpeg | 🔄 Planned |
| **WAV** | Markdown (transcription), JSON | Whisper | 🔄 Planned |
| **M4A** | Markdown (transcription), JSON | Whisper | 🔄 Planned |

### Archive Formats

| Input Format | Output Options | Status |
|-------------|----------------|---------|
| **ZIP** | Extract and process contents | 🔄 Planned |
| **TAR/GZ** | Extract and process contents | 🔄 Planned |
| **7Z** | Extract and process contents | 🔄 Planned |

## 🏗️ Architecture

```
transmutation/
├── src/
│   ├── lib.rs                  # Main library entry
│   ├── converters/
│   │   ├── mod.rs              # Converter registry
│   │   ├── pdf.rs              # PDF conversion (Docling)
│   │   ├── docx.rs             # DOCX conversion
│   │   ├── pptx.rs             # PPTX conversion
│   │   ├── xlsx.rs             # XLSX conversion
│   │   ├── html.rs             # HTML conversion
│   │   ├── xml.rs              # XML conversion
│   │   ├── image.rs            # Image OCR (Tesseract)
│   │   ├── audio.rs            # Audio transcription (Whisper)
│   │   ├── video.rs            # Video processing (FFmpeg)
│   │   └── archive.rs          # Archive extraction
│   ├── output/
│   │   ├── mod.rs              # Output format handlers
│   │   ├── markdown.rs         # Markdown generation
│   │   ├── image.rs            # Image generation/optimization
│   │   ├── json.rs             # JSON serialization
│   │   └── csv.rs              # CSV generation
│   ├── engines/
│   │   ├── mod.rs              # Engine abstractions
│   │   ├── docling.rs          # Docling Python bridge (PyO3)
│   │   ├── tesseract.rs        # Tesseract OCR wrapper
│   │   ├── whisper.rs          # Whisper ASR wrapper
│   │   └── ffmpeg.rs           # FFmpeg wrapper
│   ├── optimization/
│   │   ├── mod.rs              # Optimization strategies
│   │   ├── text.rs             # Text compression/cleanup
│   │   ├── image.rs            # Image compression
│   │   └── quality.rs          # Quality metrics
│   ├── integration/
│   │   ├── mod.rs              # Integration layer
│   │   ├── vectorizer.rs       # Vectorizer integration
│   │   ├── langchain.rs        # LangChain integration
│   │   ├── llamaindex.rs       # LlamaIndex integration
│   │   └── haystack.rs         # Haystack integration
│   ├── utils/
│   │   ├── mod.rs              # Utilities
│   │   ├── file_detect.rs      # File type detection
│   │   ├── metadata.rs         # Metadata extraction
│   │   └── cache.rs            # Conversion cache
│   └── error.rs                # Error types
├── bindings/
│   ├── python/                 # Python bindings (PyO3)
│   ├── node/                   # Node.js bindings (Neon)
│   └── wasm/                   # WebAssembly bindings
├── cli/
│   └── src/
│       └── main.rs             # CLI application
├── examples/
│   ├── basic_conversion.rs
│   ├── batch_processing.rs
│   ├── vectorizer_integration.rs
│   └── custom_pipeline.rs
├── benches/
│   └── conversion_benchmarks.rs
├── tests/
│   ├── integration/
│   └── fixtures/
├── Cargo.toml
├── README.md
├── LICENSE
├── ROADMAP.md
├── ARCHITECTURE.md
└── CONTRIBUTING.md
```

## 🚀 Quick Start

### Installation

```bash
# Add to Cargo.toml
[dependencies]
transmutation = "0.1.0"
```

### Basic Usage

```rust
use transmutation::{Converter, OutputFormat, ConversionOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize converter
    let converter = Converter::new()?;
    
    // Convert PDF to Markdown
    let result = converter
        .convert("document.pdf")
        .to(OutputFormat::Markdown)
        .with_options(ConversionOptions {
            split_pages: true,
            optimize_for_llm: true,
            ..Default::default()
        })
        .execute()
        .await?;
    
    // Save output
    result.save("output/document.md").await?;
    
    println!("Converted {} pages", result.page_count());
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
        .to(OutputFormat::Markdown)
        .parallel(4)
        .execute()
        .await?;
    
    for (file, result) in results {
        println!("{}: {} -> {}", file, result.input_size(), result.output_size());
    }
    
    Ok(())
}
```

### Vectorizer Integration

```rust
use transmutation::{Converter, OutputFormat};
use vectorizer::VectorizerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = Converter::new()?;
    let vectorizer = VectorizerClient::new("http://localhost:15002").await?;
    
    // Convert and embed in one pipeline
    let result = converter
        .convert("document.pdf")
        .to(OutputFormat::EmbeddingReady)
        .pipe_to(&vectorizer)
        .execute()
        .await?;
    
    println!("Embedded {} chunks", result.chunk_count());
    Ok(())
}
```

## 🔧 Configuration

### Conversion Options

```rust
pub struct ConversionOptions {
    // Output control
    pub split_pages: bool,           // Split output by pages
    pub optimize_for_llm: bool,      // Optimize for LLM processing
    pub max_chunk_size: usize,       // Maximum chunk size (tokens)
    
    // Quality settings
    pub image_quality: ImageQuality, // High, Medium, Low
    pub dpi: u32,                    // DPI for image output (default: 150)
    pub ocr_language: String,        // OCR language (default: "eng")
    
    // Processing options
    pub preserve_layout: bool,       // Preserve document layout
    pub extract_tables: bool,        // Extract tables separately
    pub extract_images: bool,        // Extract embedded images
    pub include_metadata: bool,      // Include document metadata
    
    // Optimization
    pub compression_level: u8,       // 0-9 for output compression
    pub remove_headers_footers: bool,
    pub remove_watermarks: bool,
    pub normalize_whitespace: bool,
}
```

## 🔌 Integrations

### Docling Integration

Transmutation leverages [Docling](https://github.com/docling-project) for advanced document understanding:

- **docling-core**: Type definitions and document models
- **docling-parse**: Advanced PDF parsing
- **docling-ibm-models**: AI models for layout understanding
- **docling-mcp**: Model Context Protocol for agents

### LLM Framework Integrations

- **LangChain**: Document loaders and text splitters
- **LlamaIndex**: Document readers and node parsers
- **Haystack**: Document converters and preprocessors
- **DSPy**: Optimized document processing

## 📊 Performance

### Benchmarks (Preliminary Targets)

| Operation | Input Size | Time | Throughput |
|-----------|-----------|------|------------|
| PDF → Markdown | 10MB (100 pages) | ~5s | 20 pages/s |
| DOCX → Markdown | 5MB (50 pages) | ~2s | 25 pages/s |
| Image OCR | 1920x1080 PNG | ~500ms | 2 images/s |
| Batch Processing | 100 files | ~30s | 3.3 files/s |

### Memory Usage

- Base: ~50MB
- Per conversion: ~100-500MB (depending on document size)
- Cached models: ~2GB (Whisper, Tesseract)

## 🛣️ Roadmap

See [ROADMAP.md](ROADMAP.md) for detailed development plan.

### Phase 1: Foundation (Q1 2025)
- ✅ Project structure and architecture
- 🔄 Core converter interfaces
- 🔄 PDF conversion (via Docling)
- 🔄 Basic Markdown output

### Phase 2: Core Formats (Q2 2025)
- 📝 DOCX, PPTX, XLSX conversion
- 📝 HTML/XML conversion
- 📝 Image OCR (Tesseract)
- 📝 Quality optimization

### Phase 3: Advanced Features (Q3 2025)
- 📝 Audio/Video transcription (Whisper)
- 📝 Archive handling
- 📝 Batch processing
- 📝 Caching system

### Phase 4: Integrations (Q4 2025)
- 📝 Vectorizer integration
- 📝 LangChain/LlamaIndex support
- 📝 Python/Node.js bindings
- 📝 WASM support

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📝 License

MIT License - see [LICENSE](LICENSE) for details.

## 🔗 Links

- **GitHub**: https://github.com/hivellm/transmutation
- **Documentation**: https://docs.hivellm.org/transmutation
- **Docling Project**: https://github.com/docling-project
- **HiveLLM Vectorizer**: https://github.com/hivellm/vectorizer

## 🏆 Credits

Built with ❤️ by the HiveLLM Team

Powered by:
- [Docling](https://github.com/docling-project) - Document understanding
- [Tesseract](https://github.com/tesseract-ocr/tesseract) - OCR engine
- [Whisper](https://github.com/openai/whisper) - Speech recognition
- [FFmpeg](https://ffmpeg.org/) - Multimedia processing

---

**Status**: 🚧 In Planning - Not yet implemented

