#!/bin/sh
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

echo "=== validate links ==="
"$release_bin" docs validate-links

echo "=== validate terms ==="
"$release_bin" docs validate-terms

echo "=== check lines ==="
"$release_bin" quality check-lines

echo "=== all gates passed ==="
