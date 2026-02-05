#!/usr/bin/env python3

from __future__ import annotations

import re
import sys
from dataclasses import dataclass
from pathlib import Path


DOCS_DIR_NAME = "docs"
MAX_DOC_LINES = 200
MAX_DIR_CHILDREN = 12


@dataclass(frozen=True)
class Violation:
    path: Path
    line: int | None
    message: str

    def format(self) -> str:
        rel = self.path.as_posix()
        if self.line is None:
            return f"{rel}: {self.message}"
        return f"{rel}:{self.line}: {self.message}"


def repo_root() -> Path:
    return Path(__file__).resolve().parents[2]


def iter_doc_dirs(docs_root: Path):
    for p in docs_root.rglob("*"):
        if p.is_dir():
            yield p


def iter_doc_files(docs_root: Path):
    for p in docs_root.rglob("*.md"):
        if p.is_file():
            yield p


_FENCE_BACKTICK_RE = re.compile(r"^\s*```+")
_FENCE_TILDE_RE = re.compile(r"^\s*~~~+")
_LINK_PARENT_RE = re.compile(r"\]\(\.\./")
_LINK_REF_PARENT_RE = re.compile(r"^\s*\[[^\]]+\]:\s*\.\./")


def check_fences(path: Path, text: str) -> list[Violation]:
    violations: list[Violation] = []
    in_mermaid = False

    for idx, line in enumerate(text.splitlines(), start=1):
        if _FENCE_TILDE_RE.match(line):
            violations.append(
                Violation(
                    path=path,
                    line=idx,
                    message="Tilde fenced blocks are forbidden under /docs/.",
                )
            )
            continue

        if not _FENCE_BACKTICK_RE.match(line):
            continue

        info = line.strip()[3:].strip()
        if not in_mermaid:
            if info == "mermaid":
                in_mermaid = True
            else:
                violations.append(
                    Violation(
                        path=path,
                        line=idx,
                        message=(
                            "Fenced blocks are forbidden under /docs/ except a Mermaid fence "
                            "with info string exactly 'mermaid'."
                        ),
                    )
                )
        else:
            # Any backtick fence line closes the Mermaid block.
            in_mermaid = False

    if in_mermaid:
        violations.append(
            Violation(
                path=path,
                line=None,
                message="Unterminated Mermaid fence (missing closing ```).",
            )
        )

    return violations


def check_links(path: Path, text: str) -> list[Violation]:
    violations: list[Violation] = []
    for idx, line in enumerate(text.splitlines(), start=1):
        if "../" not in line:
            continue
        if _LINK_PARENT_RE.search(line) or _LINK_REF_PARENT_RE.search(line):
            violations.append(
                Violation(
                    path=path,
                    line=idx,
                    message="Documentation links must not use '../' (use repo-root paths or local links).",
                )
            )
    return violations


def check_doc_file_limits(path: Path, text: str) -> list[Violation]:
    line_count = text.count("\n") + 1 if text else 0
    if line_count <= MAX_DOC_LINES:
        return []
    return [
        Violation(
            path=path,
            line=None,
            message=f"Documentation file exceeds {MAX_DOC_LINES} lines ({line_count}).",
        )
    ]


def check_dir_structure(docs_root: Path) -> list[Violation]:
    violations: list[Violation] = []

    for d in [docs_root, *iter_doc_dirs(docs_root)]:
        if not d.is_dir():
            continue

        entries = [p for p in d.iterdir() if not p.name.startswith(".")]
        if len(entries) > MAX_DIR_CHILDREN:
            violations.append(
                Violation(
                    path=d,
                    line=None,
                    message=f"Directory exceeds {MAX_DIR_CHILDREN} children ({len(entries)}).",
                )
            )

        readmes = [p for p in entries if p.is_file() and p.name == "README.md"]
        if len(readmes) != 1:
            violations.append(
                Violation(
                    path=d,
                    line=None,
                    message=f"Directory must contain exactly one README.md (found {len(readmes)}).",
                )
            )

    return violations


def main() -> int:
    root = repo_root()
    docs_root = root / DOCS_DIR_NAME

    if not docs_root.exists():
        print(f"ERROR: Missing {DOCS_DIR_NAME}/ directory.")
        return 1

    violations: list[Violation] = []
    violations.extend(check_dir_structure(docs_root))

    for path in iter_doc_files(docs_root):
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            violations.append(
                Violation(
                    path=path,
                    line=None,
                    message="Doc file must be valid UTF-8.",
                )
            )
            continue

        violations.extend(check_fences(path, text))
        violations.extend(check_links(path, text))
        violations.extend(check_doc_file_limits(path, text))

    if not violations:
        return 0

    print("Documentation policy violations found:")
    for v in sorted(violations, key=lambda x: (x.path.as_posix(), x.line or 0, x.message)):
        print(f"- {v.format()}")

    return 1


if __name__ == "__main__":
    raise SystemExit(main())
