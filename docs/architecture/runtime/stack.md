# Runtime Stack

## Technologies

- Rust stable toolchain.
- Actix Web for HTTP routing.
- Tokio runtime for async I/O.
- Serde and serde_json for payload contracts.
- PostgreSQL for all persistent data (notes, revisions, auth, sessions).
- SimpleMDE for Markdown editing in browser.

## Runtime Intent

- Keep request handling explicit and composable.
- Separate domain logic from transport and storage adapters.
- Keep process startup deterministic via environment-driven config.
- Render Markdown server-side for read-only views, client-side for editing.
