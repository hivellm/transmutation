#!/bin/bash
# Script para build das bibliotecas FFI usando Docker
# Gera as bibliotecas e copia para ./libs/

set -e

echo -e "\033[1;36m========================================\033[0m"
echo -e "\033[1;36m Building FFI libraries with Docker    \033[0m"
echo -e "\033[1;36m========================================\033[0m"

# Criar diretório libs se não existir
mkdir -p libs

# Build da imagem Docker
echo -e "\n\033[1;33m[1/2] Building Docker image...\033[0m"
docker build -f Dockerfile.build-libs -t transmutation-builder .

# Executar container e copiar libs
echo -e "\n\033[1;33m[2/2] Extracting libraries...\033[0m"
docker run --rm -v "${PWD}/libs:/output" transmutation-builder

echo -e "\n\033[1;32m========================================\033[0m"
echo -e "\033[1;32m ✅ Build complete!                     \033[0m"
echo -e "\033[1;32m========================================\033[0m"

echo -e "\n\033[1;36mLibraries available:\033[0m"
ls -lh libs/libdocling-ffi-*.so 2>/dev/null || echo "  No libraries found"

echo -e "\n\033[1;36mNext steps:\033[0m"
echo -e "  1. cargo build --release --features 'pdf,cli,docling-ffi'"
echo -e "  2. export LD_LIBRARY_PATH=\$PWD/libs:\$LD_LIBRARY_PATH"
echo -e "  3. ./target/release/transmutation convert document.pdf --ffi -o output.md"

