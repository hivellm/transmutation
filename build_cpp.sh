#!/bin/bash
# Build docling FFI library for Linux/Mac
set -e

echo -e "\033[1;36mBuilding docling FFI library...\033[0m"

# Check for zlib (required by qpdf)
if ! pkg-config --exists zlib 2>/dev/null && ! ldconfig -p | grep -q libz.so 2>/dev/null; then
    echo -e "\033[1;33m⚠️  Warning: zlib not found!\033[0m"
    echo -e "\033[1;33mPlease install: sudo apt-get install zlib1g-dev (Ubuntu/Debian)\033[0m"
    echo -e "\033[1;33m            or: brew install zlib (macOS)\033[0m"
    echo ""
fi

# Detect OS and architecture
if [ "$(uname)" == "Darwin" ]; then
    OS="macos"
    LIB_EXT="dylib"
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
    OS="linux"
    LIB_EXT="so"
else
    OS="windows"
    LIB_EXT="dll"
fi

MACHINE=$(uname -m)
if [[ "$MACHINE" == "x86_64" || "$MACHINE" == "amd64" ]]; then
    ARCH="x86"
elif [[ "$MACHINE" == "aarch64" || "$MACHINE" == "arm64" ]]; then
    ARCH="ARM"
else
    ARCH="x86"
fi

echo -e "\033[1;32mDetected OS: $OS, Architecture: $ARCH\033[0m"

# Step 1: Patch and build docling-parse without Python bindings
echo -e "\n\033[1;33m[1/3] Patching docling-parse...\033[0m"
cd docling-parse

if [ ! -f "CMakeLists.txt.bak" ]; then
    cp CMakeLists.txt CMakeLists.txt.bak
    echo -e "\033[1;32mBackup created: CMakeLists.txt.bak\033[0m"
fi

# Replace pybind11 section with a stub
cat > CMakeLists.txt.patch << 'EOF'
# ***************************
# ***  Python-binding     ***
# ***************************

# Skipped for FFI-only build (no pybind11 required)
message(STATUS "Skipping Python bindings (FFI build)")

# *****************
# ***  Install  ***
# *****************

install(TARGETS parse_v1 parse_v2 DESTINATION lib)
EOF

# Apply patch: replace from line 188 to end with our stub
head -n 187 CMakeLists.txt.bak > CMakeLists.txt
cat CMakeLists.txt.patch >> CMakeLists.txt
rm CMakeLists.txt.patch

echo -e "\n\033[1;33m[2/3] Building docling-parse C++ libs...\033[0m"
DOCLING_BUILD_DIR="build_${OS}_${ARCH}_docling"
if [ ! -d "$DOCLING_BUILD_DIR" ]; then
    mkdir -p "$DOCLING_BUILD_DIR"
fi

cd "$DOCLING_BUILD_DIR"
cmake .. -DCMAKE_BUILD_TYPE=Release -DCMAKE_POSITION_INDEPENDENT_CODE=ON
cmake --build . --config Release --parallel $(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)
cd ../..

# Step 2: Build docling FFI wrapper (FULL version with docling-parse)
echo -e "\n\033[1;33m[3/3] Building docling FFI wrapper (FULL)...\033[0m"
BUILD_DIR="build_${OS}_${ARCH}"
mkdir -p "$BUILD_DIR"

# Copy full CMakeLists for FFI build
cp cpp/CMakeLists_full.txt cpp/CMakeLists.txt

cd "$BUILD_DIR"
cmake ../cpp -DCMAKE_BUILD_TYPE=Release -DCMAKE_POSITION_INDEPENDENT_CODE=ON
cmake --build . --config Release --parallel $(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)
cd ..

echo -e "\n\033[1;32mBuild complete!\033[0m"

# Copy library to libs directory
mkdir -p libs
LIB_SRC="$BUILD_DIR/libdocling_ffi.$LIB_EXT"
LIB_DST="libs/libdocling-ffi-${OS}_${ARCH}.$LIB_EXT"

if [ -f "$LIB_SRC" ]; then
    cp "$LIB_SRC" "$LIB_DST"
    echo -e "\033[1;32m✅ Library copied to $LIB_DST\033[0m"
else
    echo -e "\033[1;33m⚠️  Warning: Library not found at $LIB_SRC\033[0m"
fi

echo -e "\033[1;36mLibrary location: $LIB_DST\033[0m"

# Also copy to target directory for easier linking
mkdir -p target/release
cp "$LIB_DST" target/release/libdocling_ffi.$LIB_EXT 2>/dev/null || true

echo -e "\n\033[1;32mNext steps:\033[0m"
echo -e "  1. cargo build --release --features 'pdf,cli,docling-ffi'"
echo -e "  2. export LD_LIBRARY_PATH=\$PWD/libs:\$LD_LIBRARY_PATH"
echo -e "  3. ./target/release/transmutation convert document.pdf --ffi -o output.md"

