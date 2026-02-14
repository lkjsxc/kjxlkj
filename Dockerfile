FROM rust:1-bookworm AS builder

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release -p kjxlkj-server

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libsqlite3-0 curl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --system --uid 10001 --create-home appuser

WORKDIR /app

RUN mkdir -p /data && chown -R appuser:appuser /data /app

COPY --from=builder /app/target/release/kjxlkj-server /usr/local/bin/kjxlkj-server

USER appuser

ENV BIND_ADDRESS=0.0.0.0
ENV PORT=8080
ENV DATABASE_URL=sqlite:/data/kjxlkj.db?mode=rwc
ENV JWT_SECRET=dev-secret-change-in-production

EXPOSE 8080

HEALTHCHECK --interval=10s --timeout=3s --start-period=5s --retries=5 \
  CMD curl -fsS http://127.0.0.1:8080/api/readyz >/dev/null || exit 1

CMD ["kjxlkj-server"]
