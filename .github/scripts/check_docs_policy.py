#!/usr/bin/env python3
"""Check documentation policy compliance."""

import os
import re
import sys
from pathlib import Path

DOCS_DIR = Path("docs")
MAX_LINES = 200
MAX_CHILDREN = 12


def check_file(path: Path) -> list[str]:
    """Check a single file for policy violations."""
    errors = []
    
    try:
        content = path.read_text()
    except Exception as e:
        errors.append(f"{path}: Cannot read file: {e}")
        return errors
    
    lines = content.split('\n')
    
    if len(lines) > MAX_LINES:
        errors.append(f"{path}: Exceeds {MAX_LINES} lines ({len(lines)} lines)")
    
    in_fence = False
    fence_start = 0
    
    for i, line in enumerate(lines, 1):
        if re.match(r'^```', line):
            if not in_fence:
                in_fence = True
                fence_start = i
                fence_info = line[3:].strip()
                if fence_info and fence_info != 'mermaid':
                    errors.append(f"{path}:{i}: Non-Mermaid fenced block ({fence_info})")
                elif not fence_info:
                    errors.append(f"{path}:{i}: Unlabeled fenced block")
            else:
                in_fence = False
        
        if '../' in line and not line.strip().startswith('#'):
            if re.search(r'\]\([^)]*\.\./[^)]*\)', line):
                errors.append(f"{path}:{i}: Link uses '../'")
    
    return errors


def check_directory(path: Path) -> list[str]:
    """Check a directory for policy compliance."""
    errors = []
    
    readme = path / "README.md"
    if not readme.exists():
        errors.append(f"{path}: Missing README.md")
    
    children = [p for p in path.iterdir() if not p.name.startswith('.')]
    if len(children) > MAX_CHILDREN:
        errors.append(f"{path}: Exceeds {MAX_CHILDREN} children ({len(children)})")
    
    return errors


def main():
    """Run all policy checks."""
    if not DOCS_DIR.exists():
        print(f"Error: {DOCS_DIR} does not exist")
        return 1
    
    errors = []
    
    for path in DOCS_DIR.rglob("*.md"):
        errors.extend(check_file(path))
    
    for path in DOCS_DIR.rglob("*"):
        if path.is_dir():
            errors.extend(check_directory(path))
    
    if errors:
        print("Documentation policy violations found:")
        for error in sorted(set(errors)):
            print(f"  - {error}")
        return 1
    
    print("Documentation policy check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
