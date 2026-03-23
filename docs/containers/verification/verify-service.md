# Verify Service Contract

## Purpose

Run all quality gates in one deterministic container command.

## Command Bundle

- `cargo fmt -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo build --release`
- `./target/release/kjxlkj docs validate-topology`
- `./target/release/kjxlkj docs validate-terms`
- `./target/release/kjxlkj quality check-lines`
