#!/bin/sh
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
