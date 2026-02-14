# Multi-stage Rust build + single-container runtime
# Per /docs/spec/architecture/deployment.md

# Stage 1: Build the Rust binary
FROM rust:1.87-bookworm AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src/ src/

RUN cargo build --release --bin kjxlkj

# Stage 2: Runtime with PostgreSQL + app
FROM debian:bookworm-slim

# Install PostgreSQL and runtime deps
RUN apt-get update && apt-get install -y --no-install-recommends \
    postgresql-15 \
    postgresql-client-15 \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy built binary
COPY --from=builder /build/target/release/kjxlkj /app/kjxlkj

# Copy entrypoint
COPY docker/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

# Create static dir placeholder
RUN mkdir -p /app/static /app/config

# Environment defaults
ENV KJXLKJ_BIND_ADDR=0.0.0.0:8080 \
    POSTGRES_DATA_DIR=/var/lib/postgresql/data \
    DATABASE_URL=postgres://kjxlkj:kjxlkj@127.0.0.1/kjxlkj \
    BIND_ADDR=0.0.0.0:8080

EXPOSE 8080

ENTRYPOINT ["/app/entrypoint.sh"]
