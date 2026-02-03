# Checks (2026-02-03)

Back: [/docs/log/2026/02/03/README.md](/docs/log/2026/02/03/README.md)

## Documentation line limits

All `docs/**/*.md` files are ≤200 lines at the time of this entry.

## Source line limits

Implementation files MUST remain ≤200 lines per file; any exception MUST be recorded here with rationale.

As of 2026-02-03:

- All `src/**/*.rs` files are ≤200 lines.
- `cargo test` passes.
- `cargo clippy --all-targets --all-features -- -D warnings` passes.
