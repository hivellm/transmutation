#!/bin/bash
# Install Transmutation dependencies on Linux (Debian/Ubuntu)

set -e

echo "╔════════════════════════════════════════╗"
echo "║  📦 Transmutation Dependencies        ║"
echo "║     Linux (Debian/Ubuntu)             ║"
echo "╚════════════════════════════════════════╝"
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    SUDO="sudo"
else
    SUDO=""
fi

echo "📥 Installing ALL dependencies for ALL features..."
echo ""

# Update package list
echo "[1/6] Updating package list..."
$SUDO apt-get update -qq

# Core build tools
echo "[2/6] Installing build essentials..."
$SUDO apt-get install -y build-essential cmake git pkg-config

# PDF & Image conversion
echo "[3/6] Installing poppler-utils (PDF → Image)..."
$SUDO apt-get install -y poppler-utils

# Office conversion (DOCX/PPTX/XLSX)
echo "[4/6] Installing LibreOffice (Office formats)..."
$SUDO apt-get install -y libreoffice

# OCR support
echo "[5/6] Installing Tesseract (OCR for images)..."
$SUDO apt-get install -y tesseract-ocr tesseract-ocr-eng tesseract-ocr-por libleptonica-dev libtesseract-dev

# Audio/Video processing
echo "[6/6] Installing FFmpeg (Audio/Video transcription)..."
$SUDO apt-get install -y ffmpeg

echo ""
echo "✅ All dependencies installed!"
echo ""
echo "📊 Installed tools:"
echo "  - Build tools: gcc, cmake, git"
echo "  - pdftoppm: $(pdftoppm -v 2>&1 | head -1)"
echo "  - LibreOffice: $(libreoffice --version | head -1)"
echo "  - Tesseract: $(tesseract --version | head -1)"
echo "  - FFmpeg: $(ffmpeg -version | head -1)"
echo ""
echo "🚀 You can now run:"
echo "   transmutation convert document.pdf --format png"
echo "   transmutation convert document.docx -o output.md"
echo ""

