# Build MSI installer for Transmutation
# Requires: WiX Toolset 3.11+ or cargo-wix

param(
    [string]$Method = "cargo-wix",  # Options: "cargo-wix" or "wix"
    [switch]$IncludeDepsInstaller = $false  # Include dependency installer
)

$ErrorActionPreference = "Stop"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Transmutation MSI Builder" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check Rust is installed
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "ERROR: Cargo not found! Install Rust first." -ForegroundColor Red
    exit 1
}

if ($Method -eq "cargo-wix") {
    Write-Host "Method: cargo-wix (Recommended)" -ForegroundColor Green
    Write-Host ""
    
    # Check if WiX Toolset is installed (required by cargo-wix)
    $wixInPath = Get-Command candle.exe -ErrorAction SilentlyContinue
    
    # Try to find WiX in common locations
    $wixPaths = @(
        "C:\Program Files (x86)\WiX Toolset v3.14\bin\candle.exe",
        "C:\Program Files (x86)\WiX Toolset v3.11\bin\candle.exe",
        "C:\Program Files\WiX Toolset v3.14\bin\candle.exe",
        "C:\Program Files\WiX Toolset v3.11\bin\candle.exe"
    )
    
    $wixFound = $false
    foreach ($path in $wixPaths) {
        if (Test-Path $path) {
            $wixFound = $true
            $wixDir = Split-Path -Parent $path
            Write-Host "Found WiX at: $wixDir" -ForegroundColor Green
            # Add to PATH for this session
            $env:PATH = "$wixDir;$env:PATH"
            break
        }
    }
    
    if (-not $wixFound -and -not $wixInPath) {
        Write-Host "WARNING: WiX Toolset not found (required by cargo-wix)" -ForegroundColor Yellow
        Write-Host ""
        
        # Check if Chocolatey is available
        if (Get-Command choco -ErrorAction SilentlyContinue) {
            Write-Host "Installing WiX Toolset via Chocolatey..." -ForegroundColor Cyan
            Write-Host "   This will take a few minutes..." -ForegroundColor Gray
            choco install wixtoolset -y
            
            if ($LASTEXITCODE -eq 0) {
                Write-Host "OK WiX Toolset installed!" -ForegroundColor Green
                Write-Host ""
                Write-Host "WARNING: Please restart PowerShell and run this script again" -ForegroundColor Yellow
                exit 0
            } else {
                Write-Host "ERROR: WiX Toolset installation failed" -ForegroundColor Red
            }
        } else {
            Write-Host "ERROR: WiX Toolset is required but not installed" -ForegroundColor Red
        }
        
        Write-Host ""
        Write-Host "Installation options:" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "Option 1: Chocolatey (recommended)" -ForegroundColor White
        Write-Host "  choco install wixtoolset" -ForegroundColor Gray
        Write-Host ""
        Write-Host "Option 2: Manual download" -ForegroundColor White
        Write-Host "  https://wixtoolset.org/releases/" -ForegroundColor Gray
        Write-Host "  Download: wix311.exe" -ForegroundColor Gray
        Write-Host ""
        Write-Host "Option 3: Use pre-built binaries (skip MSI build)" -ForegroundColor White
        Write-Host "  cargo build --release --features cli" -ForegroundColor Gray
        Write-Host ""
        exit 1
    }
    
    # Check if cargo-wix is installed
    if (-not (cargo wix --version 2>$null)) {
        Write-Host "Installing cargo-wix..." -ForegroundColor Yellow
        cargo install cargo-wix
    }
    
    # Copy dependency installer if requested
    if ($IncludeDepsInstaller) {
        Write-Host "Including dependency installer..." -ForegroundColor Cyan
        Copy-Item "wix\install-deps-embedded.ps1" "target\release\install-deps.ps1" -Force
    }
    
    # Build release binary first
    Write-Host "[1/3] Building release binary..." -ForegroundColor Yellow
    cargo build --release --bin transmutation --features "cli,pdf,office,web,pdf-to-image"
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "ERROR: Build failed!" -ForegroundColor Red
        exit 1
    }
    
    # Initialize WiX files (only needed once)
    if (-not (Test-Path "wix\main.wxs")) {
        Write-Host "[2/3] Initializing WiX configuration..." -ForegroundColor Yellow
        cargo wix init
    }
    
    # Build MSI
    Write-Host "[3/3] Building MSI installer..." -ForegroundColor Yellow
    cargo wix --nocapture
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "OK MSI installer created successfully!" -ForegroundColor Green
        Write-Host ""
        Write-Host "Installer location:" -ForegroundColor Cyan
        Get-ChildItem -Path "target\wix\*.msi" | ForEach-Object {
            Write-Host "   $($_.FullName)" -ForegroundColor White
            Write-Host "   Size: $([math]::Round($_.Length / 1MB, 2)) MB" -ForegroundColor Gray
        }
    } else {
        Write-Host "ERROR: MSI build failed!" -ForegroundColor Red
        exit 1
    }
    
} elseif ($Method -eq "wix") {
    Write-Host "Method: WiX Toolset (Manual)" -ForegroundColor Green
    Write-Host ""
    
    # Check if WiX is installed - try multiple versions
    $wixPath = $null
    $wixPaths = @(
        "C:\Program Files (x86)\WiX Toolset v3.14\bin",
        "C:\Program Files (x86)\WiX Toolset v3.11\bin",
        "C:\Program Files\WiX Toolset v3.14\bin",
        "C:\Program Files\WiX Toolset v3.11\bin"
    )
    
    foreach ($path in $wixPaths) {
        if (Test-Path "$path\candle.exe") {
            $wixPath = $path
            Write-Host "Found WiX at: $wixPath" -ForegroundColor Green
            break
        }
    }
    
    if (-not $wixPath) {
        Write-Host "ERROR: WiX Toolset not found!" -ForegroundColor Red
        Write-Host ""
        Write-Host "Install from: https://wixtoolset.org/releases/" -ForegroundColor Yellow
        Write-Host "Or use cargo-wix: .\build-msi.ps1 -Method cargo-wix" -ForegroundColor Yellow
        exit 1
    }
    
    # Build release binary
    Write-Host "[1/4] Building release binary..." -ForegroundColor Yellow
    cargo build --release --bin transmutation --features "cli,pdf,office,web,pdf-to-image"
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "ERROR: Build failed!" -ForegroundColor Red
        exit 1
    }
    
    # Get version from Cargo.toml
    $version = (Select-String -Path "Cargo.toml" -Pattern 'version\s*=\s*"(.+)"' | Select-Object -First 1).Matches.Groups[1].Value
    
    # Compile WiX source
    Write-Host "[2/4] Compiling WiX source..." -ForegroundColor Yellow
    & "$wixPath\candle.exe" -nologo `
        -dVersion="$version" `
        -dCargoTargetBinDir="$(Get-Location)\target\release" `
        -dSourceDir="$(Get-Location)" `
        -out "target\wix\" `
        "wix\main.wxs"
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "ERROR: WiX compilation failed!" -ForegroundColor Red
        exit 1
    }
    
    # Link MSI
    Write-Host "[3/4] Linking MSI..." -ForegroundColor Yellow
    & "$wixPath\light.exe" -nologo `
        -ext WixUIExtension `
        -out "target\wix\transmutation-$version-x86_64.msi" `
        "target\wix\main.wixobj"
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "OK MSI installer created successfully!" -ForegroundColor Green
        Write-Host ""
        Write-Host "Installer location:" -ForegroundColor Cyan
        $msi = Get-Item "target\wix\transmutation-$version-x86_64.msi"
        Write-Host "   $($msi.FullName)" -ForegroundColor White
        Write-Host "   Size: $([math]::Round($msi.Length / 1MB, 2)) MB" -ForegroundColor Gray
    } else {
        Write-Host "ERROR: MSI linking failed!" -ForegroundColor Red
        exit 1
    }
    
} else {
    Write-Host "ERROR: Invalid method: $Method" -ForegroundColor Red
    Write-Host ""
    Write-Host "Usage: .\build-msi.ps1 [-Method cargo-wix|wix]" -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "Installation commands:" -ForegroundColor Green
Write-Host "   # Interactive:" -ForegroundColor Cyan
Write-Host "   msiexec /i target\wix\transmutation-*.msi" -ForegroundColor White
Write-Host ""
Write-Host "   # Silent:" -ForegroundColor Cyan
Write-Host "   msiexec /i target\wix\transmutation-*.msi /qn" -ForegroundColor White
Write-Host ""

if ($IncludeDepsInstaller) {
    Write-Host "Dependency installer included:" -ForegroundColor Green
    Write-Host "   After installation, run:" -ForegroundColor Cyan
    Write-Host '   cd "C:\Program Files\Transmutation"' -ForegroundColor White
    Write-Host '   .\install-deps.ps1' -ForegroundColor White
    Write-Host ""
    Write-Host "   Or use Start Menu shortcut: Install Dependencies" -ForegroundColor Gray
    Write-Host ""
}
