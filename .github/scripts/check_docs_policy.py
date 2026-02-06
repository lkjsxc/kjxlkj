#!/usr/bin/env python3
"""Check documentation policy compliance.

Rules enforced:
- No non-Mermaid fenced code blocks under docs/.
- Files under 200 lines.
- Directories have no more than 12 children.
- Each directory has a README.md.
"""

import os
import re
import sys

DOCS_ROOT = os.path.join(os.path.dirname(__file__), "..", "..", "docs")
DOCS_ROOT = os.path.abspath(DOCS_ROOT)

errors = []

def check_fenced_blocks(path, content):
    """No non-Mermaid fenced blocks."""
    in_fence = False
    fence_lang = None
    for i, line in enumerate(content.splitlines(), 1):
        if re.match(r'^```', line):
            if in_fence:
                in_fence = False
                fence_lang = None
            else:
                in_fence = True
                m = re.match(r'^```(\w+)', line)
                fence_lang = m.group(1) if m else ""
                if fence_lang.lower() != "mermaid" and fence_lang != "":
                    errors.append(
                        f"{path}:{i}: non-Mermaid fenced block ({fence_lang})"
                    )

def check_line_count(path, content):
    lines = content.splitlines()
    if len(lines) > 200:
        errors.append(f"{path}: {len(lines)} lines (max 200)")

def check_directory_children(dirpath):
    children = os.listdir(dirpath)
    if len(children) > 12:
        errors.append(f"{dirpath}: {len(children)} children (max 12)")
    if "README.md" not in children:
        errors.append(f"{dirpath}: missing README.md")

def main():
    for root, dirs, files in os.walk(DOCS_ROOT):
        check_directory_children(root)
        for f in files:
            if not f.endswith(".md"):
                continue
            path = os.path.join(root, f)
            with open(path, encoding="utf-8") as fh:
                content = fh.read()
            check_fenced_blocks(path, content)
            check_line_count(path, content)

    if errors:
        print(f"Doc policy violations ({len(errors)}):")
        for e in errors:
            print(f"  {e}")
        sys.exit(1)
    else:
        print("Doc policy: OK")

if __name__ == "__main__":
    main()
