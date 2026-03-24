# Verify Service Contract

## Purpose

Run all quality gates in one deterministic container command.

## Command Bundle

1. `cargo fmt -- --check`
2. `cargo clippy --all-targets -- -D warnings`
3. `cargo test`
4. `cargo build --release`
5. `./target/release/kjxlkj docs validate-topology`
6. `./target/release/kjxlkj docs validate-terms`
7. `./target/release/kjxlkj quality check-lines`

## Exit Behavior

- Exit code 0: all gates pass
- Exit code non-zero: first failing gate stops execution

## Container Requirements

- Rust toolchain installed
- Source mounted at `/app`
- Docs mounted for validation

## Script Template

```bash
#!/bin/bash
set -euo pipefail

echo "=== fmt check ==="
cargo fmt -- --check

echo "=== clippy ==="
cargo clippy --all-targets -- -D warnings

echo "=== test ==="
cargo test

echo "=== build release ==="
cargo build --release

echo "=== validate topology ==="
./target/release/kjxlkj docs validate-topology

echo "=== validate terms ==="
./target/release/kjxlkj docs validate-terms

echo "=== check lines ==="
./target/release/kjxlkj quality check-lines

echo "=== all gates passed ==="
```
