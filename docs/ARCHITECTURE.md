# Transmutation Architecture

## Overview

Transmutation is designed as a modular, high-performance document conversion engine with a focus on extensibility, performance, and LLM optimization.

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      CLI / API Interface                         │
├─────────────────────────────────────────────────────────────────┤
│                   High-Level Converter API                       │
│  ┌──────────────┐  ┌──────────────┐  ┌─────────────────────┐  │
│  │   Converter  │  │BatchProcessor│  │ PipelineBuilder     │  │
│  └──────────────┘  └──────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│                     Conversion Layer                             │
│  ┌──────────┐ ┌──────────┐ ┌────────┐ ┌───────┐ ┌──────────┐ │
│  │   PDF    │ │   DOCX   │ │  PPTX  │ │ XLSX  │ │  Image   │ │
│  └──────────┘ └──────────┘ └────────┘ └───────┘ └──────────┘ │
│  ┌──────────┐ ┌──────────┐ ┌────────┐ ┌───────┐ ┌──────────┐ │
│  │   HTML   │ │   XML    │ │  Audio │ │ Video │ │ Archive  │ │
│  └──────────┘ └──────────┘ └────────┘ └───────┘ └──────────┘ │
├─────────────────────────────────────────────────────────────────┤
│                       Engine Layer                               │
│  ┌──────────┐ ┌──────────┐ ┌─────────┐ ┌────────┐             │
│  │ Docling  │ │Tesseract │ │ Whisper │ │ FFmpeg │             │
│  │ (PyO3)   │ │  (OCR)   │ │  (ASR)  │ │(Media) │             │
│  └──────────┘ └──────────┘ └─────────┘ └────────┘             │
├─────────────────────────────────────────────────────────────────┤
│                      Output Layer                                │
│  ┌──────────┐ ┌──────────┐ ┌─────────┐ ┌────────┐             │
│  │ Markdown │ │  Image   │ │  JSON   │ │  CSV   │             │
│  └──────────┘ └──────────┘ └─────────┘ └────────┘             │
├─────────────────────────────────────────────────────────────────┤
│                   Optimization Layer                             │
│  ┌──────────────┐ ┌──────────────┐ ┌─────────────────────┐    │
│  │ Text Cleanup │ │ Compression  │ │ Quality Metrics     │    │
│  └──────────────┘ └──────────────┘ └─────────────────────┘    │
├─────────────────────────────────────────────────────────────────┤
│                    Integration Layer                             │
│  ┌──────────────┐ ┌──────────────┐ ┌─────────────────────┐    │
│  │ Vectorizer   │ │ LangChain    │ │ LlamaIndex          │    │
│  │ Integration  │ │ Integration  │ │ Integration         │    │
│  └──────────────┘ └──────────────┘ └─────────────────────┘    │
├─────────────────────────────────────────────────────────────────┤
│                      Utility Layer                               │
│  ┌──────────────┐ ┌──────────────┐ ┌─────────────────────┐    │
│  │File Detection│ │  Metadata    │ │    Caching          │    │
│  └──────────────┘ └──────────────┘ └─────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Converter Trait

The central abstraction for all document converters:

```rust
#[async_trait]
pub trait Converter: Send + Sync {
    /// Get the input formats supported by this converter
    fn supported_formats(&self) -> Vec<FileFormat>;
    
    /// Get the output formats supported by this converter
    fn output_formats(&self) -> Vec<OutputFormat>;
    
    /// Convert a document
    async fn convert(
        &self,
        input: &Path,
        output_format: OutputFormat,
        options: ConversionOptions,
    ) -> Result<ConversionResult>;
    
    /// Check if this converter can handle the given format
    fn can_convert(&self, format: FileFormat) -> bool;
    
    /// Get converter metadata
    fn metadata(&self) -> ConverterMetadata;
}
```

### 2. Output Formats

```rust
pub enum OutputFormat {
    Markdown {
        split_pages: bool,
        optimize_for_llm: bool,
    },
    Image {
        format: ImageFormat,  // PNG, JPEG, WEBP
        quality: ImageQuality,
        dpi: u32,
    },
    Json {
        structured: bool,
        include_metadata: bool,
    },
    Csv {
        delimiter: char,
        include_headers: bool,
    },
    EmbeddingReady {
        max_chunk_size: usize,
        overlap: usize,
    },
}
```

### 3. Conversion Pipeline

```rust
pub struct ConversionPipeline {
    stages: Vec<Box<dyn PipelineStage>>,
    options: PipelineOptions,
}

pub trait PipelineStage: Send + Sync {
    async fn process(&self, data: ConversionData) -> Result<ConversionData>;
}

// Example stages:
// 1. FileDetectionStage -> DetectedFormat
// 2. ConverterStage -> RawOutput
// 3. OptimizationStage -> OptimizedOutput
// 4. FormattingStage -> FinalOutput
```

## Data Flow

### Simple Conversion
```
Input File
    ↓
File Type Detection
    ↓
Format-Specific Converter
    ↓
Output Generator
    ↓
Optimization
    ↓
Final Output
```

### Batch Conversion with Caching
```
Multiple Input Files
    ↓
Parallel File Processing
    ↓
Cache Check (hash-based)
    ├─ Cache Hit → Return Cached Result
    └─ Cache Miss
        ↓
    Conversion Pipeline
        ↓
    Cache Store
        ↓
    Output Collection
```

### Vectorizer Integration Pipeline
```
Document
    ↓
Convert to Text/Image
    ↓
Intelligent Chunking
    ↓
Generate Embeddings (Vectorizer)
    ↓
Store in Vector Database
```

## Engine Abstractions

### Docling Engine (PDF, DOCX, etc.)

```rust
pub struct DoclingEngine {
    py_module: PyObject,
    config: DoclingConfig,
}

impl DoclingEngine {
    pub async fn convert_pdf(&self, path: &Path) -> Result<DoclingDocument>;
    pub async fn convert_docx(&self, path: &Path) -> Result<DoclingDocument>;
    pub async fn extract_tables(&self, doc: &DoclingDocument) -> Result<Vec<Table>>;
    pub async fn extract_images(&self, doc: &DoclingDocument) -> Result<Vec<Image>>;
}
```

### OCR Engine (Tesseract)

```rust
pub struct TesseractEngine {
    api: tesseract::TesseractApi,
    languages: Vec<String>,
}

impl TesseractEngine {
    pub async fn recognize(&self, image: &DynamicImage) -> Result<OcrResult>;
    pub async fn detect_language(&self, image: &DynamicImage) -> Result<String>;
    pub async fn get_confidence(&self) -> f32;
}
```

### ASR Engine (Whisper)

```rust
pub struct WhisperEngine {
    model: WhisperModel,
    config: WhisperConfig,
}

impl WhisperEngine {
    pub async fn transcribe(&self, audio: &Path) -> Result<Transcription>;
    pub async fn translate(&self, audio: &Path, target_lang: &str) -> Result<Transcription>;
    pub async fn detect_language(&self, audio: &Path) -> Result<String>;
}
```

## Optimization Strategies

### Text Optimization

1. **Whitespace Normalization**
   - Remove excessive whitespace
   - Normalize line breaks
   - Consistent indentation

2. **Header/Footer Detection**
   - Pattern matching for repeated content
   - Position-based filtering
   - OCR confidence-based removal

3. **Content Deduplication**
   - Hash-based duplicate detection
   - Fuzzy matching for near-duplicates
   - Smart merging of similar content

4. **LLM-Specific Optimization**
   - Token counting and chunking
   - Context window optimization
   - Semantic section boundaries

### Image Optimization

1. **Compression**
   - Format selection (WEBP > PNG > JPEG)
   - Quality adjustment based on content type
   - Progressive encoding

2. **Resolution Optimization**
   - DPI adjustment for text vs images
   - Downscaling for thumbnails
   - Upscaling for OCR

3. **Format Conversion**
   - Automatic format detection
   - Best format recommendation
   - Lossless when needed

## Caching System

### Cache Architecture

```rust
pub trait CacheBackend: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
    async fn set(&self, key: &str, value: &[u8], ttl: Option<Duration>) -> Result<()>;
    async fn invalidate(&self, key: &str) -> Result<()>;
}

pub struct CacheConfig {
    pub backend: CacheBackendType,  // Memory, Redis, SQLite
    pub ttl: Duration,
    pub max_size: usize,
    pub compression: bool,
}
```

### Cache Key Generation

```rust
fn generate_cache_key(
    file_path: &Path,
    output_format: &OutputFormat,
    options: &ConversionOptions,
) -> String {
    let file_hash = hash_file(file_path);
    let options_hash = hash_options(output_format, options);
    format!("transmutation:v1:{}:{}", file_hash, options_hash)
}
```

## Error Handling

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum TransmutationError {
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),
    
    #[error("Conversion failed: {0}")]
    ConversionError(String),
    
    #[error("Engine error: {source}")]
    EngineError {
        #[from]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Invalid options: {0}")]
    InvalidOptions(String),
}
```

## Performance Considerations

### Parallelization Strategy

```rust
pub struct ParallelProcessor {
    thread_pool: ThreadPool,
    max_concurrent: usize,
}

impl ParallelProcessor {
    pub async fn process_batch<F>(&self, items: Vec<T>, process_fn: F) -> Vec<Result<R>>
    where
        F: Fn(T) -> Future<Output = Result<R>>,
    {
        // Use Rayon for CPU-bound tasks
        // Use Tokio for I/O-bound tasks
        // Smart load balancing based on task type
    }
}
```

### Memory Management

1. **Streaming Processing**
   - Process large files in chunks
   - Avoid loading entire document in memory
   - Incremental output generation

2. **Resource Pooling**
   - Reuse engine instances
   - Connection pooling for external services
   - Smart resource allocation

3. **Memory Limits**
   - Configurable memory limits per conversion
   - Automatic fallback to disk-based processing
   - OOM protection

## Security Considerations

### Input Validation

- File type verification (magic bytes)
- Size limits
- Path traversal prevention
- Malicious content detection

### Sandboxing

- Engine isolation (separate processes for untrusted documents)
- Resource limits (CPU, memory, time)
- Network isolation for conversion processes

### Output Sanitization

- Remove embedded scripts
- Strip potentially malicious content
- Validate output formats

## Testing Strategy

### Unit Tests
- Individual converter tests
- Engine abstraction tests
- Utility function tests

### Integration Tests
- End-to-end conversion tests
- Multi-format pipelines
- Error handling scenarios

### Performance Tests
- Benchmark suite
- Memory profiling
- Concurrency stress tests

### Fuzzing
- Input fuzzing for each format
- Random option combinations
- Edge case generation

## Monitoring & Observability

### Metrics

```rust
pub struct ConversionMetrics {
    pub conversion_duration_ms: u64,
    pub input_size_bytes: usize,
    pub output_size_bytes: usize,
    pub pages_processed: usize,
    pub cache_hit: bool,
    pub engine_used: String,
}
```

### Tracing

- OpenTelemetry integration
- Distributed tracing for pipelines
- Performance hotspot identification

### Logging

- Structured logging (JSON)
- Log levels per component
- Audit trail for conversions

## Extensibility

### Plugin System

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    
    fn register_converter(&self, registry: &mut ConverterRegistry);
    fn register_output_format(&self, registry: &mut OutputFormatRegistry);
    
    fn on_conversion_start(&self, context: &ConversionContext) -> Result<()>;
    fn on_conversion_complete(&self, result: &ConversionResult) -> Result<()>;
}
```

### Custom Converters

Users can implement custom converters:

```rust
struct MyCustomConverter;

#[async_trait]
impl Converter for MyCustomConverter {
    // Implementation
}

// Register
let mut registry = ConverterRegistry::new();
registry.register(Box::new(MyCustomConverter));
```

## Deployment Modes

### 1. Library Mode
- Embedded in applications
- Direct API usage
- No external dependencies (except engines)

### 2. CLI Mode
- Command-line tool
- Batch processing
- Scripting integration

### 3. Server Mode
- REST API
- WebSocket for streaming
- Job queue management

### 4. Distributed Mode
- Worker pool
- Message queue integration
- Horizontal scaling

---

**Last Updated**: 2025-10-12  
**Version**: 1.0.0  
**Status**: Draft

