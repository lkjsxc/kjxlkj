# Per /docs/spec/architecture/deployment.md:
# Single-container with both PostgreSQL and app process.
# Multi-stage build: frontend assets + Rust binary.

FROM node:20-slim AS frontend
WORKDIR /build/frontend
COPY src/frontend/app/package.json src/frontend/app/package-lock.json ./
RUN npm ci
COPY src/frontend/app/ ./
RUN npm run build

FROM rust:1.88-slim-bookworm AS backend
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src/crates/ src/crates/
RUN cargo build --release -p kjxlkj-server --bin kjxlkj

FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y postgresql-15 curl && \
    rm -rf /var/lib/apt/lists/*

# App directory
WORKDIR /app

# Copy built artifacts
COPY --from=backend /build/target/release/kjxlkj /app/kjxlkj-server
COPY --from=frontend /build/frontend/dist /app/static
COPY data/config.json /app/data/config.json
COPY src/crates/db/kjxlkj-db/src/migrations/ /app/migrations/

# Copy entrypoint
COPY scripts/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

# PostgreSQL data directory
RUN mkdir -p /var/lib/postgresql/data && \
    chown -R postgres:postgres /var/lib/postgresql/data

ENV POSTGRES_DATA_DIR=/var/lib/postgresql/data
ENV KJXLKJ_CONFIG_PATH=/app/data/config.json
ENV KJXLKJ_STATIC_DIR=/app/static
ENV KJXLKJ_MIGRATIONS_DIR=/app/migrations

EXPOSE 8080

ENTRYPOINT ["/app/entrypoint.sh"]
