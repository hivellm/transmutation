# Script para build das bibliotecas FFI usando Docker (Windows)
# Gera as bibliotecas Linux usando WSL/Docker e copia para ./libs/

$ErrorActionPreference = "Stop"

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host " Building FFI libraries with Docker    " -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

# Criar diretório libs se não existir
if (-Not (Test-Path "libs")) {
    New-Item -ItemType Directory -Path "libs" | Out-Null
}

# Build da imagem Docker
Write-Host "[1/2] Building Docker image..." -ForegroundColor Yellow
docker build -f Dockerfile.build-libs -t transmutation-builder .

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Docker build failed!" -ForegroundColor Red
    exit 1
}

# Executar container e copiar libs
Write-Host "`n[2/2] Extracting libraries..." -ForegroundColor Yellow
docker run --rm -v "${PWD}/libs:/output" transmutation-builder

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Failed to extract libraries!" -ForegroundColor Red
    exit 1
}

Write-Host "`n========================================" -ForegroundColor Green
Write-Host " ✅ Build complete!                     " -ForegroundColor Green
Write-Host "========================================`n" -ForegroundColor Green

Write-Host "Libraries available:" -ForegroundColor Cyan
Get-ChildItem libs\libdocling-ffi-*.so -ErrorAction SilentlyContinue | ForEach-Object {
    Write-Host "  - $($_.Name) ($([math]::Round($_.Length/1MB, 2)) MB)" -ForegroundColor White
}

Write-Host "`nNext steps:" -ForegroundColor Cyan
Write-Host "  For Windows (via WSL):" -ForegroundColor Yellow
Write-Host "    wsl -d Ubuntu-24.04 -- bash -c 'cd /mnt/f/Node/hivellm/transmutation && cargo build --release --features pdf,cli,docling-ffi'" -ForegroundColor White
Write-Host "`n  Or use directly in WSL:" -ForegroundColor Yellow
Write-Host "    wsl -d Ubuntu-24.04 -- bash" -ForegroundColor White
Write-Host "    cd /mnt/f/Node/hivellm/transmutation" -ForegroundColor White
Write-Host "    export LD_LIBRARY_PATH=`$PWD/libs:`$LD_LIBRARY_PATH" -ForegroundColor White
Write-Host "    cargo build --release --features 'pdf,cli,docling-ffi'" -ForegroundColor White
Write-Host "    ./target/release/transmutation convert document.pdf --ffi -o output.md" -ForegroundColor White

