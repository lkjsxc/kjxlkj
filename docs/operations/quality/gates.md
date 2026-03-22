# Quality Gates Contract

## Structural Gates

- Source files must be 200 lines or fewer.
- Docs files must be 300 lines or fewer.
- Docs directories with multiple children must contain exactly one `README.md`.
- Restricted-language scan must pass (`docs validate-terms`).

## Security Gates

- Setup route is available only before first admin user exists.
- Session-protected routes reject or redirect unauthenticated access.
- Private content is never exposed to logged-out users.
- Fixed admin identity is enforced (`admin`, password-only login).
- Article rendering must not expose author attribution/byline display.

## Build Gates

- `cargo fmt -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo build --release`
- `cargo run --bin kjxlkj -- docs validate-topology`
- `cargo run --bin kjxlkj -- docs validate-terms`
- `cargo run --bin kjxlkj -- quality check-lines`
