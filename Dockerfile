# Dockerfile for Codex CLI
# Multi-stage build for optimal image size

# Build stage
FROM rust:1.90-slim AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Set up working directory
WORKDIR /build

# Copy dependency manifests first for better caching
COPY codex-rs/Cargo.toml codex-rs/Cargo.lock ./
COPY codex-rs/*/Cargo.toml ./crates/
COPY codex-rs/rust-toolchain.toml ./
COPY codex-rs/clippy.toml ./

# Create dummy source files to build dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN mkdir -p crates/*/src && find crates -name Cargo.toml -exec dirname {} \; | xargs -I {} touch {}/src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release
RUN rm -rf src target/release/deps/codex* target/release/codex*

# Copy source code
COPY codex-rs/ ./

# Build the actual application
RUN cargo build --release --bin codex

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    git \
    bash \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN groupadd -r codex && useradd -r -g codex -s /bin/bash codex

# Set up directories
RUN mkdir -p /app /workspace /home/codex/.codex && \
    chown -R codex:codex /app /workspace /home/codex

# Copy the binary from builder
COPY --from=builder /build/target/release/codex /usr/local/bin/codex
RUN chmod +x /usr/local/bin/codex

# Copy configuration example
COPY .github/codex/home/config.toml /home/codex/.codex/config.toml.example

# Switch to non-root user
USER codex
WORKDIR /workspace

# Set environment variables
ENV HOME=/home/codex
ENV RUST_LOG=info

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD /usr/local/bin/codex --help || exit 1

# Default command
ENTRYPOINT ["/usr/local/bin/codex"]
CMD ["--help"]