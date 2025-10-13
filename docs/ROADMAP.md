# Transmutation Roadmap

## Overview

This roadmap outlines the development plan for Transmutation, a high-performance document conversion engine designed for AI/LLM embeddings.

**Current Status (v0.1.2-dev - October 13, 2025)**:
- ✅ **Phase 1**: Foundation & Core Architecture (COMPLETE)
- ✅ **Phase 1.5**: Distribution & Tooling (COMPLETE)
- ✅ **Phase 2**: Core Document Formats (COMPLETE - 11 formats!)
- ✅ **Phase 2.5**: Core Features Architecture (COMPLETE)
- 🔄 **Phase 3**: Advanced Features (25% COMPLETE - Archives ✅, Batch ✅)
- 📝 **Phase 4**: Advanced Optimizations (Planned)

**Latest Achievement**: TAR/GZ support + Batch Processing with Tokio! 4,627 pages/sec in parallel processing!

**Scope**: Pure Rust library/CLI for document conversion. No external integrations (handled by HiveLLM Vectorizer).

**Overall Progress**: 
```
Phase 1:   ████████████████████ 100% ✅ Foundation
Phase 1.5: ████████████████████ 100% ✅ Distribution
Phase 2:   ████████████████████ 100% ✅ 11 Formats
Phase 2.5: ████████████████████ 100% ✅ Core Arch
Phase 3:   █████░░░░░░░░░░░░░░░  25% 🔄 Archives + Batch
Phase 4:   ░░░░░░░░░░░░░░░░░░░░   0% 📝 Optimizations

Total:     ████████████████░░░░  80% Complete (core features)
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

## Phase 3: Advanced Features 🔄 8% COMPLETE

### Week 25-27: Image OCR
- [ ] Integrate tesseract-rs/leptess
- [ ] OCR for JPG, PNG, TIFF, BMP, GIF, WEBP
- [ ] Language detection
- [ ] Preprocessing (deskew, denoise)
- [ ] Confidence scoring
- [ ] Multi-column layout support

### Week 28-30: Audio Transcription
- [ ] Integrate whisper-rs
- [ ] Transcription for MP3, WAV, M4A
- [ ] Language detection
- [ ] Speaker diarization
- [ ] Timestamps and metadata
- [ ] Long-form audio chunking

### Week 31-32: Video Processing
- [ ] Integrate ffmpeg-next
- [ ] Video → keyframe extraction
- [ ] Video → audio → transcription pipeline
- [ ] Scene detection
- [ ] Video metadata extraction
- [ ] Thumbnail generation

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

## Phase 4: Advanced Optimizations

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

**Last Updated**: 2025-10-13  
**Version**: 0.1.2-dev  
**Status**: ✅ Phase 1, 1.5, 2, 2.5 Complete | 🔄 Phase 3 (25%)  
**Scope**: Pure Rust library/CLI (no bindings, no external integrations)

