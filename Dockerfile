FROM rust:1.88-bookworm AS build
WORKDIR /work
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src
RUN cargo build --release -p kjxlkj-server

FROM node:20-bookworm AS frontend-build
WORKDIR /work/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend .
RUN npm run build

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl postgresql postgresql-client tini \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /work/target/release/kjxlkj-server /usr/local/bin/kjxlkj-server
COPY scripts/start-single-container.sh /usr/local/bin/start-single-container.sh
COPY --from=frontend-build /work/frontend/dist /app/frontend/dist
WORKDIR /app
ENV BIND_ADDR=0.0.0.0:8080
ENV EXPORT_DIR=/tmp/kjxlkj-exports
ENV BACKUP_DIR=/tmp/kjxlkj-backups
EXPOSE 8080
ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["/usr/local/bin/start-single-container.sh"]
