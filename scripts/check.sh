#!/usr/bin/env bash
# scripts/check.sh — Per-wave verification gate.
#
# Runs all mandatory checks per /docs/reference/CI.md:
# - cargo build --workspace (Wave-build)
# - cargo test --workspace  (Wave-test)
# - file size audit (<200 lines per /docs/policy/STRUCTURE.md)
#
# Usage: ./scripts/check.sh
set -euo pipefail

echo "=== Wave-build gate ==="
cargo build --workspace

echo ""
echo "=== Wave-test gate ==="
cargo test --workspace

echo ""
echo "=== File size audit (max 200 lines) ==="
OVER=0
while IFS= read -r line; do
  count=$(echo "$line" | awk '{print $1}')
  file=$(echo "$line" | awk '{print $2}')
  if [ "$count" -gt 200 ]; then
    echo "OVER LIMIT: $file ($count lines)"
    OVER=1
  fi
done < <(find src -name '*.rs' -o -name '*.ts' | xargs wc -l | grep -v ' total$')

if [ "$OVER" -eq 1 ]; then
  echo "FAIL: some source files exceed 200 lines"
  exit 1
fi
echo "OK: all source files under 200 lines"

echo ""
echo "=== All checks passed ==="
