# Quality Gates Contract

## Structural Gates

- Source files must be 200 lines or fewer.
- Docs files must be 300 lines or fewer.
- Docs directories with multiple children must contain exactly one `README.md`.

## Security Gates

- Setup route is available only before first admin user exists.
- Session-protected routes reject or redirect unauthenticated access.
- Private content is never exposed to logged-out users.

## Build Gates

- `cargo fmt -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo build --release`
