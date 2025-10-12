# Transmutation Setup Guide

## Critical: Rust Version Requirement

**Transmutation requires Rust nightly or 1.85+ for Edition 2024 support.**

## Quick Setup

```bash
# Install rustup if not present
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install nightly toolchain
rustup install nightly

# Set nightly as default for this project
cd transmutation
rustup override set nightly

# Verify
rustc --version  # Should show "rustc 1.xx.x-nightly"

# Build
cargo build
```

## Current Blocker

Some dependencies require Edition 2024 which is not available in Rust 1.75.0:
- `pxfm v0.1.25` (transitive dependency)
- `file-format v0.27` (direct dependency - downgraded to 0.26)

**Status**: Waiting for Rust update to continue compilation testing.

## What Works Now

Even without compiling, the following is complete:
- ✅ Project structure
- ✅ Core types (`src/types.rs`)
- ✅ Converter traits (`src/converters/traits.rs`)
- ✅ File detection (`src/utils/file_detect.rs`)
- ✅ All converter stubs (PDF, DOCX, XLSX, PPTX, HTML, XML, Image, Archive)
- ✅ CLI structure
- ✅ Comprehensive documentation

## Next Steps After Rust Update

1. Run `cargo check` to verify compilation
2. Implement PDF text extraction (`src/engines/pdf_parser.rs`)
3. Implement Markdown generator (`src/output/markdown.rs`)
4. Complete PDF converter implementation
5. Add tests and benchmarks

