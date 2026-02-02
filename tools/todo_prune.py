#!/usr/bin/env python3

from __future__ import annotations

import argparse
from pathlib import Path


def prune_checked_lines(lines: list[str]) -> list[str]:
    kept: list[str] = []
    previous_blank = False

    for line in lines:
        stripped = line.lstrip()
        if stripped.startswith("- [x]") or stripped.startswith("- [X]"):
            continue

        is_blank = stripped == "" or stripped == "\n"
        if is_blank:
            if previous_blank:
                continue
            previous_blank = True
            kept.append("\n")
            continue

        previous_blank = False
        kept.append(line)

    while kept and kept[-1] == "\n":
        kept.pop()
    kept.append("\n")
    return kept


def main() -> int:
    parser = argparse.ArgumentParser(description="Delete checked TODO items (- [x]) from a markdown file.")
    parser.add_argument("path", nargs="?", default="TODO.md", help="Path to TODO file (default: TODO.md)")
    parser.add_argument("--check", action="store_true", help="Exit 0 if changes needed, else exit 1")
    args = parser.parse_args()

    path = Path(args.path)
    original = path.read_text(encoding="utf-8").splitlines(keepends=True)
    pruned = prune_checked_lines(original)

    if pruned == original:
        return 1 if args.check else 0

    if args.check:
        return 0

    path.write_text("".join(pruned), encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
