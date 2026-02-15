# --- Build stage: Rust backend ---
FROM rust:1.83-bookworm AS backend-build
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src/crates/ src/crates/
RUN cargo build --release --bin kjxlkj-server

# --- Build stage: Frontend ---
FROM node:22-bookworm AS frontend-build
WORKDIR /build
COPY package.json package-lock.json* ./
RUN npm ci
COPY tsconfig.json vite.config.ts vitest.config.ts ./
COPY src/frontend/ src/frontend/
RUN npm run build

# --- Runtime ---
FROM debian:bookworm-slim
ENV DEBIAN_FRONTEND=noninteractive

# Install PostgreSQL and runtime deps
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      postgresql-15 \
      ca-certificates \
      curl \
      gosu && \
    rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy backend binary
COPY --from=backend-build /build/target/release/kjxlkj-server /app/kjxlkj-server

# Copy static assets (built frontend)
COPY --from=frontend-build /build/static /app/static

# Copy data and entrypoint
COPY data/ /app/data/
COPY docker/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

# Ensure postgres data dir exists
RUN mkdir -p /var/lib/postgresql/data && \
    chown -R postgres:postgres /var/lib/postgresql/data

ENV POSTGRES_DATA_DIR=/var/lib/postgresql/data
ENV KJXLKJ_CONFIG_PATH=/app/data/config.json

EXPOSE 8080

HEALTHCHECK --interval=10s --timeout=3s --retries=12 --start-period=30s \
  CMD curl -fsS http://127.0.0.1:8080/api/readyz || exit 1

ENTRYPOINT ["/app/entrypoint.sh"]
