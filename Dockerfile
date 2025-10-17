# Multi-stage build for transmutation with docling-parse FFI support
FROM ubuntu:24.04 AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    curl \
    git \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /app

# Copy source code
COPY . .

# Build C++ library
RUN chmod +x build_cpp.sh && ./build_cpp.sh

# Build Rust binary with FFI support
RUN cargo build --release --features 'pdf,cli,docling-ffi'

# Runtime stage
FROM ubuntu:24.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libstdc++6 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary and libraries
COPY --from=builder /app/target/release/transmutation /usr/local/bin/
COPY --from=builder /app/libs/libdocling-ffi-linux_x86.so /usr/local/lib/libdocling_ffi.so

# Set library path
ENV LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

# Set entrypoint
ENTRYPOINT ["transmutation"]
CMD ["--help"]

