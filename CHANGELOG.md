# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## Version History

| Version | Date | Type | Description |
|---------|------|------|-------------|
| [0.1.1](#011---2025-10-13) | 2025-10-13 | **Distribution** | MSI installer, icons, automated scripts |
| [0.1.0](#010---2025-10-13) | 2025-10-13 | **Initial** | Core PDF/DOCX conversion, 98x faster than Docling |

---

## [Unreleased]

### Added
- **TXT Converter**: Plain text to Markdown (Pure Rust)
  - Automatic paragraph detection
  - Heading detection (all caps or ending with colon)
  - **Performance**: 2,805 pages/sec (0.36ms)
  - TXT → JSON with content metadata
  - Zero dependencies
  
- **CSV/TSV Converter**: Spreadsheet data to Markdown/JSON (Pure Rust)
  - CSV/TSV → Markdown tables (clean formatting)
  - CSV/TSV → JSON structured output
  - Header row detection
  - **Performance**: 2,647 pages/sec (0.38ms)
  - Zero dependencies

### Previous Additions
- **XLSX Converter**: Excel to Markdown/CSV/JSON (Pure Rust)
  - Direct XML parsing with umya-spreadsheet (no LibreOffice!)
  - CSV export with proper quoting
  - JSON export with structured data
  - Markdown tables (clean formatting)
  - **Performance**: 148 pages/sec (6.7ms per file)
  - 224x faster than LibreOffice approach
  
- **PPTX Converter**: PowerPoint with dual-mode approach
  - **Text**: Direct XML parsing from ZIP (1,639 pages/sec!)
  - **Images**: LibreOffice → PDF → Images (when needed)
  - Clean text output (vs garbage from PDF)
  - 2,666x faster than LibreOffice for text
  - Split slide export
  
- **HTML Converter**: Web page to Markdown (Pure Rust)
  - Semantic HTML parsing with scraper/html5ever
  - Preserves links, headings, lists, code blocks
  - Handles formatting (strong, em, pre)
  - **Performance**: 2,110 pages/sec (0.47ms)
  - HTML → JSON with raw + markdown
  
- **XML Converter**: XML to JSON/Markdown (Pure Rust)
  - Fast parsing with quick-xml
  - XML → JSON structure preservation
  - XML → Markdown text extraction
  - **Performance**: 2,353 pages/sec (0.42ms)

### Changed
- Office formats now use direct parsers (not LibreOffice) for text
- CI only runs with pure Rust features (no external deps)
- build-ffi.yml only triggers on tags or manual dispatch

### Fixed
- CI build errors with leptonica/tesseract dependencies
- PPTX text output (was garbage, now clean)
- Removed --all-features from CI (caused dep errors)

---

## [0.1.1] - 2025-10-13

**Distribution & Tooling Release**

This release focuses on improving distribution, installation, and user experience with professional packaging and automated dependency management.

### Added
- **Windows MSI Installer**: Professional installer with automatic dependency detection
  - Three installation methods: Chocolatey, winget, and manual download
  - Automatic WiX Toolset detection (supports v3.11 and v3.14)
  - Embedded MIT License in installer UI
  - Start Menu shortcuts with custom icons
  - System PATH integration
  - Uninstaller support
- **Application Icons**: Custom branding throughout
  - Icon embedded in Windows executable (`transmutation.exe`)
  - Icon in MSI installer
  - Icon in Start Menu shortcuts
  - Icon in Add/Remove Programs
- **Automated Installation Scripts**:
  - `install/install-deps-linux.sh` - Ubuntu/Debian dependency installer
  - `install/install-deps-macos.sh` - Homebrew dependency installer
  - `install/install-deps-windows.ps1` - Chocolatey dependency installer
  - `install/install-deps-windows.bat` - winget dependency installer
  - `install/install-deps-windows-manual.bat` - Manual download installer
  - `install-wix.ps1` - WiX Toolset quick installer
- **Build-time Dependency Checking**: 
  - Automatic detection of missing external tools
  - Platform-specific installation instructions
  - Graceful fallback when dependencies unavailable
- **Documentation Improvements**:
  - `docs/MSI_BUILD.md` - Complete MSI build guide
  - `docs/MSI_DEPENDENCIES.md` - Dependency management strategies
  - `docs/DEPENDENCIES.md` - Runtime dependency guide
  - `install/README.md` - Installation instructions for all platforms
  - All documentation consolidated in `/docs` directory

### Changed
- Suppressed all compiler warnings via `.cargo/config.toml` (`-A warnings`)
- Improved WiX Toolset detection supporting multiple versions (v3.11, v3.14)
- Enhanced `build-msi.ps1` with automatic WiX installation via Chocolatey
- Removed emoji characters from PowerShell scripts for better compatibility
- Streamlined `wix/main.wxs` for cargo-wix compatibility
- Updated README with MSI installation instructions

### Fixed
- PowerShell script encoding issues with Unicode characters
- WiX path detection for multiple installation locations
- DOCX file format detection by inspecting ZIP contents (Office formats are ZIP files)
- MSI license showing "Lorem ipsum" placeholder (now shows real MIT License)
- `cargo-wix` compatibility with custom WiX configurations

### Technical
- Added `winres` build dependency for Windows resource embedding
- Enhanced `build.rs` with Windows executable metadata
- Icon resource compilation integrated into build process
- Cross-platform path handling in build scripts

---

## [0.1.0] - 2025-10-13

### Added

#### Core Features
- **PDF Conversion**: Pure Rust PDF to Markdown conversion
  - Fast mode: 80% similarity, 250x faster than Docling
  - Precision mode: 82% similarity, 94x faster than Docling
  - FFI mode: 95%+ similarity with C++ docling-parse integration
- **DOCX Conversion**: Office document to Markdown (pure Rust)
- **CLI Tool**: Full-featured command-line interface
  - Convert documents: `transmutation convert input.pdf -o output.md`
  - Batch processing support
  - Multiple output formats (Markdown, JSON, Images)

#### Document Processing
- Intelligent paragraph joining algorithm
- Author detection and grouping
- Heading detection (title, abstract, sections)
- Text cleanup and normalization (220+ character mappings)
- Smart character joining for perfect word spacing
- Table detection and formatting
- Image extraction

#### Performance
- **98x faster** than Docling on average (tested on 97 papers)
- **63.98 pages/second** processing speed
- **50MB memory footprint** (vs 2-3GB for Docling)
- **4.8MB single binary** deployment
- Processed 3,006 pages in 46.9 seconds

#### Architecture
- Modular engine system
- Pure Rust implementations (no Python runtime)
- Optional C++ FFI for maximum accuracy
- Async/tokio-based pipeline
- Feature flags for selective compilation

#### Output Formats
- **Markdown**: Optimized for LLM processing
  - Full document export
  - Split by pages
- **Images**: Per-page PNG/JPEG/WebP
  - Configurable DPI
  - Batch export
- **JSON**: Structured document data

#### Build & Distribution
- Cross-platform support (Linux, macOS, Windows)
- Cargo workspaces integration
- Docker support
- WSL compatibility for FFI builds

#### Documentation
- Comprehensive setup guide (`docs/SETUP.md`)
- CLI usage guide (`docs/CLI_GUIDE.md`)
- FFI integration guide (`docs/FFI.md`)
- Benchmark comparisons (`docs/BENCHMARKS.md`)
- Architecture documentation (`docs/ARCHITECTURE.md`)
- Roadmap (`docs/ROADMAP.md`)

#### Benchmarks
- Tested on 97 arXiv papers (3,006 pages total)
- Average speed: 63.98 pages/second
- Success rate: 95.9%
- Output compression: 55x (528 MB → 9.6 MB)
- Fastest conversion: 168.75 pages/second
- Slowest conversion: 6.0 pages/second

### Technical Details

#### Dependencies
- Rust 1.85+ (Edition 2024)
- Optional: WiX Toolset (for MSI generation)
- Optional: poppler-utils (for PDF → Image)
- Optional: LibreOffice (for DOCX → Image)
- Optional: Tesseract (for OCR)
- Optional: FFmpeg (for audio/video)

#### Features Flags
- `pdf` - PDF conversion (default)
- `office` - DOCX/XLSX/PPTX support (default)
- `web` - HTML/XML conversion (default)
- `pdf-to-image` - PDF rendering to images
- `docling-ffi` - C++ FFI for 95%+ accuracy
- `tesseract` - OCR support
- `audio` - Audio transcription
- `video` - Video processing
- `cli` - Command-line interface

#### Project Structure
```
transmutation/
├── src/
│   ├── converters/     # Document converters (PDF, DOCX, etc)
│   ├── engines/        # Processing engines
│   ├── document/       # Document model and serialization
│   ├── ml/             # Machine learning (ONNX)
│   ├── pipeline/       # Processing pipeline
│   └── bin/            # CLI binary
├── docs/               # Documentation
├── wix/                # MSI installer configuration
├── install/            # Installation scripts
└── assets/             # Icons and resources
```

### Known Issues
- ML models (LayoutLMv3) not yet integrated
- Table structure detection is rule-based (ML version pending)
- DOCX image export requires LibreOffice (cross-platform limitation)

### Breaking Changes
- None (initial release)

---

## Release Notes

### How to Upgrade

**From source:**
```bash
git pull origin main
cargo build --release --features cli
```

**Via Cargo:**
```bash
cargo install transmutation --force
```

**Windows MSI:**
```powershell
# Uninstall old version
msiexec /x transmutation-*.msi /qn

# Install new version
msiexec /i transmutation-0.1.0-x86_64.msi
```

### Compatibility

- **Minimum Rust Version**: 1.85 (Edition 2024)
- **Supported Platforms**: Windows 10+, Linux (Ubuntu 20.04+), macOS 12+
- **API Stability**: No stability guarantees until 1.0.0

---

## Roadmap

See [ROADMAP.md](docs/ROADMAP.md) for detailed development plans.

### Upcoming (0.2.0)
- Full ONNX ML model integration
- Advanced table structure detection
- PPTX and XLSX conversion
- Python/Node.js bindings

### Future (1.0.0)
- Stable API
- WebAssembly support
- LangChain/LlamaIndex integration
- Production-ready ML pipeline

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

---

## Links

- **Repository**: https://github.com/hivellm/transmutation
- **Documentation**: https://docs.hivellm.org/transmutation
- **Issues**: https://github.com/hivellm/transmutation/issues
- **Releases**: https://github.com/hivellm/transmutation/releases

---

**Built with ❤️ by the HiveLLM Team**

