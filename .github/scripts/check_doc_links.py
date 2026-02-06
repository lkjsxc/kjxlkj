#!/usr/bin/env python3
"""Check for broken internal links in documentation."""

import os
import re
import sys

DOCS_ROOT = os.path.join(os.path.dirname(__file__), "..", "..", "docs")
DOCS_ROOT = os.path.abspath(DOCS_ROOT)
REPO_ROOT = os.path.join(DOCS_ROOT, "..")

errors = []

LINK_RE = re.compile(r'\[([^\]]*)\]\(([^)]+)\)')

def check_links(filepath, content):
    for i, line in enumerate(content.splitlines(), 1):
        for match in LINK_RE.finditer(line):
            target = match.group(2)
            # Skip external links
            if target.startswith("http://") or target.startswith("https://"):
                continue
            # Skip anchors
            if target.startswith("#"):
                continue
            # Resolve relative to file or repo root
            if target.startswith("/"):
                resolved = os.path.join(REPO_ROOT, target.lstrip("/"))
            else:
                resolved = os.path.join(os.path.dirname(filepath), target)
            resolved = os.path.normpath(resolved)
            # Strip anchor from path
            if "#" in resolved:
                resolved = resolved.split("#")[0]
            if not os.path.exists(resolved):
                errors.append(f"{filepath}:{i}: broken link → {target}")

def main():
    for root, dirs, files in os.walk(DOCS_ROOT):
        for f in files:
            if not f.endswith(".md"):
                continue
            path = os.path.join(root, f)
            with open(path, encoding="utf-8") as fh:
                content = fh.read()
            check_links(path, content)

    if errors:
        print(f"Broken links ({len(errors)}):")
        for e in errors:
            print(f"  {e}")
        sys.exit(1)
    else:
        print("Doc links: OK")

if __name__ == "__main__":
    main()
