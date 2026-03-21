# Runtime Stack

## Technologies

- Rust stable toolchain.
- Actix Web with Tokio runtime.
- Server-rendered HTML plus HTMX and minimal JavaScript.
- PostgreSQL for authentication/session data.
- Filesystem-backed Markdown content.

## Runtime Intent

- Keep request handling explicit and testable.
- Keep domain logic isolated from framework-specific concerns.
- Keep content rendering predictable for public and admin flows.
