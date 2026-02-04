# Build stage
FROM rust:1.87-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copy the source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the release binary
RUN cargo build --release --bin kjxlkj

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/kjxlkj /usr/local/bin/kjxlkj

# Set terminal environment for TUI
ENV TERM=xterm-256color

WORKDIR /work

ENTRYPOINT ["kjxlkj"]
