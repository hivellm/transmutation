# Transmutation Installation Guide

## Quick Start

### Linux (Debian/Ubuntu)

```bash
# Install dependencies
chmod +x install/install-deps-linux.sh
./install/install-deps-linux.sh

# Or manually:
sudo apt-get update
sudo apt-get install -y poppler-utils libreoffice
```

### macOS

```bash
# Install dependencies
chmod +x install/install-deps-macos.sh
./install/install-deps-macos.sh

# Or manually:
brew install poppler
brew install --cask libreoffice
```

### Windows

**Option 1: PowerShell + Chocolatey (Recommended)**
```powershell
# Run as Administrator
.\install\install-deps-windows.ps1
```

**Option 2: Batch + winget (Windows 10/11)**
```batch
REM Run as Administrator
.\install\install-deps-windows.bat
```

**Option 3: Manual Download (No package manager)**
```batch
REM Run as Administrator
.\install\install-deps-windows-manual.bat
```

**Manual installation:**
```powershell
# With Chocolatey
choco install poppler libreoffice tesseract ffmpeg -y

# With winget
winget install TheDocumentFoundation.LibreOffice
winget install UB-Mannheim.TesseractOCR
winget install Gyan.FFmpeg
```

---

## Dependencies

Transmutation uses external tools for certain conversions:

| Tool | Purpose | Linux | macOS | Windows |
|------|---------|-------|-------|---------|
| **Build Tools** | Compile C++/FFI | `apt install build-essential cmake` | Xcode Command Line Tools | VS Build Tools 2022 |
| **poppler-utils** | PDF → Image | `apt install poppler-utils` | `brew install poppler` | `choco install poppler` or winget |
| **LibreOffice** | DOCX → PDF → Image | `apt install libreoffice` | `brew install --cask libreoffice` | `choco install libreoffice` or winget |
| **Tesseract** | OCR for images | `apt install tesseract-ocr` | `brew install tesseract` | `choco install tesseract` or winget |
| **FFmpeg** | Audio/Video → Text | `apt install ffmpeg` | `brew install ffmpeg` | `choco install ffmpeg` or winget |

### Why External Dependencies?

**LibreOffice (DOCX → Image):**
- DOCX is a text format without fixed visual layout
- Rendering requires a layout engine (Word, LibreOffice, Google Docs)
- Docling uses the same approach (Pandoc or LibreOffice subprocess)
- No pure Rust library exists for DOCX rendering

**Poppler/pdftoppm (PDF → Image):**
- Industry-standard PDF rendering
- Fast and reliable
- Used by Docling, ImageMagick, and many others
- Alternative: Could use pdfium-render (requires libpdfium.so)

---

## Feature Matrix

| Feature | Dependencies | Pure Rust | Feature Flag |
|---------|--------------|-----------|--------------|
| **PDF → Markdown** | ✅ None | ✅ 100% Rust | `pdf` |
| **PDF → Images** | ⚠️ poppler-utils | ❌ | `pdf-to-image` |
| **DOCX → Markdown** | ✅ None | ✅ 100% Rust | `office` |
| **DOCX → Images** | ⚠️ LibreOffice + poppler | ❌ | `office,pdf-to-image` |
| **XLSX → Markdown** | ✅ None | ✅ 100% Rust | `office` |
| **PPTX → Markdown** | ✅ None | ✅ 100% Rust | `office` |
| **Image OCR** | ⚠️ Tesseract | ✅ Rust bindings | `tesseract` |
| **Audio → Text** | ⚠️ FFmpeg | ✅ Rust bindings | `audio` |
| **Video → Text** | ⚠️ FFmpeg | ✅ Rust bindings | `video` |
| **Split Pages (MD)** | ✅ None | ✅ 100% Rust | All |
| **Archives (ZIP/TAR)** | ✅ None | ✅ 100% Rust | `archives` |
| **HTML/XML** | ✅ None | ✅ 100% Rust | `web` |

---

## Building with Specific Features

You can compile Transmutation with only the features you need:

```bash
# Minimal build (PDF + Office only)
cargo build --release --features "pdf,office"

# Full build (all features)
cargo build --release --features "pdf,pdf-to-image,office,web,tesseract,audio,video,archives,cli"

# Without external dependencies (pure Rust only)
cargo build --release --features "pdf,office,web,archives,cli"

# With OCR support
cargo build --release --features "pdf,office,tesseract,cli"
```

### Feature Dependencies

| Feature | Requires External Tools |
|---------|------------------------|
| `pdf` | ❌ Pure Rust |
| `pdf-to-image` | ✅ poppler-utils |
| `office` | ❌ Pure Rust (MD), ✅ LibreOffice (Images) |
| `web` | ❌ Pure Rust |
| `tesseract` | ✅ Tesseract OCR |
| `audio` | ✅ FFmpeg |
| `video` | ✅ FFmpeg |
| `archives` | ❌ Pure Rust |
| `docling-ffi` | ✅ C++ build tools, docling-parse |

---

## Verification

After installation, verify:

```bash
# Check poppler
pdftoppm -v

# Check LibreOffice
libreoffice --version

# Windows
pdftoppm.exe -v
soffice.exe --version
```

---

## Platform-Specific Notes

### Linux
- **Ubuntu/Debian**: Use `apt-get` (tested on Ubuntu 24.04)
- **Fedora/RHEL**: Use `dnf install poppler-utils libreoffice`
- **Arch**: Use `pacman -S poppler libreoffice`

### macOS
- Requires Homebrew
- LibreOffice installs to `/Applications/LibreOffice.app`
- poppler installs to `/opt/homebrew/bin/pdftoppm`

### Windows
- Requires Chocolatey
- Must run PowerShell as Administrator
- poppler installs to `C:\ProgramData\chocolatey\bin\`
- LibreOffice installs to `C:\Program Files\LibreOffice\`

---

## Troubleshooting

### "Command not found: pdftoppm"
Install poppler-utils for your platform (see above)

### "Command not found: libreoffice/soffice"
Install LibreOffice for your platform (see above)

### Windows: "execution policy" error
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

---

## Optional: Docker

Don't want to install dependencies? Use Docker:

```bash
# Build Docker image with all dependencies
docker build -t transmutation .

# Run
docker run -v $(pwd)/data:/data transmutation convert /data/document.pdf
```

---

**Last Updated**: October 13, 2025  
**Supported Platforms**: Linux, macOS, Windows

