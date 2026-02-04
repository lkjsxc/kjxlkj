from __future__ import annotations

import os
import re
import sys
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class Problem:
    file: str
    line: int
    message: str


def check_docs_tree(root: Path) -> list[Problem]:
    problems: list[Problem] = []
    if not root.exists():
        return [Problem(str(root), 0, "docs root does not exist")]

    # Enforce doc topology rules:
    # - max 12 direct children per directory
    # - exactly one README.md per docs directory
    for dirpath, dirnames, filenames in os.walk(root):
        path = Path(dirpath)
        children = list(dirnames) + list(filenames)
        if len(children) > 12:
            problems.append(
                Problem(str(path), 0, f"directory has {len(children)} children (max 12)")
            )

        readmes = [f for f in filenames if f == "README.md"]
        if len(readmes) != 1:
            problems.append(
                Problem(str(path), 0, f"expected exactly one README.md, found {len(readmes)}")
            )

    md_files = sorted(root.rglob("*.md"))
    link_parent_ref = re.compile(r"\]\(\.\./")

    for file in md_files:
        lines = file.read_text(encoding="utf-8").splitlines()
        if len(lines) > 200:
            problems.append(Problem(str(file), 0, f"{len(lines)} lines (max 200)"))

        in_mermaid = False
        for idx, raw in enumerate(lines, start=1):
            line = raw.strip()
            if line.startswith("~~~"):
                problems.append(Problem(str(file), idx, "tilde fences are not allowed"))
                continue

            if line.startswith("```"):
                if line == "```mermaid":
                    if in_mermaid:
                        problems.append(
                            Problem(str(file), idx, "nested ```mermaid fence")
                        )
                    in_mermaid = True
                elif line == "```":
                    if not in_mermaid:
                        problems.append(
                            Problem(str(file), idx, "closing ``` without ```mermaid")
                        )
                    in_mermaid = False
                else:
                    problems.append(
                        Problem(str(file), idx, "non-Mermaid fenced blocks are forbidden")
                    )

            if link_parent_ref.search(raw):
                problems.append(
                    Problem(str(file), idx, "links must not contain ../ (docs policy)")
                )

        if in_mermaid:
            problems.append(Problem(str(file), 0, "unterminated ```mermaid fence"))

    return problems


def main() -> int:
    root = Path("docs")
    problems = check_docs_tree(root)

    if not problems:
        print("docs policy: OK")
        return 0

    print(f"docs policy: FAILED ({len(problems)} problems)")
    for p in problems:
        loc = f"{p.file}:{p.line}" if p.line else p.file
        print(f"{loc}: {p.message}")

    return 1


if __name__ == "__main__":
    raise SystemExit(main())

