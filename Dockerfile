# ---- Build stage ----
FROM rust:1.82-bookworm AS builder
WORKDIR /build

# Copy manifests first for layer caching
COPY Cargo.toml Cargo.lock ./
COPY src/crates/ src/crates/

# Build in release mode
RUN cargo build --release --bin kjxlkj-server

# ---- Runtime stage ----
FROM debian:bookworm-slim

# Install PostgreSQL 16 and runtime deps
RUN apt-get update && apt-get install -y --no-install-recommends \
    postgresql-16 \
    ca-certificates \
    curl \
    gosu \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -s /bin/bash kjxlkj

# Copy built binary
COPY --from=builder /build/target/release/kjxlkj-server /usr/local/bin/kjxlkj-server
RUN chmod +x /usr/local/bin/kjxlkj-server

# Copy runtime data and scripts
COPY data/ /app/data/
COPY scripts/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

# Create static dir for frontend assets
RUN mkdir -p /app/static

# Copy frontend dist if present
COPY src/frontend/app/dist/ /app/static/

WORKDIR /app

# PostgreSQL data directory
ENV PGDATA=/var/lib/postgresql/16/main
ENV DATABASE_URL=postgres://kjxlkj:kjxlkj@127.0.0.1:5432/kjxlkj

EXPOSE 8080

HEALTHCHECK --interval=10s --timeout=3s --retries=3 \
    CMD curl -fsS http://127.0.0.1:8080/api/healthz || exit 1

ENTRYPOINT ["/app/entrypoint.sh"]
