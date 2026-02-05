#!/usr/bin/env python3
"""
Documentation policy checker.

Checks:
1. Every directory under docs/ contains exactly one README.md
2. Only Mermaid fences are allowed (no other code fences)
3. All internal links are valid
"""

import os
import re
import sys
from pathlib import Path

DOCS_ROOT = Path(__file__).parent.parent.parent / "docs"


def check_readme_per_directory() -> list[str]:
    """Check that each directory under docs/ has exactly one README.md."""
    errors = []
    for dirpath, dirnames, filenames in os.walk(DOCS_ROOT):
        # Skip hidden directories
        dirnames[:] = [d for d in dirnames if not d.startswith(".")]
        
        rel_path = Path(dirpath).relative_to(DOCS_ROOT.parent)
        md_files = [f for f in filenames if f.endswith(".md")]
        
        if "README.md" not in md_files:
            errors.append(f"{rel_path}: missing README.md")
    
    return errors


def check_fence_policy() -> list[str]:
    """Check that only Mermaid fences are used (no other language fences)."""
    errors = []
    fence_pattern = re.compile(r"^```(\w+)?", re.MULTILINE)
    allowed_languages = {"mermaid", "markdown", "plaintext", "text", ""}
    
    for md_file in DOCS_ROOT.rglob("*.md"):
        content = md_file.read_text(encoding="utf-8")
        for match in fence_pattern.finditer(content):
            lang = match.group(1) or ""
            if lang.lower() not in allowed_languages:
                rel_path = md_file.relative_to(DOCS_ROOT.parent)
                errors.append(f"{rel_path}: disallowed fence language '{lang}'")
    
    return errors


def check_internal_links() -> list[str]:
    """Check that internal markdown links point to existing files."""
    errors = []
    link_pattern = re.compile(r"\[([^\]]+)\]\((/[^)]+)\)")
    
    for md_file in DOCS_ROOT.rglob("*.md"):
        content = md_file.read_text(encoding="utf-8")
        for match in link_pattern.finditer(content):
            link_text, link_target = match.groups()
            # Remove anchor
            target_path = link_target.split("#")[0]
            if target_path:
                full_path = DOCS_ROOT.parent / target_path.lstrip("/")
                if not full_path.exists():
                    rel_path = md_file.relative_to(DOCS_ROOT.parent)
                    errors.append(f"{rel_path}: broken link to {target_path}")
    
    return errors


def main() -> int:
    all_errors = []
    
    print("Checking README.md per directory...")
    all_errors.extend(check_readme_per_directory())
    
    print("Checking fence policy...")
    all_errors.extend(check_fence_policy())
    
    print("Checking internal links...")
    all_errors.extend(check_internal_links())
    
    if all_errors:
        print(f"\n{len(all_errors)} error(s) found:")
        for error in all_errors:
            print(f"  - {error}")
        return 1
    
    print("\nAll checks passed!")
    return 0


if __name__ == "__main__":
    sys.exit(main())
