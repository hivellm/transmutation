# Embedded dependency installer for Transmutation MSI
# This script is included in the MSI and can be run after installation

param(
    [switch]$Silent = $false
)

$ErrorActionPreference = "Stop"

if (-not $Silent) {
    Write-Host "╔════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║  📦 Transmutation Dependencies        ║" -ForegroundColor Cyan
    Write-Host "║     Optional External Tools           ║" -ForegroundColor Cyan
    Write-Host "╚════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""
}

# Check if running as Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "⚠️  This script should be run as Administrator for system-wide installation." -ForegroundColor Yellow
    Write-Host ""
    $continue = Read-Host "Continue anyway? (y/N)"
    if ($continue -ne "y") {
        exit 0
    }
}

# Detect available package managers
$hasChoco = Get-Command choco -ErrorAction SilentlyContinue
$hasWinget = Get-Command winget -ErrorAction SilentlyContinue

if (-not $hasChoco -and -not $hasWinget) {
    Write-Host "❌ No package manager found (Chocolatey or winget)" -ForegroundColor Red
    Write-Host ""
    Write-Host "Options:" -ForegroundColor Yellow
    Write-Host "1. Install Chocolatey (recommended):" -ForegroundColor White
    Write-Host "   Set-ExecutionPolicy Bypass -Scope Process -Force;" -ForegroundColor Gray
    Write-Host "   iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))" -ForegroundColor Gray
    Write-Host ""
    Write-Host "2. Use winget (Windows 10/11):" -ForegroundColor White
    Write-Host "   Install 'App Installer' from Microsoft Store" -ForegroundColor Gray
    Write-Host ""
    Write-Host "3. Manual installation:" -ForegroundColor White
    Write-Host "   See: https://github.com/hivellm/transmutation/blob/main/install/README.md" -ForegroundColor Gray
    Write-Host ""
    exit 1
}

$method = if ($hasChoco) { "Chocolatey" } else { "winget" }

if (-not $Silent) {
    Write-Host "📥 Using $method to install dependencies..." -ForegroundColor Green
    Write-Host ""
    Write-Host "Dependencies to install:" -ForegroundColor Cyan
    Write-Host "  • Poppler (PDF → Image conversion)" -ForegroundColor White
    Write-Host "  • LibreOffice (DOCX → Image conversion)" -ForegroundColor White
    Write-Host "  • Tesseract OCR (Image → Text)" -ForegroundColor White
    Write-Host "  • FFmpeg (Audio/Video processing)" -ForegroundColor White
    Write-Host ""
    
    $confirm = Read-Host "Proceed with installation? (Y/n)"
    if ($confirm -eq "n") {
        Write-Host "Installation cancelled." -ForegroundColor Yellow
        exit 0
    }
}

try {
    if ($hasChoco) {
        Write-Host "[1/4] Installing Poppler..." -ForegroundColor Yellow
        choco install poppler -y --no-progress 2>&1 | Out-Null
        
        Write-Host "[2/4] Installing LibreOffice..." -ForegroundColor Yellow
        choco install libreoffice -y --no-progress 2>&1 | Out-Null
        
        Write-Host "[3/4] Installing Tesseract..." -ForegroundColor Yellow
        choco install tesseract -y --no-progress 2>&1 | Out-Null
        
        Write-Host "[4/4] Installing FFmpeg..." -ForegroundColor Yellow
        choco install ffmpeg -y --no-progress 2>&1 | Out-Null
        
    } elseif ($hasWinget) {
        Write-Host "[1/3] Installing LibreOffice..." -ForegroundColor Yellow
        winget install --id TheDocumentFoundation.LibreOffice --silent --accept-package-agreements --accept-source-agreements 2>&1 | Out-Null
        
        Write-Host "[2/3] Installing Tesseract..." -ForegroundColor Yellow
        winget install --id UB-Mannheim.TesseractOCR --silent --accept-package-agreements --accept-source-agreements 2>&1 | Out-Null
        
        Write-Host "[3/3] Installing FFmpeg..." -ForegroundColor Yellow
        winget install --id Gyan.FFmpeg --silent --accept-package-agreements --accept-source-agreements 2>&1 | Out-Null
        
        Write-Host ""
        Write-Host "⚠️  Poppler: winget doesn't have poppler package." -ForegroundColor Yellow
        Write-Host "   Download from: https://github.com/oschwartz10612/poppler-windows/releases" -ForegroundColor Gray
    }
    
    Write-Host ""
    Write-Host "✅ Dependencies installed successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "⚠️  Restart your terminal/PowerShell to use the new tools" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "🧪 Test installation:" -ForegroundColor Cyan
    Write-Host "   transmutation --version" -ForegroundColor White
    Write-Host "   pdftoppm -v" -ForegroundColor White
    Write-Host "   tesseract --version" -ForegroundColor White
    Write-Host "   ffmpeg -version" -ForegroundColor White
    Write-Host ""
    
} catch {
    Write-Host ""
    Write-Host "❌ Installation failed: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "Try manual installation:" -ForegroundColor Yellow
    Write-Host "  https://github.com/hivellm/transmutation/blob/main/install/README.md" -ForegroundColor Gray
    exit 1
}

