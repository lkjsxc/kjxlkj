#!/usr/bin/env python3
"""Check documentation policy compliance.

This script verifies:
1. Required documentation files exist
2. Documentation structure follows policy
3. Markdown files have proper formatting
"""

import os
import sys
from pathlib import Path


def main() -> int:
    repo_root = Path(__file__).parent.parent.parent
    docs_dir = repo_root / "docs"
    
    errors = []
    
    # Check docs directory exists
    if not docs_dir.is_dir():
        errors.append("Missing /docs/ directory")
        print_errors(errors)
        return 1
    
    # Required top-level docs
    required_files = [
        "docs/README.md",
        "docs/policy/README.md",
        "docs/policy/INSTRUCT.md",
        "docs/policy/WORKFLOW.md",
        "docs/policy/STRUCTURE.md",
        "docs/policy/ROOT_LAYOUT.md",
        "docs/overview/README.md",
        "docs/overview/all-in-docs.md",
        "docs/spec/README.md",
        "docs/reference/README.md",
        "docs/guides/README.md",
    ]
    
    for rel_path in required_files:
        full_path = repo_root / rel_path
        if not full_path.is_file():
            errors.append(f"Missing required file: {rel_path}")
    
    # Check all markdown files for basic formatting
    for md_file in docs_dir.rglob("*.md"):
        rel = md_file.relative_to(repo_root)
        content = md_file.read_text(encoding="utf-8")
        
        # Check for title (first line should be heading)
        lines = content.strip().split("\n")
        if not lines or not lines[0].startswith("# "):
            errors.append(f"{rel}: Missing title heading (# Title)")
        
        # Check for unclosed code fences
        fence_count = content.count("```")
        if fence_count % 2 != 0:
            errors.append(f"{rel}: Unclosed code fence")
    
    if errors:
        print_errors(errors)
        return 1
    
    print("✓ Documentation policy check passed")
    return 0


def print_errors(errors: list[str]) -> None:
    print("Documentation policy errors:")
    for err in errors:
        print(f"  ✗ {err}")


if __name__ == "__main__":
    sys.exit(main())
