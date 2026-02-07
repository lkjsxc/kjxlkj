#!/usr/bin/env python3
"""Documentation policy checker.

Validates structural constraints from /docs/policy/STRUCTURE.md:
- Every directory under docs/ has a README.md
- No directory has more than 12 direct children
- No non-exempt file exceeds 200 lines
- No relative parent links (../) in documentation
"""

import os
import sys

DOCS_ROOT = os.path.join(os.path.dirname(__file__), "..", "..", "docs")
SRC_ROOT = os.path.join(os.path.dirname(__file__), "..", "..", "src")

LINE_EXEMPT = {
    "docs/reference/CONFORMANCE_MODES_KEYS.md",
    "docs/reference/CONFORMANCE_EDITING.md",
    "docs/reference/CONFORMANCE_COMMANDS_TESTING.md",
    "docs/reference/LIMITATIONS.md",
}

MAX_ITEMS = 12
MAX_LINES = 200
errors = []


def check_docs():
    """Check documentation structure."""
    for dirpath, dirnames, filenames in os.walk(DOCS_ROOT):
        rel = os.path.relpath(dirpath, os.path.join(DOCS_ROOT, ".."))
        # README.md required
        if "README.md" not in filenames:
            errors.append(f"{rel}: missing README.md")
        # Max children
        total = len(dirnames) + len(filenames)
        if total > MAX_ITEMS:
            errors.append(f"{rel}: {total} items (max {MAX_ITEMS})")
        # Line counts and link checks
        for f in filenames:
            if not f.endswith(".md"):
                continue
            fpath = os.path.join(dirpath, f)
            rel_file = os.path.relpath(fpath, os.path.join(DOCS_ROOT, ".."))
            with open(fpath, "r", errors="replace") as fh:
                lines = fh.readlines()
            if len(lines) > MAX_LINES and rel_file not in LINE_EXEMPT:
                errors.append(f"{rel_file}: {len(lines)} lines (max {MAX_LINES})")
            for i, line in enumerate(lines, 1):
                if "../" in line and "](../" in line:
                    errors.append(f"{rel_file}:{i}: relative parent link (../)")


def check_src():
    """Check source file line counts."""
    if not os.path.isdir(SRC_ROOT):
        return
    for dirpath, _, filenames in os.walk(SRC_ROOT):
        for f in filenames:
            if not f.endswith(".rs"):
                continue
            fpath = os.path.join(dirpath, f)
            rel = os.path.relpath(fpath, os.path.join(SRC_ROOT, ".."))
            with open(fpath, "r", errors="replace") as fh:
                count = sum(1 for _ in fh)
            if count > MAX_LINES:
                errors.append(f"{rel}: {count} lines (max {MAX_LINES})")


def main():
    check_docs()
    check_src()
    if errors:
        print("Documentation policy violations:")
        for e in errors:
            print(f"  - {e}")
        sys.exit(1)
    print("Documentation policy check passed.")


if __name__ == "__main__":
    main()
