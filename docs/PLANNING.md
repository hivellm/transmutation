# Transmutation - Project Planning Document

## Executive Summary

**Transmutation** is a high-performance document conversion engine written in Rust, designed specifically for AI/LLM embeddings and vector database ingestion. Built as a core component of the HiveLLM Vectorizer ecosystem, it leverages [Docling](https://github.com/docling-project) for advanced document understanding.

### Key Objectives

1. **Convert documents to LLM-optimized formats** (Markdown, Images, JSON)
2. **Maintain maximum quality with minimum size** for efficient embeddings
3. **Support 20+ document formats** with extensible architecture
4. **High-performance Rust implementation** with parallel processing
5. **Seamless integration** with HiveLLM Vectorizer and LLM frameworks

### Target Release

- **v0.1.0 (MVP)**: Q1 2025 - PDF/DOCX conversion
- **v0.5.0 (Beta)**: Q2 2025 - All core formats
- **v1.0.0 (Stable)**: Q4 2025 - Production-ready with integrations

---

## Supported Formats Summary

### Documents (8 formats)
- PDF, DOCX, PPTX, XLSX, HTML, XML, TXT, MD, RTF, ODT

### Images + OCR (6 formats)
- JPG, PNG, TIFF, BMP, GIF, WEBP

### Audio/Video (4 formats)
- MP3, MP4, WAV, M4A

### Archives (3 formats)
- ZIP, TAR/GZ, 7Z

**Total: 21+ formats** with extensible plugin architecture

---

## Technical Stack

### Core Technologies
- **Language**: Rust (Edition 2024)
- **Async Runtime**: Tokio
- **Python Bridge**: PyO3 (for Docling)
- **Parallelization**: Rayon

### External Engines
- **Docling**: PDF, DOCX, PPTX parsing (via Python)
- **Tesseract**: OCR for images
- **Whisper**: Speech-to-text transcription
- **FFmpeg**: Video/audio processing

### Key Dependencies (Pure Rust)
```toml
tokio = "1.47"          # Async runtime
lopdf = "0.35"          # PDF parsing
docx-rs = "0.4"         # DOCX parsing
image = "0.25"          # Image processing
rayon = "1.10"          # Parallelization
serde = "1.0"           # Serialization
thiserror = "2.0"       # Error handling
```

---

## Architecture Highlights

### Modular Design

```
transmutation/
├── converters/     # Format-specific converters
├── engines/        # External tool wrappers
├── output/         # Output format generators
├── optimization/   # Quality & compression
├── integration/    # Framework integrations
└── utils/          # Utilities (detection, cache)
```

### Conversion Pipeline

```
Input → Detection → Converter → Optimizer → Output
         ↓            ↓           ↓          ↓
      FileType    DocFormat   Compress   Markdown
                                          /Image/JSON
```

### Performance Strategy
- **Parallel processing**: Rayon for CPU-bound tasks
- **Streaming**: Process large files in chunks
- **Caching**: Redis/SQLite for converted documents
- **Batch operations**: Optimize multi-document workflows

---

## Development Roadmap (12 Months)

### Q1 2025: Foundation (Weeks 1-12)
- [x] Project structure and planning
- [ ] Core converter traits and interfaces
- [ ] Docling integration (PDF → Markdown)
- [ ] Basic optimization pipeline
- [ ] CLI tool

**Deliverable**: MVP with PDF conversion

### Q2 2025: Core Formats (Weeks 13-24)
- [ ] DOCX, PPTX, XLSX converters
- [ ] HTML/XML converters
- [ ] Image OCR (Tesseract)
- [ ] Quality optimization
- [ ] Batch processing

**Deliverable**: Beta with all document formats

### Q3 2025: Advanced Features (Weeks 25-36)
- [ ] Audio transcription (Whisper)
- [ ] Video processing (FFmpeg)
- [ ] Archive handling
- [ ] Advanced caching
- [ ] Performance tuning

**Deliverable**: Feature-complete v0.9.0

### Q4 2025: Integrations & Polish (Weeks 37-48)
- [ ] Vectorizer integration
- [ ] Python/Node.js bindings
- [ ] LangChain/LlamaIndex support
- [ ] WASM support
- [ ] Production hardening

**Deliverable**: v1.0.0 stable release

---

## Integration Strategy

### HiveLLM Vectorizer
```rust
// Direct integration example
let converter = Converter::new()?;
let vectorizer = VectorizerClient::new("http://localhost:15002").await?;

let result = converter
    .convert("document.pdf")
    .to(OutputFormat::EmbeddingReady)
    .pipe_to(&vectorizer)
    .execute()
    .await?;
```

### LangChain (Python)
```python
from langchain.document_loaders import TransmutationLoader

loader = TransmutationLoader("document.pdf")
documents = loader.load()
```

### LlamaIndex (Python)
```python
from llama_index.readers import TransmutationReader

reader = TransmutationReader()
documents = reader.load_data("document.pdf")
```

---

## Performance Targets

| Operation | Target | Notes |
|-----------|--------|-------|
| PDF → Markdown | 20 pages/s | 10MB document |
| DOCX → Markdown | 25 pages/s | 5MB document |
| Image OCR | 2 images/s | 1920x1080 |
| Batch (100 files) | 30 seconds | Mixed formats |
| Memory Usage | <500MB | Per conversion |

---

## Quality Metrics

### Text Optimization
- Token efficiency: >90% (remove unnecessary content)
- Semantic preservation: >95% (maintain meaning)
- Layout accuracy: >85% (complex documents)

### Image Optimization
- File size reduction: 50-70% (vs original)
- Visual quality: Minimal degradation
- OCR accuracy: >95% (clean documents)

---

## Docling Integration Details

### Supported Docling Features
- **docling-core**: Type definitions, DoclingDocument model
- **docling-parse**: Advanced PDF parsing with layout understanding
- **docling-ibm-models**: AI models for table/figure extraction
- **docling-mcp**: Model Context Protocol for agents

### Integration via PyO3
```rust
use pyo3::prelude::*;

pub struct DoclingEngine {
    py_module: PyObject,
}

impl DoclingEngine {
    pub async fn convert_pdf(&self, path: &Path) -> Result<DoclingDocument> {
        Python::with_gil(|py| {
            // Call Python Docling API
            let docling = py.import("docling")?;
            let converter = docling.getattr("DocumentConverter")?;
            let result = converter.call_method1("convert", (path,))?;
            
            // Convert to Rust types
            Ok(DoclingDocument::from_python(result)?)
        })
    }
}
```

### LangChain/LlamaIndex Integration
Transmutation will support Docling's existing integrations:
- LangChain document loaders
- LlamaIndex readers
- Haystack converters
- DSPy optimizers

---

## Development Environment

### Prerequisites
- Rust 1.75+ (Edition 2024)
- Python 3.9+ (for Docling)
- Tesseract OCR (optional)
- FFmpeg (optional)

### Setup
```bash
# Clone repository
git clone https://github.com/hivellm/transmutation
cd transmutation

# Install Python dependencies (for Docling)
python3 -m venv venv
source venv/bin/activate
pip install docling

# Build Rust project
cargo build --release

# Run tests
cargo test

# Run CLI
cargo run --bin transmutation -- --help
```

---

## Testing Strategy

### Unit Tests
- Individual converter tests
- Engine wrapper tests
- Utility function tests
- Target: >80% code coverage

### Integration Tests
- End-to-end conversion pipelines
- Multi-format workflows
- Error handling scenarios
- Performance regression tests

### Benchmark Suite
- Conversion speed benchmarks
- Memory usage profiling
- Comparison with alternatives
- CI/CD performance tracking

---

## Security Considerations

### Input Validation
- File type verification (magic bytes)
- Size limits per format
- Path traversal prevention
- Malicious content detection

### Sandboxing
- Separate processes for untrusted documents
- Resource limits (CPU, memory, time)
- Network isolation during conversion

### Output Sanitization
- Remove embedded scripts
- Strip potentially dangerous content
- Validate output formats

---

## Deployment Options

### 1. Library (Embedded)
```toml
[dependencies]
transmutation = "1.0"
```

### 2. CLI Tool
```bash
transmutation convert document.pdf --to markdown --output doc.md
```

### 3. API Server (Future)
```bash
transmutation serve --port 8080 --workers 4
```

### 4. Docker Container (Future)
```bash
docker run -v $(pwd):/data hivellm/transmutation convert /data/doc.pdf
```

---

## Success Criteria

### Technical
- ✅ Support for 20+ file formats
- ✅ >20 pages/second conversion speed
- ✅ <500MB memory per conversion
- ✅ >80% code coverage
- ✅ Zero unsafe code

### Adoption
- 🎯 1,000+ GitHub stars (first year)
- 🎯 10,000+ downloads (first quarter)
- 🎯 50+ production deployments
- 🎯 20+ community contributors

### Quality
- ✅ >95% OCR accuracy (clean documents)
- ✅ >90% transcription accuracy
- ✅ >85% layout preservation
- ✅ 100% format compatibility

---

## Risk Assessment

### High Risk
- ⚠️ **Docling Python dependency**: Mitigation: Implement fallback pure-Rust parsers
- ⚠️ **Performance bottlenecks**: Mitigation: Regular profiling and optimization

### Medium Risk
- ⚠️ **Model download sizes**: Mitigation: Lazy loading, caching
- ⚠️ **Maintenance burden**: Mitigation: Focus on code quality, documentation

### Low Risk
- ✅ File format compatibility (extensive test suite)
- ✅ Cross-platform support (Rust/Cargo handles this)

---

## Next Steps (Immediate)

### Week 1-2
1. [ ] Initialize Rust workspace
2. [ ] Set up CI/CD (GitHub Actions)
3. [ ] Configure linting/formatting
4. [ ] Create issue templates

### Week 3-4
5. [ ] Define core converter traits
6. [ ] Implement file type detection
7. [ ] Create error handling framework
8. [ ] Set up basic tests

### Week 5-6
9. [ ] Integrate PyO3 for Python bridge
10. [ ] Install Docling package
11. [ ] Implement PDF → Markdown converter
12. [ ] Write conversion tests

---

## Resources

### Documentation
- [Docling GitHub](https://github.com/docling-project)
- [Docling Documentation](https://docling-project.github.io/docling/)
- [PyO3 User Guide](https://pyo3.rs/)
- [Rust Async Book](https://rust-lang.github.io/async-book/)

### Reference Implementations
- [Converter Buddy](https://github.com/attilio-oliva/converter-buddy) - Rust image converter
- [pdf-extract](https://github.com/jrmuizel/pdf-extract) - Rust PDF text extraction
- [tesseract-rs](https://github.com/antimatter15/tesseract-ocr) - Rust Tesseract bindings

### Community
- HiveLLM Discord: https://discord.gg/hivellm
- GitHub Discussions: https://github.com/hivellm/transmutation/discussions
- Project Board: https://github.com/orgs/hivellm/projects/transmutation

---

## Contributors

- **Project Lead**: HiveLLM Team
- **Architecture**: TBD
- **Engineering**: Open for contributors!

---

## License

MIT License - See [LICENSE](LICENSE) for details.

---

**Document Version**: 1.0.0  
**Last Updated**: 2025-10-12  
**Status**: Planning Phase  
**Next Review**: 2025-10-19

