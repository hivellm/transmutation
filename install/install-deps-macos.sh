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
echo "[1/6] Checking Xcode Command Line Tools..."
xcode-select -p &> /dev/null || xcode-select --install

# PDF & Image conversion
echo "[2/6] Installing poppler (PDF → Image)..."
brew install poppler

# Office conversion
echo "[3/6] Installing LibreOffice (Office formats)..."
brew install --cask libreoffice

# OCR support
echo "[4/6] Installing Tesseract (OCR for images)..."
brew install tesseract tesseract-lang

# Audio/Video processing
echo "[5/6] Installing FFmpeg (Video → Audio extraction)..."
brew install ffmpeg

# Audio/Video transcription
echo "[6/6] Installing Whisper (Audio/Video → Text transcription)..."
brew install pipx
pipx install openai-whisper
pipx ensurepath

echo ""
echo "✅ All dependencies installed!"
echo ""
echo "📊 Installed tools:"
echo "  - Xcode tools: $(xcode-select -p)"
echo "  - pdftoppm: $(pdftoppm -v 2>&1 | head -1)"
echo "  - LibreOffice: /Applications/LibreOffice.app"
echo "  - Tesseract: $(tesseract --version | head -1)"
echo "  - FFmpeg: $(ffmpeg -version | head -1)"
echo "  - Whisper: $(whisper --help 2>&1 | head -1 || echo 'installed (check PATH)')"
echo ""
echo "⚠️  IMPORTANT: Ensure Whisper is in PATH:"
echo "   pipx ensurepath"
echo "   source ~/.zshrc  # or ~/.bash_profile"
echo ""
echo "🚀 You can now run:"
echo "   transmutation convert document.pdf --format png"
echo "   transmutation convert document.docx -o output.md"
echo "   transmutation convert image.jpg -o ocr.md        # OCR"
echo "   transmutation convert audio.mp3 -o transcript.md # Whisper"
echo "   transmutation convert video.mp4 -o transcript.md # FFmpeg + Whisper"
echo ""

