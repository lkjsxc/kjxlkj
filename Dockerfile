FROM rust:1-bookworm AS builder
WORKDIR /build
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY src/ src/
RUN cargo build --release --locked

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/kjxlkj /usr/local/bin/kjxlkj
ENTRYPOINT ["kjxlkj"]
