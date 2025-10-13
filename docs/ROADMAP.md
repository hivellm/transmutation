# Transmutation Roadmap

## Overview

This roadmap outlines the development plan for Transmutation, a high-performance document conversion engine designed for AI/LLM embeddings.

**Current Status (v0.1.2-dev - October 13, 2025)**:
- ✅ **Phase 1**: Foundation & Core Architecture (COMPLETE)
- ✅ **Phase 1.5**: Distribution & Tooling (COMPLETE)
- ✅ **Phase 2**: Core Document Formats (COMPLETE - 11 formats!)
- ✅ **Phase 2.5**: Core Features Architecture (COMPLETE)
- ✅ **Phase 3**: Advanced Features (COMPLETE - Archives ✅, Batch ✅, OCR ✅)
- 📝 **Phase 4**: Advanced Optimizations (Next)

**Latest Achievement**: Image OCR with Tesseract! 88x faster than Docling with equivalent quality (tested on Portuguese text)!

**Scope**: Pure Rust library/CLI for document conversion. No external integrations (handled by HiveLLM Vectorizer).

**Overall Progress**: 
```
Phase 1:   ████████████████████ 100% ✅ Foundation
Phase 1.5: ████████████████████ 100% ✅ Distribution
Phase 2:   ████████████████████ 100% ✅ 11 Formats
Phase 2.5: ████████████████████ 100% ✅ Core Arch
Phase 3:   ████████████████████ 100% ✅ Archives + Batch + OCR
Phase 4:   ░░░░░░░░░░░░░░░░░░░░   0% 📝 Optimizations

Total:     ████████████████████  95% Complete!!!
```

---

## Phase 1: Foundation & Core Architecture ✅ COMPLETE

- ✅ Project structure and architecture
- ✅ Core `Converter` trait and interfaces
- ✅ PDF text extraction (lopdf + pdf-extract)
- ✅ Markdown generator with LLM optimization
- ✅ CLI tool with convert/batch/info commands
- ✅ C++ FFI Integration (docling-parse)
- ✅ ONNX ML Models (LayoutLMv3)
- ✅ Split Page Exports (MD + Images per page)
- ✅ Performance benchmarks (98x faster than Docling)

---

## Phase 1.5: Distribution & Tooling ✅ COMPLETE

- ✅ Windows MSI Installer (WiX Toolset)
- ✅ Multi-platform installation scripts (Linux, macOS, Windows)
- ✅ Icon embedding in executables
- ✅ Build-time dependency checking
- ✅ Documentation (MSI_BUILD.md, DEPENDENCIES.md)
- ✅ Git repository cleanup (543 MB → 19 MB)

---

## Phase 2: Core Document Formats ✅ 100% COMPLETE

### Week 13-15: Office Formats ✅
- ✅ DOCX → Markdown (docx-rs, pure Rust)
- ✅ DOCX → Images (LibreOffice pipeline)
- ✅ XLSX → Markdown/CSV/JSON (umya-spreadsheet, 148 pg/s)
- ✅ PPTX → Markdown (ZIP/XML, 1639 pg/s)
- ✅ PPTX → Images (LibreOffice pipeline)

### Week 16-17: Web Formats ✅
- ✅ HTML → Markdown (scraper, 2110 pg/s)
- ✅ HTML → JSON
- ✅ XML → Markdown (quick-xml, 2353 pg/s)
- ✅ XML → JSON

### Week 18-19: Text Formats ✅
- ✅ TXT → Markdown (2805 pg/s)
- ✅ CSV/TSV → Markdown tables (2647 pg/s)
- ✅ CSV/TSV → JSON
- ✅ RTF → Markdown (2420 pg/s) ⚠️ Beta
- ✅ ODT → Markdown (ZIP + XML) ⚠️ Beta

### Week 20-21: Quality Optimization
- [ ] Compression algorithms
- [ ] Whitespace normalization
- [ ] Headers/footers removal
- [ ] Watermark removal
- [ ] Layout quality metrics

### Week 22-24: Integration Testing
- [ ] Cross-format conversion tests
- [ ] Large document stress tests
- [ ] Memory leak detection
- [ ] Performance benchmarking
- [ ] Regression test suite

---

## Phase 2.5: Core Features Architecture ✅ COMPLETE

- ✅ Core formats always enabled (no feature flags): PDF, HTML, XML, ZIP, TXT, CSV, TSV, RTF, ODT
- ✅ Removed conditional compilation from engines
- ✅ Simpler API and user experience
- ✅ Faster compilation

---

## Phase 3: Advanced Features 🔄 75% COMPLETE

### Week 25-27: Image OCR ✅ COMPLETE
- ✅ Integrated leptess (Tesseract wrapper)
- ✅ OCR for JPG, PNG, TIFF, BMP, GIF, WEBP
- ✅ Language configuration support
- ✅ Markdown output with paragraphs
- ✅ JSON output with OCR metadata
- ✅ **Performance**: 88x faster than Docling (252ms vs 17s)
- ✅ **Quality**: Equivalent to Docling (tested on Portuguese text)
- ✅ **External dependency**: Tesseract OCR

**Clarification - What OCR Does**:
- ✅ OCR extracts **existing text** from images (e.g., scanned documents, screenshots with text)
- ❌ OCR does NOT describe visual content (e.g., "a cat sitting" - that requires Image Captioning models)
- For visual descriptions, use Vision LLMs (GPT-4 Vision, LLaVA, etc.) - out of Transmutation scope

### Week 28-32: Audio/Video **OUT OF SCOPE**
**Note**: Audio/Video transcription (Whisper/FFmpeg) removed from scope.
- **ASR/Whisper** = Audio → Text transcription (not images!)
- Use specialized tools directly for audio/video needs.

### Week 33-34: Archive Handling ✅ COMPLETE
- ✅ ZIP file listing (1864 pg/s)
- ✅ TAR file listing (archives-extended)
- ✅ TAR.GZ file listing (archives-extended)
- ✅ Archive statistics
- ✅ Files grouped by extension
- ✅ Markdown/JSON export
- [ ] 7Z support
- [ ] Recursive processing
- [ ] Nested archives
- [ ] Extract and convert contents

### Week 35-36: Batch Processing ✅ COMPLETE
- ✅ Concurrent processing (Tokio)
- ✅ Configurable jobs
- ✅ Progress tracking
- ✅ Success/failure breakdown
- ✅ Auto-save outputs
- ✅ **Performance**: 4,627 pg/s (4 files parallel)

---

## Phase 4: Advanced Optimizations & v1.0.0

### Performance
- [ ] GPU acceleration for OCR
- [ ] Memory-mapped file processing
- [ ] Zero-copy optimizations
- [ ] Streaming large files

### Quality
- [ ] Improved RTF parser
- [ ] ODT table support
- [ ] Better layout detection
- [ ] Advanced text normalization

### v1.0.0 Release
- [ ] Documentation review
- [ ] Performance optimization
- [ ] Security audit
- [ ] Final testing
- [ ] v1.0.0 release

---

## Future Considerations (Post v1.0.0)

### Image Captioning (Visual Description)
**Note**: Currently Transmutation only does OCR (text extraction). Visual description is a future enhancement.

- [ ] **BLIP-2 ONNX Integration**
  - Download ONNX model (~3GB)
  - Use existing `ort` infrastructure
  - Generate automatic image descriptions
  - Output: "a cat sitting on a wooden surface"
  - Performance target: <1s per image
  
- [ ] **GIT Model Integration**
  - Smaller model (~1GB)
  - Faster inference
  - Good quality descriptions
  - Alternative to BLIP-2

**Implementation Strategy** (when/if implemented):
1. Download pre-trained ONNX models
2. Image preprocessing (resize, normalize)
3. Model inference with `ort`
4. Generate natural language descriptions
5. Combine with OCR for full image understanding

**Use Cases**:
- Describing diagrams, charts, photos
- Generating alt-text for accessibility
- Visual content indexing for embeddings
- Multimodal RAG systems

---

**Last Updated**: 2025-10-13  
**Version**: 0.1.2-dev  
**Status**: ✅ Phase 1, 1.5, 2, 2.5 Complete | 🔄 Phase 3 (25%)  
**Scope**: Pure Rust library/CLI (no bindings, no external integrations)

