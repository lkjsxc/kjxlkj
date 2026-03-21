# Quality Gates

## Structural Gates

- Source files must be 200 lines or fewer.
- Docs files must be 300 lines or fewer.
- Docs directories with multiple children require exactly one `README.md` TOC.

## Security Gates

- Setup endpoint can only run when no admin exists.
- Session-protected routes redirect or reject unauthenticated users.
- Private articles are never exposed to logged-out users.

## Build Gates

- `cargo fmt -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo build --release`
