# syntax=docker/dockerfile:1

FROM rust:1.88-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests
COPY docs ./docs
COPY README.md LICENSE ./

RUN cargo build --release --bin kjxlkj --bin kjxlkj-app

FROM rust:1.88-bookworm AS verify
WORKDIR /workspace

RUN rustup component add rustfmt clippy

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests
COPY docs ./docs
COPY README.md LICENSE ./

CMD ["/bin/bash", "-c", "cargo fmt -- --check && cargo clippy --all-targets -- -D warnings && cargo test && cargo build --release && ./target/release/kjxlkj docs validate-topology && ./target/release/kjxlkj docs validate-terms && ./target/release/kjxlkj quality check-lines"]

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/kjxlkj-app /usr/local/bin/kjxlkj-app

RUN mkdir -p /app/data/records

ENV BIND_HOST=0.0.0.0 \
    BIND_PORT=8080 \
    DATA_ROOT=/app/data \
    DATABASE_URL=postgresql://kjxlkj:kjxlkj@postgres:5432/kjxlkj \
    ADMIN_TOKEN=local-dev-token

EXPOSE 8080

CMD ["/usr/local/bin/kjxlkj-app", "serve"]
