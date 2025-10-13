#!/bin/bash
# Install Transmutation dependencies on macOS

set -e

echo "╔════════════════════════════════════════╗"
echo "║  📦 Transmutation Dependencies        ║"
echo "║     macOS (Homebrew)                  ║"
echo "╚════════════════════════════════════════╝"
echo ""

# Check if Homebrew is installed
if ! command -v brew &> /dev/null; then
    echo "❌ Homebrew not found!"
    echo ""
    echo "Install Homebrew first:"
    echo "  /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
    echo ""
    exit 1
fi

echo "📥 Installing ALL dependencies for ALL features via Homebrew..."
echo ""

# Core build tools (usually pre-installed on macOS with Xcode Command Line Tools)
echo "[1/5] Checking Xcode Command Line Tools..."
xcode-select -p &> /dev/null || xcode-select --install

# PDF & Image conversion
echo "[2/5] Installing poppler (PDF → Image)..."
brew install poppler

# Office conversion
echo "[3/5] Installing LibreOffice (Office formats)..."
brew install --cask libreoffice

# OCR support
echo "[4/5] Installing Tesseract (OCR for images)..."
brew install tesseract tesseract-lang

# Audio/Video processing
echo "[5/5] Installing FFmpeg (Audio/Video transcription)..."
brew install ffmpeg

echo ""
echo "✅ All dependencies installed!"
echo ""
echo "📊 Installed tools:"
echo "  - Xcode tools: $(xcode-select -p)"
echo "  - pdftoppm: $(pdftoppm -v 2>&1 | head -1)"
echo "  - LibreOffice: /Applications/LibreOffice.app"
echo "  - Tesseract: $(tesseract --version | head -1)"
echo "  - FFmpeg: $(ffmpeg -version | head -1)"
echo ""
echo "🚀 You can now run:"
echo "   transmutation convert document.pdf --format png"
echo "   transmutation convert document.docx -o output.md"
echo ""

