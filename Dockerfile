# Build stage
FROM rust:1.91-alpine AS builder

WORKDIR /build

RUN apk add --no-cache musl-dev

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release && rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM alpine:3.19

RUN apk add --no-cache ca-certificates curl

RUN addgroup -S app && adduser -S app -G app

WORKDIR /app

COPY --from=builder /build/target/release/kjxlkj /app/kjxlkj

RUN mkdir -p /app/data && chown -R app:app /app

USER app

EXPOSE 8080

ENV BIND_HOST=0.0.0.0
ENV BIND_PORT=8080
ENV DATA_ROOT=/app/data

CMD ["/app/kjxlkj"]
