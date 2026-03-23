# Runtime Stack

## Technologies

- Rust stable toolchain.
- Actix Web for HTTP routing.
- Tokio runtime for async I/O.
- Serde and serde_json for payload contracts.
- Local filesystem as canonical persistence layer.

## Runtime Intent

- Keep request handling explicit and composable.
- Separate domain logic from transport and storage adapters.
- Keep process startup deterministic via environment-driven config.
