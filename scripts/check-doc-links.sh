#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

errors=0

while IFS= read -r -d '' file; do
  while IFS= read -r token; do
    target="${token#*](}"
    target="${target%)}"

    # Skip external URLs and anchors.
    case "$target" in
      http://*|https://*|mailto:*)
        continue
        ;;
      \#*)
        continue
        ;;
      '')
        continue
        ;;
    esac

    target="${target%%#*}"
    if [[ "$target" == /* ]]; then
      resolved="$repo_root$target"
    else
      resolved="$(cd "$(dirname "$file")" && realpath -m "$target")"
    fi

    if [[ ! -e "$resolved" ]]; then
      echo "BROKEN: $file -> $target"
      errors=$((errors + 1))
    fi
  done < <(grep -oE '\[[^]]+\]\(([^)]+)\)' "$file" || true)
done < <(find docs -type f -name '*.md' -print0)

if [[ "$errors" -gt 0 ]]; then
  echo "Doc link check failed with $errors broken links."
  exit 1
fi

echo "Doc link check passed."
