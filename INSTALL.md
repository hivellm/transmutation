# Installation Guide

## Requirements

### Rust Version
**Transmutation requires Rust 1.85+ with Edition 2024 support.**

### Pure Rust - Zero Runtime Dependencies
Unlike Docling (which requires Python), Transmutation is 100% Rust with:
- ✅ No Python runtime needed
- ✅ No ML model downloads
- ✅ Single binary deployment
- ✅ Fast startup (<100ms)

## Install/Update Rust

### Option 1: Install rustup (Recommended)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Option 2: Update existing Rust
```bash
rustup update stable
```

### Option 3: Use nightly (for Edition 2024)
```bash
rustup install nightly
rustup default nightly
```

## Verify Installation

```bash
rustc --version  # Should show 1.85.0 or higher
cargo --version  # Should show 1.85.0 or higher
```

## Build Transmutation

### As Library (for Rust projects)
```bash
# Add to your Cargo.toml
[dependencies]
transmutation = "0.1"
```

### As CLI Tool
```bash
# Install globally
cargo install --path . --features cli

# Or build locally
cargo build --release --features cli

# Run
./target/release/transmutation --help
```

## Platform-Specific Notes

### Windows
- Install Visual Studio Build Tools or MinGW
- **No Python required!** (pure Rust)

### macOS
- Install Xcode Command Line Tools: `xcode-select --install`
- **No Python required!** (pure Rust)

### Linux (Ubuntu/Debian)
```bash
# Install build essentials
sudo apt-get update
sudo apt-get install build-essential pkg-config libssl-dev

# Optional: Install Tesseract for OCR
sudo apt-get install tesseract-ocr libtesseract-dev

# Optional: Install FFmpeg for audio/video
sudo apt-get install ffmpeg libavcodec-dev libavformat-dev libavutil-dev
```

**Note**: Unlike Docling, Transmutation does NOT require Python or any ML model downloads!

## Features

### Available Features (Pure Rust)
- `pdf` - PDF parsing (lopdf - pure Rust)
- `office` - Microsoft Office formats (docx-rs, umya-spreadsheet - pure Rust)
- `web` - HTML/XML parsing (scraper, quick-xml - pure Rust)
- `image-ocr` - Image OCR (Tesseract bindings - optional C library)
- `tesseract` - Tesseract OCR engine (optional)
- `ffmpeg` - Video processing (optional C library)
- `audio` - Audio transcription (pure Rust ASR)
- `video` - Video processing
- `archives` - ZIP/TAR/7Z support (pure Rust)
- `cli` - Command-line interface
- `cache` - Redis/SQLite caching
- `full` - All features

### Build with Specific Features
```bash
# Minimal build (no external dependencies)
cargo build --no-default-features

# With CLI only
cargo build --features cli

# Full build (all features)
cargo build --features full
```

## Troubleshooting

### Edition 2024 Errors
**Error**: `feature edition2024 is required`

**Solution**: Update Rust to 1.85+ or use nightly:
```bash
rustup install nightly
rustup override set nightly  # For this project only
```

### Optional: Tesseract/FFmpeg Errors
**Error**: Linker errors for Tesseract or FFmpeg

**Solution**: Install optional C libraries if needed:
```bash
# Ubuntu/Debian
sudo apt-get install libtesseract-dev libavcodec-dev

# macOS
brew install tesseract ffmpeg

# Windows
# Download from tesseract-ocr.github.io and ffmpeg.org
```

**Note**: These are OPTIONAL. The core functionality works without them!

### Linker Errors
**Error**: `could not find native library`

**Solution**: Install missing system libraries:
```bash
# Ubuntu/Debian
sudo apt-get install pkg-config libssl-dev

# macOS
brew install openssl pkg-config
```

## Next Steps

1. ✅ Install Rust 1.85+
2. ✅ Build the project
3. ✅ Run tests: `cargo test`
4. ✅ Try the CLI: `cargo run --features cli -- --help`
5. ✅ Read the [README.md](README.md) for usage examples

## Support

- **Issues**: https://github.com/hivellm/transmutation/issues
- **Discussions**: https://github.com/hivellm/transmutation/discussions
- **Discord**: https://discord.gg/hivellm

