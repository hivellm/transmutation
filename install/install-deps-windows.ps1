# Install Transmutation dependencies on Windows
# Requires: Chocolatey package manager

Write-Host "╔════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  📦 Transmutation Dependencies        ║" -ForegroundColor Cyan
Write-Host "║     Windows (Chocolatey)              ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Check if running as Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "❌ This script must be run as Administrator!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Right-click PowerShell and select 'Run as Administrator'" -ForegroundColor Yellow
    exit 1
}

# Check if Chocolatey is installed
if (-not (Get-Command choco -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Chocolatey not found!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Install Chocolatey first (run as Administrator):" -ForegroundColor Yellow
    Write-Host "  Set-ExecutionPolicy Bypass -Scope Process -Force;" -ForegroundColor Gray
    Write-Host "  [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072;" -ForegroundColor Gray
    Write-Host "  iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))" -ForegroundColor Gray
    Write-Host ""
    exit 1
}

Write-Host "📥 Installing ALL dependencies for ALL features via Chocolatey..." -ForegroundColor Green
Write-Host ""

# Core build tools
Write-Host "[1/7] Installing Visual Studio Build Tools..." -ForegroundColor Yellow
choco install visualstudio2022buildtools -y
choco install visualstudio2022-workload-vctools -y

# CMake and Git
Write-Host "[2/7] Installing CMake and Git..." -ForegroundColor Yellow
choco install cmake git -y

# PDF & Image conversion
Write-Host "[3/7] Installing poppler (PDF → Image)..." -ForegroundColor Yellow
choco install poppler -y

# Office conversion
Write-Host "[4/7] Installing LibreOffice (Office formats)..." -ForegroundColor Yellow
choco install libreoffice -y

# OCR support
Write-Host "[5/7] Installing Tesseract (OCR for images)..." -ForegroundColor Yellow
choco install tesseract -y

# Audio/Video processing
Write-Host "[6/7] Installing FFmpeg (Video → Audio extraction)..." -ForegroundColor Yellow
choco install ffmpeg -y

# Audio/Video transcription
Write-Host "[7/7] Installing Python + Whisper (Audio/Video → Text)..." -ForegroundColor Yellow
choco install python3 -y
pip install --upgrade pip
pip install openai-whisper

Write-Host ""
Write-Host "✅ All dependencies installed!" -ForegroundColor Green
Write-Host ""
Write-Host "📊 Installed tools:" -ForegroundColor Cyan
Write-Host "  - Visual Studio Build Tools"
Write-Host "  - CMake & Git"
Write-Host "  - pdftoppm.exe (poppler)"
Write-Host "  - soffice.exe (LibreOffice)"
Write-Host "  - tesseract.exe (OCR)"
Write-Host "  - ffmpeg.exe (Audio/Video)"
Write-Host "  - whisper.exe (Transcription)"
Write-Host ""
Write-Host "⚠️  IMPORTANT: Restart your PowerShell terminal!" -ForegroundColor Yellow
Write-Host "   New PATH entries need to be loaded."
Write-Host ""
Write-Host "🚀 You can now run:" -ForegroundColor Green
Write-Host "   transmutation convert document.pdf --format png"
Write-Host "   transmutation convert document.docx -o output.md"
Write-Host "   transmutation convert image.jpg -o ocr.md        # OCR"
Write-Host "   transmutation convert audio.mp3 -o transcript.md # Whisper"
Write-Host "   transmutation convert video.mp4 -o transcript.md # FFmpeg + Whisper"
Write-Host ""

