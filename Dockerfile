# syntax=docker/dockerfile:1

FROM rust:1.88-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY data ./data

RUN cargo build --release -p kjxlkj-server

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
RUN useradd --uid 10001 --create-home appuser

COPY --from=builder /app/target/release/kjxlkj-server /usr/local/bin/kjxlkj-server
COPY data ./data

RUN mkdir -p /app/static /app/data/attachments /app/data/backups \
    && chown -R appuser:appuser /app

USER appuser
EXPOSE 8080

CMD ["kjxlkj-server"]
