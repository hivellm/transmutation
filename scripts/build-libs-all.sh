#!/bin/bash
# Script para build de TODAS as bibliotecas FFI usando Docker
# Gera bibliotecas para múltiplas arquiteturas

set -e

echo -e "\033[1;36m========================================"
echo -e " Building FFI libraries (All Platforms)"
echo -e "========================================\033[0m"

# Criar diretório libs se não existir
mkdir -p libs

# Detectar arquitetura do host
HOST_ARCH=$(uname -m)
echo -e "\n\033[1;33mHost architecture: $HOST_ARCH\033[0m"

# Build Linux x86_64
echo -e "\n\033[1;33m[1/2] Building for Linux x86_64...\033[0m"
docker build -f Dockerfile.build-libs -t transmutation-builder-x86 .
docker run --rm -v "${PWD}/libs:/output" transmutation-builder-x86

# Build Linux ARM64 (se buildx disponível)
if docker buildx version &>/dev/null; then
    echo -e "\n\033[1;33m[2/2] Building for Linux ARM64...\033[0m"
    docker buildx build --platform linux/arm64 -f Dockerfile.build-libs-arm -t transmutation-builder-arm --load .
    docker run --rm -v "${PWD}/libs:/output" transmutation-builder-arm
else
    echo -e "\n\033[1;33m⚠️  Docker buildx not available, skipping ARM64 build\033[0m"
    echo -e "\033[1;33m   Install with: docker buildx install\033[0m"
fi

echo -e "\n\033[1;32m========================================"
echo -e " ✅ Build complete!"
echo -e "========================================\033[0m"

echo -e "\n\033[1;36mLibraries built:\033[0m"
ls -lh libs/libdocling-ffi-*.so 2>/dev/null | awk '{print "  - " $9 " (" $5 ")"}'

echo -e "\n\033[1;36mNext steps:\033[0m"
echo -e "  1. cargo build --release --features 'pdf,cli,docling-ffi'"
echo -e "  2. export LD_LIBRARY_PATH=\$PWD/libs:\$LD_LIBRARY_PATH"
echo -e "  3. ./target/release/transmutation convert document.pdf --ffi -o output.md"

