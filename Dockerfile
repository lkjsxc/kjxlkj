FROM rust:1.85.0-bookworm AS builder

WORKDIR /workspace

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release -p kjxlkj-server

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --uid 10001 appuser

WORKDIR /app

COPY --from=builder /workspace/target/release/kjxlkj-server /usr/local/bin/kjxlkj-server

RUN mkdir -p /data && chown -R appuser:appuser /data

USER appuser

ENV BIND_ADDRESS=0.0.0.0 \
    PORT=8080 \
    DATABASE_URL=sqlite:/data/kjxlkj.db?mode=rwc

EXPOSE 8080

HEALTHCHECK --interval=10s --timeout=3s --retries=5 --start-period=5s \
    CMD curl -fsS http://127.0.0.1:8080/api/readyz || exit 1

CMD ["/usr/local/bin/kjxlkj-server"]
