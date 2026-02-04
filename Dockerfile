# Multi-stage build for kjxlkj

# Build stage
FROM rust:1.82-slim-bookworm AS builder

WORKDIR /build

# Copy workspace manifests first for layer caching
COPY Cargo.toml ./
COPY src/crates/ ./src/crates/

# Build release binary
RUN cargo build --release --package kjxlkj

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/kjxlkj /usr/local/bin/kjxlkj

# Set terminal for TUI
ENV TERM=xterm-256color

ENTRYPOINT ["kjxlkj"]
