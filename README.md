# Transmutation

**High-performance document conversion engine for AI/LLM embeddings**

Transmutation is a **pure Rust** document conversion engine designed to transform various file formats into optimized text and image outputs suitable for LLM processing and vector embeddings. Built as a core component of the HiveLLM Vectorizer ecosystem, Transmutation is a **high-performance alternative to Docling**, offering superior speed, lower memory usage, and zero runtime dependencies.

## 🎯 Project Goals

- **Pure Rust implementation** - No Python dependencies, maximum performance
- Convert documents to LLM-friendly formats (Markdown, Images, JSON)
- Optimize output for embedding generation (text and multimodal)
- Maintain maximum quality with minimum size
- **Competitor to Docling** - **98x faster**, more efficient, and easier to deploy
- Seamless integration with HiveLLM Vectorizer

## 📊 Benchmark Results

**Transmutation vs Docling** (Fast Mode - Pure Rust):

| Metric | Paper 1 (15 pages) | Paper 2 (25 pages) | Average |
|--------|--------------------|--------------------|---------|
| **Similarity** | 76.36% | 84.44% | **80.40%** |
| **Speed** | 108x faster | 88x faster | **98x faster** |
| **Time (Docling)** | 31.36s | 40.56s | ~35s |
| **Time (Transmutation)** | 0.29s | 0.46s | ~0.37s |

- ✅ **80% similarity** - Acceptable for most use cases
- ✅ **98x faster** - Near-instant conversion  
- ✅ **Pure Rust** - No Python/ML dependencies
- ✅ **Low memory** - 50 MB footprint
- 🎯 **Goal**: 95% similarity (Precision Mode with C++ FFI - in development)

See [BENCHMARK_COMPARISON.md](BENCHMARK_COMPARISON.md) for detailed results.

## 📋 Supported Formats

### Document Formats

| Input Format | Output Options | Status | Modes |
|-------------|----------------|---------|-------|
| **PDF** | Image per page, Markdown (per page/full), JSON | ✅ **Production** | Fast, Precision, FFI |
| **DOCX** | Image per page, Markdown (per page/full), JSON | ✅ **Production** | Pure Rust + LibreOffice |
| **XLSX** | Markdown tables, CSV, JSON | ✅ **Production** | Pure Rust (148 pg/s) |
| **PPTX** | Image per slide, Markdown per slide | ✅ **Production** | Pure Rust (1639 pg/s) |
| **HTML** | Markdown, JSON | ✅ **Production** | Pure Rust (2110 pg/s) |
| **XML** | Markdown, JSON | ✅ **Production** | Pure Rust (2353 pg/s) |
| **TXT** | Markdown, JSON | ✅ **Production** | Pure Rust (2805 pg/s) |
| **CSV/TSV** | Markdown tables, JSON | ✅ **Production** | Pure Rust (2647 pg/s) |
| **RTF** | Markdown, JSON | ⚠️ **Beta** | Pure Rust (simplified parser) |
| **ODT** | Markdown, JSON | ⚠️ **Beta** | Pure Rust (ZIP + XML) |
| **MD** | Markdown (normalized), JSON | 🔄 Planned | - |

### Image Formats (OCR)

| Input Format | Output Options | OCR Engine | Status |
|-------------|----------------|------------|---------|
| **JPG/JPEG** | Markdown (OCR), JSON | Tesseract | ✅ **Production** |
| **PNG** | Markdown (OCR), JSON | Tesseract | ✅ **Production** |
| **TIFF/TIF** | Markdown (OCR), JSON | Tesseract | ✅ **Production** |
| **BMP** | Markdown (OCR), JSON | Tesseract | ✅ **Production** |
| **GIF** | Markdown (OCR), JSON | Tesseract | ✅ **Production** |
| **WEBP** | Markdown (OCR), JSON | Tesseract | ✅ **Production** |

### Audio/Video Formats

| Input Format | Output Options | Engine | Status |
|-------------|----------------|---------|---------|
| **MP3** | Markdown (transcription), JSON | Whisper | ✅ **Production** |
| **WAV** | Markdown (transcription), JSON | Whisper | ✅ **Production** |
| **M4A** | Markdown (transcription), JSON | Whisper | ✅ **Production** |
| **FLAC** | Markdown (transcription), JSON | Whisper | ✅ **Production** |
| **OGG** | Markdown (transcription), JSON | Whisper | ✅ **Production** |
| **MP4** | Markdown (transcription), JSON | FFmpeg + Whisper | ✅ **Production** |
| **AVI** | Markdown (transcription), JSON | FFmpeg + Whisper | ✅ **Production** |
| **MKV** | Markdown (transcription), JSON | FFmpeg + Whisper | ✅ **Production** |
| **MOV** | Markdown (transcription), JSON | FFmpeg + Whisper | ✅ **Production** |
| **WEBM** | Markdown (transcription), JSON | FFmpeg + Whisper | ✅ **Production** |

### Archive Formats

| Input Format | Output Options | Status | Performance |
|-------------|----------------|---------|-------------|
| **ZIP** | File listing, statistics, Markdown index, JSON | ✅ **Production** | Pure Rust (1864 pg/s) |
| **TAR/GZ** | Extract and process contents | 🔄 Planned | - |
| **7Z** | Extract and process contents | 🔄 Planned | - |

## 🏗️ Architecture

```
transmutation/
├── src/
│   ├── lib.rs                  # Main library entry
│   ├── converters/
│   │   ├── mod.rs              # Converter registry
│   │   ├── pdf.rs              # PDF conversion (pure Rust)
│   │   ├── docx.rs             # DOCX conversion
│   │   ├── pptx.rs             # PPTX conversion
│   │   ├── xlsx.rs             # XLSX conversion
│   │   ├── html.rs             # HTML conversion
│   │   ├── xml.rs              # XML conversion
│   │   ├── image.rs            # Image OCR (Tesseract)
│   │   ├── audio.rs            # Audio transcription (pure Rust ASR)
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
│   │   ├── pdf_parser.rs       # Pure Rust PDF parsing
│   │   ├── tesseract.rs        # Tesseract OCR wrapper
│   │   ├── audio_asr.rs        # Pure Rust audio transcription
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
├── src/bin/
│   └── transmutation.rs        # CLI application (included in main crate)
├── bindings/
│   ├── python/                 # Python bindings (PyO3) - Future
│   ├── node/                   # Node.js bindings (Neon) - Future
│   └── wasm/                   # WebAssembly bindings - Future
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

**Windows MSI Installer:**
```powershell
# Download from releases or build:
.\build-msi.ps1
msiexec /i target\wix\transmutation-0.1.1-x86_64.msi
```
See [`docs/MSI_BUILD.md`](docs/MSI_BUILD.md) for details.

**Cargo:**
```bash
# Add to Cargo.toml
[dependencies]
transmutation = "0.1"

# Core features (always enabled, no flags needed):
# - PDF, HTML, XML, ZIP, TXT, CSV, TSV, RTF, ODT

# With Office formats (default)
[dependencies.transmutation]
version = "0.1"
features = ["office"]  # DOCX, XLSX, PPTX

# With optional features (requires external tools)
features = ["office", "pdf-to-image", "tesseract", "audio"]
```

### External Dependencies

Transmutation is **mostly pure Rust**, with **core features requiring ZERO dependencies**:

| Feature | Requires | Status |
|---------|----------|---------|
| **Core** (PDF, HTML, XML, ZIP, TXT, CSV, TSV, RTF, ODT) | ✅ **None** | Always enabled |
| `office` (DOCX, XLSX, PPTX - Text) | ✅ **None** | Pure Rust (default) |
| `pdf-to-image` | ⚠️ poppler-utils | Optional |
| `office` + images | ⚠️ LibreOffice | Optional |
| `image-ocr` | ⚠️ Tesseract OCR | Optional |
| `audio` | ⚠️ Whisper CLI | Optional |
| `video` | ⚠️ FFmpeg + Whisper | Optional |
| `archives-extended` (TAR, GZ, 7Z) | ⚠️ tar, flate2 crates | Optional |

**During compilation**, `build.rs` will automatically **detect missing dependencies** and provide installation instructions:

```bash
cargo build --features "pdf-to-image"

# If pdftoppm is missing, you'll see:
⚠️  Optional External Dependencies Missing

  ❌ pdftoppm (poppler-utils): PDF → Image conversion
     Install: sudo apt-get install poppler-utils

📖 Quick install (all dependencies):
   ./install/install-deps-linux.sh
```

**Installation scripts** are provided for all platforms:
- **Linux**: `./install/install-deps-linux.sh`
- **macOS**: `./install/install-deps-macos.sh`  
- **Windows**: `.\install\install-deps-windows.ps1` (or `.bat`)

See [`install/README.md`](install/README.md) for detailed instructions.

## 📖 Usage Guide

### CLI Usage

**Basic Conversion:**
```bash
# Convert PDF to Markdown
transmutation convert document.pdf -o output.md

# Convert DOCX to Markdown with images
transmutation convert report.docx -o output.md --extract-images

# Convert with precision mode (77% similarity)
transmutation convert paper.pdf -o output.md --precision

# Convert multiple files
transmutation batch *.pdf -o output/ --parallel 4
```

**Format-Specific Examples:**
```bash
# PDF → Markdown (split by pages)
transmutation convert document.pdf -o output/ --split-pages

# DOCX → Markdown + Images
transmutation convert report.docx -o output.md --images

# XLSX → CSV
transmutation convert data.xlsx -o output.csv --format csv

# PPTX → Markdown (one file per slide)
transmutation convert slides.pptx -o output/ --split-slides

# Image OCR → Markdown
transmutation convert scan.jpg -o output.md --ocr --lang eng

# ZIP → Extract and convert all
transmutation convert archive.zip -o output/ --recursive
```

**Advanced Options:**
```bash
# Optimize for LLM embeddings
transmutation convert document.pdf \
  --optimize-llm \
  --max-chunk-size 512 \
  --remove-headers \
  --normalize-whitespace

# High-quality image extraction
transmutation convert document.pdf \
  --extract-images \
  --dpi 300 \
  --image-quality high

# Batch processing with progress
transmutation batch papers/*.pdf \
  -o converted/ \
  --parallel 8 \
  --progress \
  --format markdown
```

### Library Usage (Rust)

**Basic Conversion:**
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

### Python Usage (PyO3 Bindings - Future)

```python
from transmutation import Converter, OutputFormat

# Initialize converter
converter = Converter()

# Convert PDF to Markdown
result = converter.convert(
    "document.pdf",
    output_format=OutputFormat.Markdown,
    split_pages=True,
    optimize_for_llm=True
)

result.save("output/document.md")
print(f"Converted {result.page_count()} pages")

# Batch processing
from transmutation import BatchProcessor

batch = BatchProcessor(converter)
results = batch.add_files([
    "doc1.pdf",
    "doc2.docx",
    "doc3.pptx"
]).to(OutputFormat.Markdown).parallel(4).execute()

for file, result in results:
    print(f"{file}: {result.input_size()} -> {result.output_size()}")
```

### JavaScript/TypeScript (Neon Bindings - Future)

```typescript
import { Converter, OutputFormat, ConversionOptions } from 'transmutation';

// Initialize converter
const converter = new Converter();

// Convert PDF to Markdown
const result = await converter
  .convert('document.pdf')
  .to(OutputFormat.Markdown)
  .withOptions({
    splitPages: true,
    optimizeForLlm: true,
    extractImages: false
  })
  .execute();

await result.save('output/document.md');
console.log(`Converted ${result.pageCount()} pages`);

// Batch processing
import { BatchProcessor } from 'transmutation';

const batch = new BatchProcessor(converter);
const results = await batch
  .addFiles(['doc1.pdf', 'doc2.docx', 'doc3.pptx'])
  .to(OutputFormat.Markdown)
  .parallel(4)
  .execute();

results.forEach(([file, result]) => {
  console.log(`${file}: ${result.inputSize()} -> ${result.outputSize()}`);
});
```

## 🎯 Common Use Cases

### 1. RAG System Document Ingestion

```bash
# Convert research papers for semantic search
transmutation batch papers/*.pdf \
  -o embeddings/ \
  --optimize-llm \
  --split-pages \
  --max-chunk-size 512 \
  --parallel 8

# Then index with Vectorizer
vectorizer insert --collection research_papers embeddings/*.md
```

### 2. Document Archive Migration

```bash
# Convert legacy documents to Markdown
transmutation batch archive/ \
  -o markdown/ \
  --recursive \
  --format markdown \
  --parallel 16 \
  --progress

# Supported: PDF, DOCX, XLSX, PPTX, RTF, ODT, HTML, XML
```

### 3. OCR for Scanned Documents

```bash
# Batch OCR with Tesseract
transmutation batch scans/*.jpg \
  -o text/ \
  --ocr \
  --lang eng \
  --dpi 300 \
  --parallel 4

# Multi-language support
transmutation convert document_pt.jpg \
  -o output.md \
  --ocr \
  --lang por
```

### 4. Legal Document Processing

```bash
# Convert legal PDFs with high precision
transmutation convert contract.pdf \
  -o contract.md \
  --precision \
  --preserve-layout \
  --extract-tables \
  --include-metadata

# Batch process court documents
transmutation batch cases/*.pdf \
  -o processed/ \
  --precision \
  --parallel 4
```

### 5. Academic Paper Analysis

```bash
# Extract text from arXiv papers
transmutation batch papers/*.pdf \
  -o markdown/ \
  --split-pages \
  --extract-tables \
  --normalize-whitespace

# Create embeddings for similarity search
vectorizer insert --collection arxiv markdown/*.md
```

### 6. Data Extraction from Spreadsheets

```bash
# Convert Excel to Markdown tables
transmutation convert data.xlsx -o tables.md --format markdown

# Convert to CSV for analysis
transmutation convert data.xlsx -o data.csv --format csv

# Convert to JSON
transmutation convert data.xlsx -o data.json --format json
```

### 7. Presentation Content Extraction

```bash
# Extract text from PowerPoint slides
transmutation convert presentation.pptx \
  -o slides/ \
  --split-slides \
  --extract-images \
  --format markdown

# Batch process training materials
transmutation batch trainings/*.pptx \
  -o content/ \
  --split-slides \
  --parallel 8
```

### 8. Web Content Archiving

```bash
# Convert saved HTML pages
transmutation batch pages/*.html \
  -o markdown/ \
  --format markdown \
  --normalize-whitespace

# Process downloaded documentation
transmutation batch docs/*.html \
  -o processed/ \
  --extract-images \
  --parallel 4
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

## 🆚 Why Transmutation vs Docling?

| Feature | Transmutation | Docling |
|---------|--------------|---------|
| **Language** | 100% Rust | Python |
| **Performance** | ✅ **250x faster** | Baseline |
| **Memory Usage** | ✅ ~20MB | ~2-3GB |
| **Dependencies** | ✅ Zero runtime deps | Python + ML models |
| **Deployment** | ✅ Single binary (~5MB) | Python env + models (~2GB) |
| **Startup Time** | ✅ <100ms | ~5-10s |
| **Platform Support** | ✅ Windows/Mac/Linux | Requires Python |

### LLM Framework Integrations

- **LangChain**: Document loaders and text splitters
- **LlamaIndex**: Document readers and node parsers
- **Haystack**: Document converters and preprocessors
- **DSPy**: Optimized document processing

## 📊 Performance

### Real-World Benchmarks ✅

**Test Document:** Attention Is All You Need (arXiv:1706.03762v7.pdf)  
**Size:** 2.22 MB, 15 pages

| Metric | Transmutation | Docling | Improvement |
|--------|--------------|---------|-------------|
| **Conversion Time** | 0.21s | 52.68s | ✅ **250x faster** |
| **Processing Speed** | 71 pages/sec | 0.28 pages/sec | ✅ **254x faster** |
| **Memory Usage** | ~20MB | ~2-3GB | ✅ **100-150x less** |
| **Startup Time** | <0.1s | ~6s | ✅ **60x faster** |
| **Output Quality (Fast)** | 71.8% similarity | 100% (reference) | ⚠️ **Trade-off** |
| **Output Quality (Precision)** | 77.3% similarity | 100% (reference) | ⚠️ **+5.5% better** |

### Projected Performance

| Operation | Input Size | Time | Throughput |
|-----------|-----------|------|------------|
| PDF → Markdown | 2.2MB (15 pages) | 0.21s | **71 pages/s** ✅ |
| PDF → Markdown | 10MB (100 pages) | ~1.4s | **71 pages/s** |
| Batch (1,000 PDFs) | 2.2GB (15,000 pages) | ~4 min | **3,750 pages/min** |

### Memory Footprint

- Base: ~20MB (pure Rust, no Python runtime) ✅
- Per conversion: Minimal (streaming processing)
- No ML models required (unlike Docling's 2-3GB)

### Precision vs Performance Trade-off

**Fast Mode (default)** - 71.8% similarity:
- ✅ 250x faster than Docling
- ✅ Pure Rust with basic text heuristics
- ✅ Works on any PDF without training
- ✅ Zero runtime dependencies

**Precision Mode (`--precision`)** - 77.3% similarity:
- ✅ 250x faster than Docling (same speed as fast mode)
- ✅ Enhanced text processing with space correction
- ✅ +5.5% better than fast mode
- ✅ No hardcoded rules, all generic heuristics

**Why not 95%+ similarity?**

Docling uses:
1. **`docling-parse`** (C++ library) - Extracts text with precise coordinates, fonts, and layout info
2. **LayoutModel** (ML) - Deep learning to detect block types (headings, paragraphs, tables) visually
3. **ReadingOrderModel** (ML) - ML-based reading order determination

Transmutation provides **three modes**:

**1. Fast Mode (default):**
- Pure Rust text extraction (`pdf-extract`)
- Generic heuristics (no ML)
- 71.8% similarity, 250x faster

**2. Precision Mode (`--precision`):**
- Enhanced text processing
- Generic heuristics + space correction
- 77.3% similarity, 250x faster

**Future: C++ FFI Mode** - Direct integration with docling-parse (no Python):
- Will use C++ library via FFI for 95%+ similarity
- No Python dependency, pure Rust + C++ shared library
- In development

| Mode | Similarity | Speed | Memory | Dependencies |
|------|-----------|-------|--------|--------------|
| **Fast** | 71.8% | 250x | 50 MB | None (pure Rust) |
| **Precision** | 77.3% | 250x | 50 MB | None (pure Rust) |
| **FFI** *(future)* | 95%+ | ~50x | 100 MB | C++ shared lib only |

## 🛣️ Roadmap

See [ROADMAP.md](ROADMAP.md) for detailed development plan.

### Phase 1: Foundation (Q1 2025) ✅ COMPLETE
- ✅ Project structure and architecture
- ✅ Core converter interfaces
- ✅ PDF conversion (pure Rust - pdf-extract)
- ✅ Advanced Markdown output with intelligent paragraph joining
- ✅ **98x faster than Docling** benchmark achieved (97 papers tested)

### Phase 1.5: Distribution & Tooling (Oct 2025) ✅ COMPLETE
- ✅ Windows MSI installer with dependency management
- ✅ Custom icons and professional branding
- ✅ Multi-platform installation scripts (5 variants)
- ✅ Build-time dependency detection
- ✅ Comprehensive documentation

### Phase 2: Core Formats (Q2 2025) ✅ 100% COMPLETE
- ✅ **DOCX conversion** (Markdown + Images - Pure Rust)
- ✅ **XLSX conversion** (Markdown/CSV/JSON - Pure Rust, 148 pg/s)
- ✅ **PPTX conversion** (Markdown/Images - Pure Rust, 1639 pg/s)
- ✅ **HTML/XML conversion** (Pure Rust, 2110-2353 pg/s)
- ✅ **Text formats** (TXT, CSV, TSV, RTF, ODT - Pure Rust)
- ✅ **11 formats** total (8 production, 2 beta)

### Phase 2.5: Core Features Architecture ✅ COMPLETE
- ✅ Core formats always enabled (no feature flags)
- ✅ Simplified API and user experience
- ✅ Faster compilation

### Phase 3: Advanced Features (Q3 2025) ✅ COMPLETE
- ✅ **Archive handling** (ZIP, TAR, TAR.GZ - 1864 pg/s)
- ✅ **Batch processing** (Concurrent with Tokio - 4,627 pg/s)
- ✅ **Image OCR** (Tesseract - 6 formats, 88x faster than Docling)

### Phase 4: Advanced Optimizations
- 📝 Performance optimizations
- 📝 Quality improvements (RTF, ODT)
- 📝 Memory optimizations
- 📝 v1.0.0 Release

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📝 License

MIT License - see [LICENSE](LICENSE) for details.

## 📝 Changelog

See [CHANGELOG.md](CHANGELOG.md) for detailed version history and release notes.

**Current Version**: 0.1.1 (October 13, 2025)

## 🔗 Links

- **GitHub**: https://github.com/hivellm/transmutation
- **Documentation**: https://docs.hivellm.org/transmutation
- **Changelog**: [CHANGELOG.md](CHANGELOG.md)
- **Docling Project**: https://github.com/docling-project
- **HiveLLM Vectorizer**: https://github.com/hivellm/vectorizer

## 🏆 Credits

Built with ❤️ by the HiveLLM Team

**Pure Rust implementation** - No Python, no ML model dependencies

Powered by:
- [lopdf](https://github.com/J-F-Liu/lopdf) - Pure Rust PDF parsing
- [docx-rs](https://github.com/bokuweb/docx-rs) - Pure Rust DOCX parsing
- [Tesseract](https://github.com/tesseract-ocr/tesseract) - OCR engine (optional)
- [FFmpeg](https://ffmpeg.org/) - Multimedia processing (optional)

**Inspired by** [Docling](https://github.com/docling-project), but built to be faster, lighter, and easier to deploy.

---

**Status**: ✅ v0.1.1 - Production Ready with Professional Distribution Tools

**Latest Updates (v0.1.1)**:
- 🪟 Windows MSI Installer with dependency management
- 🎨 Custom icons and branding
- 📦 Multi-platform installation scripts
- 🔧 Automated build and distribution tools

