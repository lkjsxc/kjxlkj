#!/usr/bin/env python3
"""
Check documentation policy compliance.

Verifies:
1. No directory under docs/ exceeds 12 direct children
2. All directories under docs/ contain exactly one README.md
3. No file under docs/ exceeds 200 lines
4. No non-Mermaid fenced code blocks in docs/
"""

import os
import re
import sys
from pathlib import Path

DOCS_DIR = Path("docs")
MAX_CHILDREN = 12
MAX_LINES = 200


def check_directory_children(docs_path: Path) -> list[str]:
    """Check that no directory exceeds MAX_CHILDREN direct children."""
    errors = []
    for root, dirs, files in os.walk(docs_path):
        root_path = Path(root)
        child_count = len(dirs) + len(files)
        if child_count > MAX_CHILDREN:
            errors.append(
                f"{root_path}: {child_count} children (max {MAX_CHILDREN})"
            )
    return errors


def check_readme_presence(docs_path: Path) -> list[str]:
    """Check that every directory has exactly one README.md."""
    errors = []
    for root, dirs, files in os.walk(docs_path):
        root_path = Path(root)
        readme_count = sum(1 for f in files if f.lower() == "readme.md")
        if readme_count == 0:
            errors.append(f"{root_path}: missing README.md")
        elif readme_count > 1:
            errors.append(f"{root_path}: multiple README.md files")
    return errors


def check_line_count(docs_path: Path) -> list[str]:
    """Check that no file exceeds MAX_LINES."""
    errors = []
    for root, dirs, files in os.walk(docs_path):
        for f in files:
            if not f.endswith(".md"):
                continue
            file_path = Path(root) / f
            try:
                with open(file_path, "r", encoding="utf-8") as fp:
                    lines = fp.readlines()
                if len(lines) > MAX_LINES:
                    errors.append(
                        f"{file_path}: {len(lines)} lines (max {MAX_LINES})"
                    )
            except Exception as e:
                errors.append(f"{file_path}: could not read ({e})")
    return errors


def check_fenced_blocks(docs_path: Path) -> list[str]:
    """Check that no non-Mermaid fenced code blocks exist."""
    errors = []
    # Match fenced code blocks: ``` or ~~~ followed by optional info string
    fence_pattern = re.compile(r"^(`{3,}|~{3,})(\s*\S*)?", re.MULTILINE)
    
    for root, dirs, files in os.walk(docs_path):
        for f in files:
            if not f.endswith(".md"):
                continue
            file_path = Path(root) / f
            try:
                with open(file_path, "r", encoding="utf-8") as fp:
                    content = fp.read()
                
                # Find all fenced blocks
                in_fence = False
                fence_char = None
                fence_info = None
                
                for i, line in enumerate(content.splitlines(), 1):
                    match = fence_pattern.match(line)
                    if match:
                        fence = match.group(1)
                        info = (match.group(2) or "").strip()
                        
                        if not in_fence:
                            # Opening fence
                            in_fence = True
                            fence_char = fence[0]
                            fence_info = info
                            
                            # Check for tilde fences (not allowed)
                            if fence_char == "~":
                                errors.append(
                                    f"{file_path}:{i}: tilde fence not allowed"
                                )
                            # Check for non-mermaid info string
                            elif info and info != "mermaid":
                                errors.append(
                                    f"{file_path}:{i}: non-Mermaid fence "
                                    f"'{info}' not allowed"
                                )
                            # Check for unlabeled fence (no info string)
                            elif not info:
                                errors.append(
                                    f"{file_path}:{i}: unlabeled fence "
                                    "not allowed"
                                )
                        else:
                            # Closing fence
                            if fence[0] == fence_char:
                                in_fence = False
                                fence_char = None
                                fence_info = None
                                
            except Exception as e:
                errors.append(f"{file_path}: could not read ({e})")
    
    return errors


def main() -> int:
    if not DOCS_DIR.exists():
        print(f"Warning: {DOCS_DIR} does not exist, skipping docs policy check")
        return 0
    
    all_errors = []
    
    print("Checking directory children count...")
    all_errors.extend(check_directory_children(DOCS_DIR))
    
    print("Checking README.md presence...")
    all_errors.extend(check_readme_presence(DOCS_DIR))
    
    print("Checking line counts...")
    all_errors.extend(check_line_count(DOCS_DIR))
    
    print("Checking fenced blocks...")
    all_errors.extend(check_fenced_blocks(DOCS_DIR))
    
    if all_errors:
        print("\nPolicy violations found:")
        for error in all_errors:
            print(f"  - {error}")
        return 1
    else:
        print("\nAll policy checks passed!")
        return 0


if __name__ == "__main__":
    sys.exit(main())
