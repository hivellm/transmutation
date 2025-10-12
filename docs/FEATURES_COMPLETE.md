# Transmutation - Complete Feature Set

**Based on Docling v2.55.1 analysis + Planned improvements**

This document outlines ALL features that Transmutation will implement to compete with (and surpass) Docling.

---

## 📋 Supported Formats (Complete List)

### Document Formats (15 formats)

| Format | Input | Output | Priority | Status | Docling Equivalent |
|--------|-------|--------|----------|--------|-------------------|
| **PDF** | ✅ | Markdown, Image, JSON | Critical | 🔄 Week 3-8 | ✅ Via docling_parse_v4 |
| **DOCX** | ✅ | Markdown, Image, JSON | Critical | 🔄 Week 13-15 | ✅ Via python-docx |
| **PPTX** | ✅ | Markdown, Image, JSON | High | 🔄 Week 20-21 | ✅ Via python-pptx |
| **XLSX** | ✅ | Markdown, CSV, JSON | High | 🔄 Week 16-17 | ✅ Via openpyxl |
| **HTML** | ✅ | Markdown, JSON | High | 🔄 Week 18-19 | ✅ Via beautifulsoup4 |
| **XML** | ✅ | Markdown, JSON | High | 🔄 Week 18-19 | ✅ Generic |
| **JATS XML** | ✅ | Markdown, JSON | Medium | 📝 Q2 2025 | ✅ Scientific papers |
| **USPTO XML** | ✅ | Markdown, JSON | Medium | 📝 Q2 2025 | ✅ Patent documents |
| **CSV** | ✅ | Markdown, JSON | High | 🔄 Week 18-19 | ✅ Via pandas |
| **TSV** | ✅ | Markdown, JSON | Medium | 🔄 Week 18-19 | ✅ Via pandas |
| **Markdown** | ✅ | Normalized MD, JSON | Medium | 🔄 Week 18-19 | ✅ Via marko |
| **AsciiDoc** | ✅ | Markdown, JSON | Low | 📝 Q3 2025 | ✅ |
| **RTF** | ✅ | Markdown, JSON | Medium | 📝 Q2 2025 | ❌ Not in Docling |
| **ODT** | ✅ | Markdown, JSON | Medium | 📝 Q2 2025 | ❌ Not in Docling |
| **METS/GBS** | ✅ | Markdown, JSON | Low | 📝 Q3 2025 | ✅ Digital library format |

### Image Formats (6 formats + OCR)

| Format | OCR Engine | Priority | Status | Docling Equivalent |
|--------|-----------|----------|--------|-------------------|
| **JPG/JPEG** | Tesseract, RapidOCR | High | 🔄 Week 25-27 | ✅ Multiple OCR engines |
| **PNG** | Tesseract, RapidOCR | High | 🔄 Week 25-27 | ✅ Multiple OCR engines |
| **TIFF** | Tesseract, RapidOCR | High | 🔄 Week 25-27 | ✅ Multiple OCR engines |
| **BMP** | Tesseract, RapidOCR | Medium | 🔄 Week 25-27 | ✅ Multiple OCR engines |
| **GIF** | Tesseract, RapidOCR | Medium | 🔄 Week 25-27 | ✅ Multiple OCR engines |
| **WEBP** | Tesseract, RapidOCR | Medium | 🔄 Week 25-27 | ✅ Multiple OCR engines |

### Audio Formats (4 formats + ASR)

| Format | ASR Engine | Priority | Status | Docling Equivalent |
|--------|-----------|----------|--------|-------------------|
| **MP3** | Whisper (Rust) | Medium | 📝 Q3 2025 | ✅ Via Whisper |
| **WAV** | Whisper (Rust) | Medium | 📝 Q3 2025 | ✅ Via Whisper |
| **M4A** | Whisper (Rust) | Medium | 📝 Q3 2025 | ✅ Via Whisper |
| **FLAC** | Whisper (Rust) | Low | 📝 Q4 2025 | ❌ Not in Docling |
| **WebVTT** | Parser | Medium | 📝 Q3 2025 | ✅ New in v2.54 |

### Video Formats (4 formats)

| Format | Features | Priority | Status | Docling Equivalent |
|--------|----------|----------|--------|-------------------|
| **MP4** | Extract audio, keyframes | Medium | 📝 Q3 2025 | ⚠️ Indirect |
| **AVI** | Extract audio, keyframes | Low | 📝 Q4 2025 | ❌ Not in Docling |
| **MKV** | Extract audio, keyframes | Low | 📝 Q4 2025 | ❌ Not in Docling |
| **MOV** | Extract audio, keyframes | Low | 📝 Q4 2025 | ❌ Not in Docling |

### Archive Formats (5 formats)

| Format | Features | Priority | Status | Docling Equivalent |
|--------|----------|----------|--------|-------------------|
| **ZIP** | Recursive extraction | High | 🔄 Week 28-30 | ❌ Not in Docling |
| **TAR** | Recursive extraction | High | 🔄 Week 28-30 | ❌ Not in Docling |
| **TAR.GZ** | Recursive extraction | High | 🔄 Week 28-30 | ❌ Not in Docling |
| **TAR.BZ2** | Recursive extraction | Medium | 🔄 Week 28-30 | ❌ Not in Docling |
| **7Z** | Recursive extraction | Medium | 🔄 Week 28-30 | ❌ Not in Docling |

**Total Formats**: 34+ formats (Docling: ~20 formats)

---

## 🤖 AI/ML Features (Docling Strengths)

### Layout Understanding (Critical - Must Match)

| Feature | Implementation | Priority | Status | Notes |
|---------|---------------|----------|--------|-------|
| **Page Layout Detection** | Pure Rust heuristics | Critical | 🔄 Week 9-10 | Docling uses Heron model |
| **Reading Order** | Text position analysis | Critical | 🔄 Week 9-10 | Docling uses ML model |
| **Multi-column Detection** | Whitespace analysis | High | 🔄 Week 9-10 | Docling uses ML model |
| **Table Structure** | Grid detection | Critical | 🔄 Week 9-10 | Docling uses TableFormer |
| **Table Headers** | Heuristic detection | High | 🔄 Week 10 | Docling uses ML model |
| **Code Block Detection** | Syntax pattern matching | Medium | 📝 Q2 2025 | Docling uses ML model |
| **Formula Detection** | LaTeX pattern matching | Medium | 📝 Q2 2025 | Docling uses ML model |
| **Image Classification** | Basic type detection | Medium | 📝 Q3 2025 | Docling uses classifier |

### OCR Engines (Match Docling)

| Engine | Platform | Priority | Status | Notes |
|--------|----------|----------|--------|-------|
| **Tesseract** | All | Critical | 🔄 Week 25-27 | ✅ Same as Docling |
| **RapidOCR** | All | High | 📝 Q3 2025 | ✅ Same as Docling |
| **EasyOCR** | All | Medium | 📝 Q3 2025 | ✅ Same as Docling (optional) |
| **OCRMac** | macOS | Low | 📝 Q4 2025 | ✅ Same as Docling (macOS only) |

### Text Extraction (Docling's Core Strength)

| Feature | Implementation | Priority | Status |
|---------|---------------|----------|--------|
| **Text with position** | PDF operators | Critical | 🔄 Week 3-4 |
| **Font information** | PDF font tables | High | 🔄 Week 5-6 |
| **Text encoding** | Multi-encoding support | Critical | 🔄 Week 3-4 |
| **Ligatures handling** | Unicode normalization | Medium | 🔄 Week 6 |
| **Hyphenation removal** | Pattern matching | High | 🔄 Week 6 |

---

## 🎯 Advanced Features (Docling + Our Improvements)

### Document Understanding

| Feature | Docling | Transmutation | Status |
|---------|---------|---------------|--------|
| **Page layout analysis** | ✅ ML model | 🔄 Heuristics + ML (optional) | Week 9-10 |
| **Reading order** | ✅ ML model | 🔄 Position-based + ML (optional) | Week 9-10 |
| **Table detection** | ✅ TableFormer | 🔄 Grid detection + ML (optional) | Week 9-10 |
| **Table structure** | ✅ Cell-level | 🔄 Cell-level | Week 10 |
| **Code blocks** | ✅ ML model | 🔄 Pattern matching | Q2 2025 |
| **Formulas** | ✅ ML model | 🔄 LaTeX detection | Q2 2025 |
| **Image classification** | ✅ Classifier | 🔄 Basic types | Q3 2025 |
| **Heading hierarchy** | ✅ Font-based | 🔄 Font + position | Week 6 |
| **List detection** | ✅ Pattern | 🔄 Enhanced patterns | Week 6 |
| **Footnotes** | ✅ Position-based | 🔄 Position + markers | Week 10 |
| **References** | ⚠️ Coming soon | 🔄 Pattern matching | Q3 2025 |

### Export Formats

| Format | Docling | Transmutation | Notes |
|--------|---------|---------------|-------|
| **Markdown** | ✅ GFM | ✅ GFM + Custom | More compact for LLMs |
| **HTML** | ✅ | ✅ | Semantic HTML5 |
| **JSON** | ✅ DoclingDocument | ✅ Custom schema | Optimized for embeddings |
| **DocTags** | ✅ | 🔄 Planned Q4 | Docling proprietary format |
| **CSV** | ⚠️ Limited | ✅ | Enhanced for spreadsheets |
| **Plain Text** | ✅ | ✅ | With optional formatting |

---

## 🔌 Pipelines (Docling Architecture to Match)

### Processing Pipelines

| Pipeline | Purpose | Status | Implementation |
|----------|---------|--------|----------------|
| **Simple Pipeline** | Basic formats (DOCX, HTML, etc.) | 🔄 Week 5-6 | No ML required |
| **Standard PDF Pipeline** | PDF with ML models | 🔄 Week 7-8 | Optional ML, fallback to heuristics |
| **Threaded PDF Pipeline** | Parallel PDF processing | 🔄 Week 31 | Built-in with Rayon |
| **VLM Pipeline** | Visual Language Models | 📝 Q4 2025 | Optional integration |
| **ASR Pipeline** | Audio transcription | 📝 Q3 2025 | Pure Rust ASR |
| **Extraction Pipeline** | Structured extraction | 📝 Q4 2025 | Information extraction |

### Pipeline Features

| Feature | Docling | Transmutation | Priority |
|---------|---------|---------------|----------|
| **Batch processing** | ✅ | ✅ Enhanced | Critical |
| **Parallel workers** | ✅ | ✅ Rayon-based | Critical |
| **Progress tracking** | ✅ | ✅ | High |
| **Error recovery** | ✅ | ✅ | High |
| **Caching** | ⚠️ Limited | ✅ Redis/SQLite | High |
| **Streaming** | ⚠️ Limited | ✅ | Medium |

---

## 🧠 AI/ML Models (Competitive Edge)

### Docling's ML Models (We Must Match/Replace)

| Model | Purpose | Docling Solution | Transmutation Solution | Status |
|-------|---------|------------------|----------------------|--------|
| **Layout Model** | Page segmentation | Heron (newest), LayoutLMv3 | Pure Rust heuristics + optional ML | Week 9-10 |
| **Table Structure** | TableFormer | ML-based | Grid detection + optional ML | Week 9-10 |
| **Reading Order** | ML model | ML-based | Position-based algorithm | Week 9-10 |
| **Code Detector** | ML classifier | ML-based | Syntax pattern matching | Q2 2025 |
| **Formula Detector** | ML classifier | ML-based | LaTeX pattern matching | Q2 2025 |
| **Image Classifier** | CNN | ML-based | Basic type detection | Q3 2025 |
| **OCR** | Tesseract, RapidOCR, EasyOCR | Multiple engines | Same engines | Week 25-27 |
| **ASR** | Whisper | Python Whisper | Pure Rust ASR (whisper-rs) | Q3 2025 |

### Visual Language Models (VLM) Support

| Model | Docling | Transmutation | Notes |
|-------|---------|---------------|-------|
| **GraniteDocling** | ✅ Native | 🔄 Optional | Can integrate via API |
| **Qwen-VL** | ✅ | 🔄 Optional | Via external API |
| **MLX acceleration** | ✅ macOS | 🔄 Future | Apple Silicon |
| **VLLM** | ✅ Linux | 🔄 Future | GPU acceleration |

**Strategy**: Transmutation focuses on pure Rust heuristics first, with optional ML model integration later.

---

## 🎨 Advanced Features

### PDF Understanding (Must Match Docling)

| Feature | Description | Status | Implementation |
|---------|-------------|--------|----------------|
| **Page layout** | Detect headers, footers, columns | 🔄 Week 9 | Position analysis |
| **Reading order** | Correct text flow | 🔄 Week 9 | Z-order + position |
| **Table structure** | Rows, columns, merged cells | 🔄 Week 9-10 | Whitespace grid analysis |
| **Code blocks** | Detect code in PDFs | 📝 Q2 | Monospace font detection |
| **Formulas** | LaTeX, MathML | 📝 Q2 | Pattern matching |
| **Image extraction** | Extract embedded images | 🔄 Week 8 | PDF object extraction |
| **Image classification** | Chart, diagram, photo | 📝 Q3 | Basic heuristics |
| **Encrypted PDFs** | Password-protected | 📝 Q2 | lopdf supports |
| **Form fields** | Extract form data | 📝 Q3 | PDF form parsing |

### Document Metadata Extraction

| Metadata | PDF | DOCX | PPTX | XLSX | Priority |
|----------|-----|------|------|------|----------|
| **Title** | ✅ | ✅ | ✅ | ✅ | High |
| **Author** | ✅ | ✅ | ✅ | ✅ | High |
| **Creation date** | ✅ | ✅ | ✅ | ✅ | High |
| **Modified date** | ✅ | ✅ | ✅ | ✅ | High |
| **Page/slide count** | ✅ | ✅ | ✅ | ✅ | Critical |
| **Language** | ✅ | ✅ | ✅ | ⚠️ | Medium |
| **Keywords** | ✅ | ✅ | ⚠️ | ⚠️ | Low |
| **Subject** | ✅ | ✅ | ⚠️ | ⚠️ | Low |
| **Producer** | ✅ | ⚠️ | ⚠️ | ⚠️ | Low |

### LLM Optimization (Transmutation Advantage)

| Feature | Docling | Transmutation | Status |
|---------|---------|---------------|--------|
| **Token counting** | ⚠️ Basic | ✅ Accurate (tiktoken-rs) | Week 22 |
| **Smart chunking** | ✅ Via docling-core | ✅ Enhanced algorithms | Week 22-23 |
| **Context window optimization** | ⚠️ Basic | ✅ Advanced | Week 23 |
| **Semantic boundaries** | ⚠️ Limited | ✅ Sentence-aware | Week 23 |
| **Overlap strategies** | ✅ | ✅ Configurable | Week 23 |
| **Compact output** | ⚠️ | ✅ Optimized | Week 6 |
| **Metadata removal** | ⚠️ | ✅ Configurable | Week 6 |

---

## 🔧 Processing Features

### Page Processing

| Feature | Implementation | Status | Notes |
|---------|---------------|--------|-------|
| **Page range selection** | Config option | 🔄 Week 7 | Process specific pages |
| **Page size detection** | PDF/DOCX metadata | 🔄 Week 4 | A4, Letter, Custom |
| **Page orientation** | Auto-detect | 🔄 Week 4 | Portrait, Landscape |
| **Page rotation** | PDF operators | 📝 Q2 | Rotate before processing |
| **Page cropping** | Margin detection | 📝 Q2 | Remove margins |

### Text Processing

| Feature | Implementation | Status | Notes |
|---------|---------------|--------|-------|
| **Whitespace normalization** | Regex + heuristics | 🔄 Week 6 | Remove excessive spaces |
| **Line break handling** | Context-aware | 🔄 Week 6 | Smart paragraph merging |
| **Hyphenation removal** | Pattern matching | 🔄 Week 6 | End-of-line hyphens |
| **Ligature handling** | Unicode normalization | 🔄 Week 6 | fi, fl, etc. |
| **Encoding detection** | Auto-detect | 🔄 Week 4 | UTF-8, Latin1, etc. |
| **Language detection** | Pattern-based | 📝 Q3 | Optional feature |

### Table Processing

| Feature | Implementation | Status | Notes |
|---------|---------------|--------|-------|
| **Table detection** | Grid analysis | 🔄 Week 9-10 | Whitespace-based |
| **Cell extraction** | Position-based | 🔄 Week 10 | Row/column parsing |
| **Merged cells** | Span detection | 🔄 Week 10 | Handle colspan/rowspan |
| **Table headers** | First row heuristic | 🔄 Week 10 | Bold/position based |
| **Nested tables** | Recursive parsing | 📝 Q2 | Complex tables |
| **Table captions** | Position-based | 📝 Q2 | Above/below detection |

### Image Processing

| Feature | Implementation | Status | Notes |
|---------|---------------|--------|-------|
| **Image extraction** | PDF objects | 🔄 Week 8 | Extract embedded images |
| **Image compression** | WebP, JPEG | 🔄 Week 8 | Optimize file size |
| **Image preprocessing** | Deskew, denoise | 🔄 Week 26 | Better OCR results |
| **Image upscaling** | Bicubic | 📝 Q3 | For low-res images |
| **Thumbnail generation** | Resize | 📝 Q3 | Quick previews |

---

## 🚀 Performance Features (Transmutation Advantages)

| Feature | Docling | Transmutation | Advantage |
|---------|---------|---------------|-----------|
| **Startup time** | ~5-10s | <100ms | **50-100x faster** |
| **Memory usage** | ~2-3GB | <500MB | **4-6x less** |
| **Processing speed** | Baseline | 10x target | **10x faster** |
| **Parallel processing** | ✅ Python threads | ✅ Rayon | **True parallelism** |
| **Binary size** | ~500MB (with models) | <50MB | **10x smaller** |
| **Dependencies** | Python + 50+ packages | Zero runtime | **No dependencies** |

### Caching (Transmutation Exclusive)

| Feature | Docling | Transmutation | Status |
|---------|---------|---------------|--------|
| **In-memory cache** | ❌ | ✅ | Week 31 |
| **SQLite cache** | ❌ | ✅ | Week 32 |
| **Redis cache** | ❌ | ✅ | Week 32 |
| **Hash-based dedup** | ❌ | ✅ | Week 32 |
| **TTL support** | ❌ | ✅ | Week 32 |

---

## 🔌 Integrations (Match + Extend)

### Framework Integrations

| Framework | Docling | Transmutation | Status |
|-----------|---------|---------------|--------|
| **LangChain** | ✅ Native | ✅ | Week 37 |
| **LlamaIndex** | ✅ Native | ✅ | Week 38 |
| **Haystack** | ✅ Native | ✅ | Week 38 |
| **CrewAI** | ✅ | ✅ | Week 38 |
| **DSPy** | ⚠️ | ✅ | Week 39 |
| **Vectorizer** | ❌ | ✅ Native | Week 37 |

### MCP (Model Context Protocol)

| Feature | Docling | Transmutation | Status |
|---------|---------|---------------|--------|
| **MCP Server** | ✅ | ✅ | Week 40 |
| **Convert tool** | ✅ | ✅ | Week 40 |
| **Batch tool** | ⚠️ | ✅ | Week 40 |
| **Extract tool** | ✅ | ✅ | Week 40 |

---

## 📤 Output Features

### Markdown Options

| Feature | Status | Notes |
|---------|--------|-------|
| **GitHub Flavored Markdown** | 🔄 Week 5 | Default |
| **CommonMark** | 📝 Q2 | Strict compliance |
| **Custom flavor** | 📝 Q3 | LLM-optimized |
| **Table syntax** | 🔄 Week 10 | GFM tables |
| **Code blocks** | 🔄 Week 10 | With language hints |
| **Math blocks** | 📝 Q2 | LaTeX math |
| **Footnotes** | 📝 Q2 | Markdown syntax |
| **Front matter** | 📝 Q2 | YAML metadata |

### JSON Schema

| Feature | Status | Notes |
|---------|--------|-------|
| **Document structure** | 🔄 Week 12 | Hierarchical |
| **Page-level data** | 🔄 Week 12 | Per-page JSON |
| **Metadata** | 🔄 Week 12 | Rich metadata |
| **Positioning info** | 📝 Q2 | Text coordinates |
| **Confidence scores** | 📝 Q2 | OCR confidence |

---

## 🆕 NEW Features (Beyond Docling)

### Features Docling Doesn't Have

| Feature | Description | Status | Priority |
|---------|-------------|--------|----------|
| **Archive support** | ZIP, TAR, 7Z extraction | Week 28-30 | High |
| **Advanced caching** | Redis, SQLite | Week 32 | High |
| **Watch mode** | Auto-convert on changes | Week 40 | Medium |
| **Config files** | YAML/TOML configuration | Week 40 | Medium |
| **Output templates** | Custom Markdown templates | Week 40 | Medium |
| **Dry-run mode** | Test without conversion | Week 40 | Low |
| **Diff support** | Compare document versions | Q4 2025 | Low |
| **Merge documents** | Combine multiple documents | Q4 2025 | Low |

---

## 📊 Feature Comparison Summary

### Format Support
- **Docling**: ~20 formats
- **Transmutation**: 34+ formats (includes archives, more video/audio)

### Performance
- **Docling**: Python-based, ML models required
- **Transmutation**: Pure Rust, 10x faster, optional ML

### Deployment
- **Docling**: Python + dependencies + models (~2GB)
- **Transmutation**: Single binary (<50MB)

### AI Features
- **Docling**: Heavy ML models for layout/tables
- **Transmutation**: Smart heuristics + optional ML

### Integrations
- **Docling**: LangChain, LlamaIndex, Haystack, CrewAI
- **Transmutation**: Same + Vectorizer + MCP

---

## 🎯 Implementation Strategy

### Phase 1 (Week 1-12): Match Basic PDF
- Core PDF features
- Text extraction
- Basic layout detection
- Markdown output

### Phase 2 (Week 13-24): Match Office Formats
- DOCX, PPTX, XLSX
- HTML, XML
- Table handling
- LLM optimization

### Phase 3 (Week 25-36): Match OCR/Media
- Image OCR (multiple engines)
- Archive support
- Batch processing
- Caching

### Phase 4 (Week 37-48): Exceed Docling
- Vectorizer integration
- Advanced caching
- CLI enhancements
- Multi-language bindings

---

## 🏆 Competitive Advantages

### Where Transmutation Will Excel

1. **Performance**: 10x faster (pure Rust vs Python)
2. **Memory**: 4-6x less (no ML models in memory)
3. **Deployment**: Single binary (vs Python + 50+ packages)
4. **Startup**: <100ms (vs 5-10s)
5. **Archive support**: Native (Docling doesn't have)
6. **Caching**: Built-in (Docling doesn't have)
7. **Cross-platform**: True native (vs Python wrapper)

### Where Docling Has Advantages (Initially)

1. **ML models**: Pre-trained for layout/tables
2. **Accuracy**: ML-based detection more accurate
3. **VLM support**: Native integration
4. **Community**: Established user base
5. **LF AI backing**: Foundation support

### Our Mitigation

1. **Heuristics + optional ML**: Start with heuristics, add ML later
2. **Accuracy through testing**: Extensive test suite
3. **API integration**: Can call VLM APIs
4. **Better performance**: Speed/efficiency attracts users
5. **Open source**: Community-driven development

---

**Status**: Planning Complete - Ready for Implementation  
**Next**: Implement PDF parser (Week 3-4)

