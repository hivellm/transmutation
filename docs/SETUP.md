# Setup and Installation Guide

## Requirements

### Rust Toolchain
- **Required:** Rust 1.85+ with Edition 2024 support
- **Installation:**
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
  ```

### System Dependencies

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install build-essential cmake git zlib1g-dev pkg-config libssl-dev
```

**macOS:**
```bash
xcode-select --install
brew install cmake openssl pkg-config
```

**Windows:**
- Visual Studio Build Tools or MinGW
- For FFI: Use WSL (recommended)

---

## Build Methods

### 1. Quick Build (Pure Rust - No FFI)

**Recommended for most users** - Zero external dependencies.

```bash
cargo build --release --features pdf,cli
```

**Features:**
- ✅ 250x faster than Docling
- ✅ 80.40% similarity
- ✅ Zero dependencies
- ✅ Single binary

---

### 2. Build with FFI (Advanced - 95%+ Similarity)

Requires docling-parse C++ libraries.

#### Option A: Docker Build (Easiest) ⭐

**Linux/macOS:**
```bash
./build-libs-docker.sh
cargo build --release --features pdf,cli,docling-ffi
```

**Windows:**
```powershell
.\build-libs-docker.ps1
wsl -d Ubuntu-24.04 -- bash -c "cd /mnt/f/Node/hivellm/transmutation && cargo build --release --features pdf,cli,docling-ffi"
```

**Output:**
- `libs/libdocling-ffi-linux_x86.so` (7.4MB)

#### Option B: Native Build (Linux/WSL)

```bash
# Build C++ libraries
./build_cpp.sh

# Build Rust with FFI
cargo build --release --features pdf,cli,docling-ffi

# Copy FFI library
cp build_linux_x86/libdocling_ffi.so target/release/
```

#### Option C: Cross-platform (x86 + ARM)

```bash
# Linux/macOS
./build-libs-all.sh

# Windows
.\build-libs-all.ps1
```

**Output:**
- `libs/libdocling-ffi-linux_x86.so`
- `libs/libdocling-ffi-linux_ARM.so`

---

## FFI Setup (Required for --ffi mode)

### 1. Create Resource Symlink

The docling-parse library requires resources in a specific location:

```bash
# Run from repository root (hivellm/)
cd /path/to/hivellm
mkdir -p docling_parse
cd docling_parse
ln -sfn ../transmutation/docling-parse/docling_parse/pdf_resources_v2 pdf_resources_v2
```

**Verification:**
```bash
ls -la docling_parse/pdf_resources_v2/
# Should show: cmap-resources, encodings, fonts, glyphs
```

### 2. Configure Library Path

**Linux/WSL:**
```bash
export LD_LIBRARY_PATH=$PWD/target/release:$LD_LIBRARY_PATH
```

**Add to ~/.bashrc for persistence:**
```bash
echo 'export LD_LIBRARY_PATH=/path/to/transmutation/target/release:$LD_LIBRARY_PATH' >> ~/.bashrc
```

**macOS:**
```bash
export DYLD_LIBRARY_PATH=$PWD/target/release:$DYLD_LIBRARY_PATH
```

---

## Docker Build Details

### What Docker Builds

Docker creates a clean Ubuntu 24.04 environment with all dependencies and compiles:
1. docling-parse C++ library (2.9MB static libs)
2. FFI wrapper (7.4MB shared library)
3. Copies output to `./libs/`

### Platform Support

| Platform | Docker Support | Notes |
|----------|---------------|-------|
| **Linux x86_64** | ✅ Yes | Primary target |
| **Linux ARM64** | ✅ Yes (buildx) | Raspberry Pi, ARM servers |
| **Windows** | ⚠️ Via WSL | Docker generates Linux binaries |
| **macOS** | ❌ No | Use native build |

### Docker Scripts

**Simple build (x86 only):**
- `build-libs-docker.sh` / `.ps1` - Builds Linux x86_64 library

**Multi-arch build:**
- `build-libs-all.sh` / `.ps1` - Builds x86_64 + ARM64 libraries

### Dockerfile Structure

```
Dockerfile.build-libs          # Linux x86_64 builder
Dockerfile.build-libs-arm      # Linux ARM64 builder
```

**What's inside:**
- Base: Ubuntu 24.04
- Dependencies: build-essential, cmake, git, zlib1g-dev
- Build: Runs `build_cpp.sh`
- Output: Copies `.so` to `/output`

### Docker Cleanup

```bash
# Remove images
docker rmi transmutation-builder-x86 transmutation-builder-arm

# Clean cache
docker builder prune

# Remove libs (optional)
rm -rf libs/
```

---

## Verification

### Check Build

```bash
# Check binary
./target/release/transmutation --version

# Check FFI library (if built)
ls -lh target/release/libdocling_ffi.so
ls -lh libs/libdocling-ffi-*.so
```

### Test Conversion

**Fast Mode (no FFI):**
```bash
./target/release/transmutation convert document.pdf -o output.md
```

**Precision Mode (no FFI):**
```bash
./target/release/transmutation convert document.pdf --precision -o output.md
```

**FFI Mode (requires FFI build):**
```bash
export LD_LIBRARY_PATH=$PWD/target/release:$LD_LIBRARY_PATH
./target/release/transmutation convert document.pdf --ffi -o output.json
```

---

## Troubleshooting

### Edition 2024 Errors

**Error:** `feature 'edition2024' is required`

**Solution:** Update Rust or use nightly:
```bash
rustup update stable
# OR
rustup install nightly && rustup default nightly
```

### Library Not Found (FFI)

**Error:** `libdocling_ffi.so: cannot open shared object file`

**Solutions:**
1. Set `LD_LIBRARY_PATH`:
   ```bash
   export LD_LIBRARY_PATH=$PWD/target/release:$LD_LIBRARY_PATH
   ```

2. Copy library:
   ```bash
   cp build_linux_x86/libdocling_ffi.so target/release/
   ```

3. System-wide install (not recommended):
   ```bash
   sudo cp libs/libdocling-ffi-linux_x86.so /usr/local/lib/
   sudo ldconfig
   ```

### Resources Not Found (FFI)

**Error:** `ERR| no existing pdf_resources_dir: ../docling_parse/pdf_resources_v2/`

**Solution:** Create symlink (see "FFI Setup" above)

### Docker Build Fails

**Error:** `docker: command not found`

**Solution:** Install Docker Desktop from https://docs.docker.com/get-docker/

**Error:** `permission denied while trying to connect to the Docker daemon`

**Solution (Linux):**
```bash
sudo usermod -aG docker $USER
newgrp docker
```

### CMake/Build Errors

**Error:** ZLIB not found

**Solution:**
```bash
# Ubuntu/Debian
sudo apt-get install zlib1g-dev

# macOS
brew install zlib
```

---

## Build Comparison

| Method | Difficulty | Time | Platform | Recommendation |
|--------|-----------|------|----------|----------------|
| **Pure Rust** | ⭐ Easy | 2min | All | ✅ Start here |
| **Docker FFI** | ⭐⭐ Medium | 10min | Linux/Win(WSL) | ✅ Best for FFI |
| **Native FFI (Linux)** | ⭐⭐⭐ Hard | 5min | Linux/WSL | For developers |
| **Native FFI (Windows)** | ⭐⭐⭐⭐⭐ Very Hard | N/A | Windows | ❌ Use WSL instead |

---

## Next Steps

1. ✅ Build the project (start with Pure Rust)
2. ✅ Read [CLI_GUIDE.md](CLI_GUIDE.md) for usage
3. ✅ Check [FFI.md](FFI.md) if you need 95%+ similarity
4. ✅ See [BENCHMARKS.md](BENCHMARKS.md) for performance details


