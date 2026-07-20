# Script para build de TODAS as bibliotecas FFI usando Docker (Windows)
# Gera bibliotecas Linux (x86 e ARM) usando Docker

$ErrorActionPreference = "Stop"

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host " Building FFI libraries (All Platforms)" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

# Criar diretório libs se não existir
if (-Not (Test-Path "libs")) {
    New-Item -ItemType Directory -Path "libs" | Out-Null
}

# Build Linux x86_64
Write-Host "[1/2] Building for Linux x86_64..." -ForegroundColor Yellow
docker build -f Dockerfile.build-libs -t transmutation-builder-x86 .

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Docker build failed for x86_64!" -ForegroundColor Red
    exit 1
}

docker run --rm -v "${PWD}/libs:/output" transmutation-builder-x86

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Failed to extract x86_64 libraries!" -ForegroundColor Red
    exit 1
}

# Build Linux ARM64 (se buildx disponível)
Write-Host "`n[2/2] Building for Linux ARM64..." -ForegroundColor Yellow

$buildxAvailable = $false
try {
    docker buildx version | Out-Null
    $buildxAvailable = $true
} catch {
    $buildxAvailable = $false
}

if ($buildxAvailable) {
    docker buildx build --platform linux/arm64 -f Dockerfile.build-libs-arm -t transmutation-builder-arm --load .
    
    if ($LASTEXITCODE -eq 0) {
        docker run --rm -v "${PWD}/libs:/output" transmutation-builder-arm
    } else {
        Write-Host "⚠️  ARM64 build failed, but continuing..." -ForegroundColor Yellow
    }
} else {
    Write-Host "⚠️  Docker buildx not available, skipping ARM64 build" -ForegroundColor Yellow
    Write-Host "   Install with: docker buildx install" -ForegroundColor Yellow
}

Write-Host "`n========================================" -ForegroundColor Green
Write-Host " ✅ Build complete!" -ForegroundColor Green
Write-Host "========================================`n" -ForegroundColor Green

Write-Host "Libraries built:" -ForegroundColor Cyan
Get-ChildItem libs\libdocling-ffi-*.so -ErrorAction SilentlyContinue | ForEach-Object {
    Write-Host "  - $($_.Name) ($([math]::Round($_.Length/1MB, 2)) MB)" -ForegroundColor White
}

Write-Host "`nNext steps:" -ForegroundColor Cyan
Write-Host "  For WSL:" -ForegroundColor Yellow
Write-Host "    wsl -d Ubuntu-24.04 -- bash -c 'cd /mnt/f/Node/hivellm/transmutation && cargo build --release --features pdf,cli,docling-ffi'" -ForegroundColor White
Write-Host "`n  Or use directly in WSL:" -ForegroundColor Yellow
Write-Host "    wsl -d Ubuntu-24.04 -- bash" -ForegroundColor White
Write-Host "    cd /mnt/f/Node/hivellm/transmutation" -ForegroundColor White
Write-Host "    export LD_LIBRARY_PATH=`$PWD/libs:`$LD_LIBRARY_PATH" -ForegroundColor White
Write-Host "    cargo build --release --features 'pdf,cli,docling-ffi'" -ForegroundColor White
Write-Host "    ./target/release/transmutation convert document.pdf --ffi -o output.md" -ForegroundColor White

