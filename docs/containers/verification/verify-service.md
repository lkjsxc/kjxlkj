# Verify Service Contract

## Purpose

Run all quality gates in one deterministic container command.

## Command Bundle

1. `cargo fmt -- --check`
2. `cargo clippy --all-targets -- -D warnings`
3. `cargo test`
4. `cargo build --release`
5. `"$CARGO_TARGET_DIR/release/kjxlkj" docs validate-topology`
6. `"$CARGO_TARGET_DIR/release/kjxlkj" docs validate-terms`
7. `"$CARGO_TARGET_DIR/release/kjxlkj" quality check-lines`

## Exit Behavior

- Exit code 0: all gates pass
- Exit code non-zero: first failing gate stops execution

## Container Requirements

- Rust toolchain installed
- Source mounted read-only at `/workspace`
- Writable Cargo target dir mounted outside the source tree at `/target`
- `CARGO_TARGET_DIR=/target`
- Authored tests and browser verification live under `src/`

## Script Template

```bash
#!/bin/bash
set -euo pipefail

target_dir="${CARGO_TARGET_DIR:-target}"
release_bin="$target_dir/release/kjxlkj"

echo "=== fmt check ==="
cargo fmt -- --check

echo "=== clippy ==="
cargo clippy --all-targets -- -D warnings

echo "=== test ==="
cargo test

echo "=== build release ==="
cargo build --release

echo "=== validate topology ==="
"$release_bin" docs validate-topology

echo "=== validate terms ==="
"$release_bin" docs validate-terms

echo "=== check lines ==="
"$release_bin" quality check-lines

echo "=== all gates passed ==="
```
