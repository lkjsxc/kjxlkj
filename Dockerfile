# kjxlkj Server Dockerfile
# Multi-stage build for minimal production image

# Stage 1: Build
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace manifests
COPY Cargo.toml Cargo.lock ./

# Copy crate manifests
COPY src/crates/domain/kjxlkj-domain/Cargo.toml ./src/crates/domain/kjxlkj-domain/
COPY src/crates/db/kjxlkj-db/Cargo.toml ./src/crates/db/kjxlkj-db/
COPY src/crates/auth/kjxlkj-auth/Cargo.toml ./src/crates/auth/kjxlkj-auth/
COPY src/crates/rbac/kjxlkj-rbac/Cargo.toml ./src/crates/rbac/kjxlkj-rbac/
COPY src/crates/workspace/kjxlkj-workspace/Cargo.toml ./src/crates/workspace/kjxlkj-workspace/
COPY src/crates/search/kjxlkj-search/Cargo.toml ./src/crates/search/kjxlkj-search/
COPY src/crates/automation/kjxlkj-automation/Cargo.toml ./src/crates/automation/kjxlkj-automation/
COPY src/crates/http/kjxlkj-http/Cargo.toml ./src/crates/http/kjxlkj-http/
COPY src/crates/ws/kjxlkj-ws/Cargo.toml ./src/crates/ws/kjxlkj-ws/
COPY src/crates/app/kjxlkj-server/Cargo.toml ./src/crates/app/kjxlkj-server/

# Copy source code
COPY src/crates ./src/crates

# Build application (full build with real source)
RUN cargo build --release --workspace

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -u 1000 kjxlkj

# Copy binary from builder
COPY --from=builder /app/target/release/kjxlkj-server /app/kjxlkj-server

# Copy configuration
COPY data/ ./data/
COPY static/ ./static/
COPY migrations/ ./migrations/

# Set ownership
RUN chown -R kjxlkj:kjxlkj /app

# Switch to non-root user
USER kjxlkj

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/api/healthz || exit 1

# Run server
CMD ["./kjxlkj-server"]
