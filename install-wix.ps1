# Quick WiX Toolset installer

$ErrorActionPreference = "Stop"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  WiX Toolset Quick Installer" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check if already installed
if (Test-Path "C:\Program Files (x86)\WiX Toolset v3.11\bin\candle.exe") {
    Write-Host "OK WiX Toolset is already installed!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Location: C:\Program Files (x86)\WiX Toolset v3.11\bin" -ForegroundColor Gray
    exit 0
}

if (Get-Command candle.exe -ErrorAction SilentlyContinue) {
    Write-Host "OK WiX Toolset found in PATH!" -ForegroundColor Green
    Write-Host ""
    $candlePath = (Get-Command candle.exe).Source
    Write-Host "Location: $candlePath" -ForegroundColor Gray
    exit 0
}

Write-Host "WiX Toolset not found. Installing..." -ForegroundColor Yellow
Write-Host ""

# Check for Chocolatey
if (Get-Command choco -ErrorAction SilentlyContinue) {
    Write-Host "Using Chocolatey..." -ForegroundColor Cyan
    Write-Host "This will take 2-3 minutes..." -ForegroundColor Gray
    Write-Host ""
    
    choco install wixtoolset -y
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "OK WiX Toolset installed successfully!" -ForegroundColor Green
        Write-Host ""
        Write-Host "Location: C:\Program Files (x86)\WiX Toolset v3.11\bin" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "IMPORTANT: Restart PowerShell to update PATH" -ForegroundColor Yellow
        Write-Host ""
        Write-Host "Then run:" -ForegroundColor Cyan
        Write-Host "  .\build-msi.ps1" -ForegroundColor White
        Write-Host ""
    } else {
        Write-Host ""
        Write-Host "ERROR Installation failed!" -ForegroundColor Red
        Write-Host ""
        Write-Host "Try manual installation:" -ForegroundColor Yellow
        Write-Host "  https://wixtoolset.org/releases/" -ForegroundColor Gray
        exit 1
    }
} else {
    Write-Host "ERROR Chocolatey not found!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Option 1: Install Chocolatey first" -ForegroundColor Yellow
    Write-Host "  Run as Administrator:" -ForegroundColor Cyan
    Write-Host '  Set-ExecutionPolicy Bypass -Scope Process -Force;' -ForegroundColor Gray
    $installCmd = 'iex ((New-Object System.Net.WebClient).DownloadString("https://community.chocolatey.org/install.ps1"))'
    Write-Host "  $installCmd" -ForegroundColor Gray
    Write-Host ""
    Write-Host "  Then run this script again:" -ForegroundColor Cyan
    Write-Host '  .\install-wix.ps1' -ForegroundColor White
    Write-Host ""
    Write-Host "Option 2: Manual WiX download" -ForegroundColor Yellow
    Write-Host "  1. Download from: https://wixtoolset.org/releases/" -ForegroundColor Cyan
    Write-Host "  2. Run: wix311.exe" -ForegroundColor Cyan
    Write-Host "  3. Restart PowerShell" -ForegroundColor Cyan
    Write-Host ""
    exit 1
}
