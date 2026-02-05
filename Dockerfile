# Build stage
FROM rust:1.83-slim AS builder

WORKDIR /app

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./
COPY src/ src/

# Build in release mode
RUN cargo build --release --bin kjxlkj

# Runtime stage
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/kjxlkj /usr/local/bin/kjxlkj

# Set entrypoint
ENTRYPOINT ["kjxlkj"]
