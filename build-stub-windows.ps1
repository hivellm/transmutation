# Build stub FFI library for Windows (sem docling-parse)
# Gera DLL que compila mas retorna erros (use WSL para funcionalidade real)

$ErrorActionPreference = "Stop"

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host " Building Windows Stub Library          " -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

# Detectar arquitetura
$ARCH = if ($env:PROCESSOR_ARCHITECTURE -eq "AMD64") { "x86" } else { "ARM" }
Write-Host "Detected architecture: $ARCH" -ForegroundColor Cyan

# Check for Visual Studio
$vsPath = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC"
if (-Not (Test-Path $vsPath)) {
    $vsPath = "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC"
}

if (-Not (Test-Path $vsPath)) {
    Write-Host "❌ Visual Studio 2022 not found!" -ForegroundColor Red
    Write-Host "Please install Visual Studio 2022 with C++ tools" -ForegroundColor Yellow
    exit 1
}

Write-Host "✅ Visual Studio found!" -ForegroundColor Green

# Create minimal CMakeLists.txt for stub only
Write-Host "`n[1/2] Creating stub CMakeLists.txt..." -ForegroundColor Yellow

$stubCMake = @"
cmake_minimum_required(VERSION 3.12)
project(docling_ffi_stub LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# Stub FFI library (no docling-parse dependencies)
add_library(docling_ffi SHARED 
    docling_ffi_stub.cpp
)

target_include_directories(docling_ffi PUBLIC
    `${CMAKE_CURRENT_SOURCE_DIR}
)

install(TARGETS docling_ffi 
    LIBRARY DESTINATION lib
    RUNTIME DESTINATION bin
    ARCHIVE DESTINATION lib
)

install(FILES docling_ffi.h DESTINATION include)
"@

$BUILD_DIR = "build_windows_$ARCH"

# Create build directory
if (Test-Path $BUILD_DIR) {
    Remove-Item -Recurse -Force $BUILD_DIR
}
New-Item -ItemType Directory -Path $BUILD_DIR | Out-Null

# Write CMakeLists.txt
Set-Content "$BUILD_DIR\CMakeLists.txt" $stubCMake

# Copy source files to build directory
Copy-Item "cpp\docling_ffi.h" "$BUILD_DIR\"
Copy-Item "cpp\docling_ffi_stub.cpp" "$BUILD_DIR\"

Write-Host "`n[2/2] Building stub library..." -ForegroundColor Yellow

Set-Location $BUILD_DIR

cmake . -G "Visual Studio 17 2022" -A x64 -DCMAKE_BUILD_TYPE=Release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ CMake failed!" -ForegroundColor Red
    Set-Location ..
    exit 1
}

cmake --build . --config Release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    Set-Location ..
    exit 1
}

Set-Location ..

# Copy to libs
Write-Host "`n[3/3] Copying to libs/..." -ForegroundColor Yellow

if (-Not (Test-Path "libs")) {
    New-Item -ItemType Directory -Path "libs" | Out-Null
}

$dllSrc = "$BUILD_DIR\Release\docling_ffi.dll"
$libSrc = "$BUILD_DIR\Release\docling_ffi.lib"

if (Test-Path $dllSrc) {
    Copy-Item $dllSrc "libs\docling-ffi-windows_$ARCH.dll" -Force
    Write-Host "✅ DLL: libs/docling-ffi-windows_$ARCH.dll" -ForegroundColor Green
}

if (Test-Path $libSrc) {
    Copy-Item $libSrc "libs\docling-ffi-windows_$ARCH.lib" -Force
    Write-Host "✅ LIB: libs/docling-ffi-windows_$ARCH.lib" -ForegroundColor Green
}

Write-Host "`n========================================" -ForegroundColor Green
Write-Host " ✅ Windows stub library built!         " -ForegroundColor Green
Write-Host "========================================`n" -ForegroundColor Green

Write-Host "⚠️  IMPORTANT:" -ForegroundColor Yellow
Write-Host "This is a STUB library - it compiles but doesn't parse PDFs." -ForegroundColor Yellow
Write-Host "For actual PDF parsing, use WSL:" -ForegroundColor Yellow
Write-Host "  wsl -d Ubuntu-24.04 -- bash" -ForegroundColor White
Write-Host "  cd /mnt/f/Node/hivellm/transmutation" -ForegroundColor White
Write-Host "  export LD_LIBRARY_PATH=`$PWD/libs:`$LD_LIBRARY_PATH" -ForegroundColor White
Write-Host "  cargo build --release --features 'pdf,cli,docling-ffi'" -ForegroundColor White
Write-Host "  ./target/release/transmutation convert document.pdf --ffi -o output.md" -ForegroundColor White

Write-Host "`nFiles created:" -ForegroundColor Cyan
$dllFile = "libs/docling-ffi-windows_$ARCH.dll"
$libFile = "libs/docling-ffi-windows_$ARCH.lib"
Write-Host "  - $dllFile (stub)" -ForegroundColor White
Write-Host "  - $libFile (stub)" -ForegroundColor White

