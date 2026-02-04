# syntax=docker/dockerfile:1

FROM rust:1-slim-bookworm AS builder

WORKDIR /work

COPY rust-toolchain.toml Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked -p kjxlkj

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /work/target/release/kjxlkj /usr/local/bin/kjxlkj

ENTRYPOINT ["kjxlkj"]

