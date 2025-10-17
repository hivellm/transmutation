# Build docling FFI library for Windows
# Requires: Visual Studio 2019+ with C++ tools, CMake

$ErrorActionPreference = "Stop"

Write-Host "`n=====================================" -ForegroundColor Cyan
Write-Host "Building docling FFI library (Windows)" -ForegroundColor Cyan
Write-Host "=====================================`n" -ForegroundColor Cyan

# Step 1: Patch and build docling-parse without Python bindings
Write-Host "[1/3] Patching docling-parse..." -ForegroundColor Yellow
Set-Location docling-parse

if (-Not (Test-Path "CMakeLists.txt.bak")) {
    Copy-Item "CMakeLists.txt" "CMakeLists.txt.bak"
    Write-Host "Backup created: CMakeLists.txt.bak" -ForegroundColor Green
}

# Read the original CMakeLists.txt
$content = Get-Content "CMakeLists.txt.bak" -Raw

# Remove ZLIB dependency (not needed for FFI-only build)
$content = $content -replace 'find_package\(ZLIB REQUIRED\)', '# find_package(ZLIB REQUIRED) - Skipped for FFI build'
$content = $content -replace 'find_package\(ZLIB\)', '# find_package(ZLIB) - Skipped for FFI build'

# Patch os_opts.cmake to remove ZLIB
if (-Not (Test-Path "cmake\os_opts.cmake.bak")) {
    Copy-Item "cmake\os_opts.cmake" "cmake\os_opts.cmake.bak"
}
$osOptsContent = Get-Content "cmake\os_opts.cmake.bak" -Raw
$osOptsContent = $osOptsContent -replace 'find_package\(ZLIB\)', '# find_package(ZLIB) - Skipped for FFI build'
$osOptsContent = $osOptsContent -replace 'set\(OS_DEPENDENCIES ZLIB::ZLIB\)', 'set(OS_DEPENDENCIES)'
Set-Content "cmake\os_opts.cmake" $osOptsContent

Write-Host "✅ Patched cmake/os_opts.cmake to remove ZLIB" -ForegroundColor Green

# Replace pybind11 section with a stub
$patchContent = @"
# ***************************
# ***  Python-binding     ***
# ***************************

# Skipped for FFI-only build (no pybind11 required)
message(STATUS "Skipping Python bindings (FFI build)")

# *****************
# ***  Install  ***
# *****************

install(TARGETS parse_v1 parse_v2 DESTINATION lib)
"@

# Find the line number where Python binding starts (around line 188)
$lines = $content -split "`n"
$pythonBindingIndex = 0
for ($i = 0; $i -lt $lines.Count; $i++) {
    if ($lines[$i] -match "Python-binding") {
        $pythonBindingIndex = $i - 2  # Start 2 lines before the comment
        break
    }
}

# If not found, use line 186 as default
if ($pythonBindingIndex -eq 0) {
    $pythonBindingIndex = 186
}

# Apply patch: replace from Python binding section to end with our stub
$newContent = $lines[0..($pythonBindingIndex-1)] -join "`n"
$newContent += "`n$patchContent"
Set-Content "CMakeLists.txt" $newContent

Write-Host "`n[2/3] Building docling-parse C++ libs..." -ForegroundColor Yellow

# Detect architecture
$ARCH = if ($env:PROCESSOR_ARCHITECTURE -eq "AMD64") { "x86" } else { "ARM" }
Write-Host "Detected architecture: $ARCH" -ForegroundColor Cyan

$DOCLING_BUILD_DIR = "build_windows_${ARCH}_docling"
if (-Not (Test-Path $DOCLING_BUILD_DIR)) {
    New-Item -ItemType Directory -Path $DOCLING_BUILD_DIR | Out-Null
}

Set-Location $DOCLING_BUILD_DIR

# Configure CMake for Windows (Visual Studio)
cmake .. -G "Visual Studio 17 2022" -A x64 -DCMAKE_BUILD_TYPE=Release -DCMAKE_POSITION_INDEPENDENT_CODE=ON
if ($LASTEXITCODE -ne 0) {
    Write-Host "CMake configuration failed!" -ForegroundColor Red
    exit 1
}

# Build
cmake --build . --config Release --parallel
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

Set-Location ..\..

# Step 2: Build docling FFI wrapper
Write-Host "`n[3/3] Building docling FFI wrapper..." -ForegroundColor Yellow

$BUILD_DIR = "..\build_windows_$ARCH"
if (-Not (Test-Path $BUILD_DIR)) {
    New-Item -ItemType Directory -Path $BUILD_DIR | Out-Null
}

Set-Location $BUILD_DIR

cmake ..\cpp -G "Visual Studio 17 2022" -A x64 -DCMAKE_BUILD_TYPE=Release -DCMAKE_POSITION_INDEPENDENT_CODE=ON
if ($LASTEXITCODE -ne 0) {
    Write-Host "CMake configuration failed!" -ForegroundColor Red
    exit 1
}

cmake --build . --config Release --parallel
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

Set-Location ..\..

Write-Host "`n=====================================" -ForegroundColor Green
Write-Host "Build complete!" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Green

# Copy libraries to libs folder
if (-Not (Test-Path "libs")) {
    New-Item -ItemType Directory -Path "libs" | Out-Null
}

$dllPath = "build_windows_$ARCH\Release\docling_ffi.dll"
$libPath = "build_windows_$ARCH\Release\docling_ffi.lib"

if (Test-Path $dllPath) {
    Copy-Item $dllPath "libs\docling-ffi-windows_$ARCH.dll" -Force
    Write-Host "✅ DLL copied to libs/docling-ffi-windows_$ARCH.dll" -ForegroundColor Green
}

if (Test-Path $libPath) {
    Copy-Item $libPath "libs\docling-ffi-windows_$ARCH.lib" -Force
    Write-Host "✅ LIB copied to libs/docling-ffi-windows_$ARCH.lib" -ForegroundColor Green
}

if (Test-Path $dllPath) {
    Write-Host "`nLibrary location:" -ForegroundColor Cyan
    Write-Host "  DLL: libs\docling-ffi-windows_$ARCH.dll" -ForegroundColor White
    Write-Host "  LIB: libs\docling-ffi-windows_$ARCH.lib" -ForegroundColor White
} else {
    Write-Host "`nWarning: DLL not found at expected location" -ForegroundColor Yellow
    Write-Host "Check: build_windows_$ARCH\" -ForegroundColor Yellow
}

Write-Host "`nNext steps:" -ForegroundColor Cyan
Write-Host "  1. cargo build --release --features 'pdf,cli,docling-ffi'" -ForegroundColor White
Write-Host "  2. Copy docling_ffi.dll to target\release\" -ForegroundColor White
Write-Host "  3. .\target\release\transmutation.exe convert document.pdf --precision --ffi -o output.md" -ForegroundColor White
