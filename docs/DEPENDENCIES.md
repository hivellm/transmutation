# Dependency Management

## Overview

Transmutation uses a **hybrid approach** to dependencies:

1. **Core functionality** is **100% Pure Rust** (no external dependencies)
2. **Advanced features** can optionally use external tools for specific tasks
3. **Build-time detection** automatically checks for missing dependencies and provides installation instructions

## Dependency Detection

### How It Works

When you compile Transmutation with features that require external tools, the `build.rs` script:

1. ✅ **Checks** if the required tool is available in `PATH`
2. ⚠️ **Warns** if a dependency is missing (but **does NOT fail** the build)
3. 📖 **Provides** platform-specific installation instructions
4. 🚀 **Links** to automated installation scripts

### Example Output

```bash
$ cargo build --features "pdf-to-image"

   Compiling transmutation v0.1.0
warning: 
╔════════════════════════════════════════════════════════════╗
║  ⚠️  Optional External Dependencies Missing             ║
╚════════════════════════════════════════════════════════════╝

Transmutation will compile, but some features won't work:

  ❌ pdftoppm (poppler-utils): PDF → Image conversion
     Install: sudo apt-get install poppler-utils

📖 For detailed installation instructions:
   https://github.com/yourusername/transmutation/blob/main/install/README.md

💡 Quick install (all dependencies):
   ./install/install-deps-linux.sh

    Finished `dev` profile in 8.70s
```

## Why This Approach?

### 1. **Security**

Cargo build scripts **cannot** and **should not** install system-level dependencies automatically. This is a security feature to prevent malicious packages from modifying your system.

### 2. **Flexibility**

Users can:
- Use **pure Rust features** without any external tools
- Install **only the dependencies** they need
- Use **pre-compiled binaries** in production

### 3. **Discoverability**

The `build.rs` script makes it **immediately clear** what's missing and **exactly how** to install it for your platform.

## Cargo.toml Usage

When using Transmutation as a library, specify only the features you need:

### Pure Rust (No Dependencies)

```toml
[dependencies]
transmutation = { version = "0.1", features = ["pdf", "office", "web"] }
```

This will compile **without any warnings** because these features are 100% Rust.

### With Optional Features

```toml
[dependencies]
transmutation = { version = "0.1", features = ["pdf", "pdf-to-image"] }
```

During `cargo build`, you'll see warnings if `pdftoppm` is not installed, along with instructions.

### Feature Matrix

| Feature | External Dependency | Required At |
|---------|---------------------|-------------|
| `pdf` | None | - |
| `office` | None (Markdown only) | - |
| `office` + `pdf-to-image` | LibreOffice | Runtime |
| `pdf-to-image` | poppler-utils | Runtime |
| `tesseract` | Tesseract OCR | Runtime |
| `audio` | FFmpeg | Runtime |
| `video` | FFmpeg | Runtime |
| `web` | None | - |
| `archives` | None | - |
| `docling-ffi` | C++ build tools | Compile-time |

## Runtime Behavior

If a feature is enabled but the dependency is missing at **runtime**, Transmutation will:

1. ✅ **Detect** the missing tool
2. ❌ **Return an error** with installation instructions
3. 💡 **Suggest** pure Rust alternatives (if available)

Example:

```rust
use transmutation::Converter;

#[tokio::main]
async fn main() {
    let converter = Converter::new().unwrap();
    
    let result = converter
        .convert("document.pdf")
        .to_images()  // Requires pdftoppm
        .execute()
        .await;
    
    match result {
        Ok(images) => println!("Converted to {} images", images.len()),
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Install poppler-utils: sudo apt-get install poppler-utils");
        }
    }
}
```

## Installation Scripts

For convenience, we provide **automated installation scripts** for all platforms:

### Linux (Debian/Ubuntu)

```bash
./install/install-deps-linux.sh
```

Installs:
- build-essential (gcc, cmake, git)
- poppler-utils
- libreoffice
- tesseract-ocr
- ffmpeg

### macOS (Homebrew)

```bash
./install/install-deps-macos.sh
```

Installs:
- Xcode Command Line Tools
- poppler
- libreoffice
- tesseract
- ffmpeg

### Windows

**Option 1: Chocolatey (Recommended)**
```powershell
.\install\install-deps-windows.ps1
```

**Option 2: winget (Windows 10/11)**
```batch
.\install\install-deps-windows.bat
```

**Option 3: Manual Download**
```batch
.\install\install-deps-windows-manual.bat
```

Installs:
- Visual Studio Build Tools
- CMake & Git
- Poppler
- LibreOffice
- Tesseract OCR
- FFmpeg

See [`install/README.md`](../install/README.md) for detailed instructions.

## Docker Alternative

Don't want to install dependencies on your host system? Use Docker:

```bash
# Build Docker image with all dependencies
docker build -t transmutation .

# Run conversion
docker run -v $(pwd)/data:/data transmutation convert /data/document.pdf
```

The Docker image includes all dependencies pre-installed.

## FAQ

### Q: Why can't Cargo install these dependencies automatically?

**A:** For security reasons, Cargo build scripts cannot and should not install system-level packages. This prevents malicious crates from compromising your system.

### Q: Can I use Transmutation without ANY external dependencies?

**A:** Yes! Use only the `pdf`, `office`, and `web` features for 100% Pure Rust functionality.

### Q: What happens if I try to use a feature without its dependency?

**A:** You'll get a clear runtime error with installation instructions. The feature will not work until the dependency is installed.

### Q: Can I check if a dependency is available before using it?

**A:** Yes! Use the feature-check API:

```rust
if transmutation::features::has_pdftoppm() {
    // Use PDF → Image conversion
} else {
    // Use pure Rust fallback
}
```

### Q: Do I need to install dependencies in production?

**A:** Only for the features you use. If you only use `pdf` → Markdown conversion, no external dependencies are needed.

---

**Last Updated**: October 13, 2025  
**See also**: [`install/README.md`](../install/README.md), [`README.md`](../README.md)

