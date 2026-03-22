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
- Keep auth semantics fixed to username `admin` with password-only login input.
- Keep content visibility private-by-default with no author attribution display.
