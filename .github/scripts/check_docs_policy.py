#!/usr/bin/env python3

from __future__ import annotations

import re
import sys
from dataclasses import dataclass
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
DOCS_ROOT = REPO_ROOT / "docs"

EXEMPT_LINE_CAP = {
    (DOCS_ROOT / "reference" / "LIMITATIONS.md").resolve(),
}


@dataclass(frozen=True)
class PolicyError:
    path: Path
    message: str


def iter_docs_dirs(root: Path) -> list[Path]:
    return [p for p in root.rglob("*") if p.is_dir() and not p.name.startswith(".")] + [root]


def iter_md_files(root: Path) -> list[Path]:
    return [p for p in root.rglob("*.md") if p.is_file()]


def count_visible_children(dir_path: Path) -> int:
    return sum(1 for p in dir_path.iterdir() if not p.name.startswith("."))


def check_docs_directory_structure() -> list[PolicyError]:
    errors: list[PolicyError] = []
    for d in iter_docs_dirs(DOCS_ROOT):
        readme = d / "README.md"
        if not readme.exists():
            errors.append(PolicyError(d, "missing README.md"))
        child_count = count_visible_children(d)
        if child_count > 12:
            errors.append(PolicyError(d, f"directory has {child_count} direct children (max 12)"))
    return errors


FENCE_TICK_RE = re.compile(r"^\s*```(.*)$")
FENCE_TILDE_RE = re.compile(r"^\s*~~~")


def check_markdown_fences(path: Path, lines: list[str]) -> list[PolicyError]:
    errors: list[PolicyError] = []
    in_fence: str | None = None
    fence_info: str | None = None

    for idx, raw in enumerate(lines, start=1):
        line = raw.rstrip("\n")
        if FENCE_TILDE_RE.match(line):
            errors.append(PolicyError(path, f"line {idx}: tilde fences are forbidden"))
            continue

        m = FENCE_TICK_RE.match(line)
        if not m:
            continue

        rest = m.group(1).strip()
        if in_fence is None:
            in_fence = "```"
            fence_info = rest
            if fence_info == "":
                errors.append(PolicyError(path, f"line {idx}: unlabeled fenced block is forbidden"))
            elif fence_info != "mermaid":
                errors.append(PolicyError(path, f"line {idx}: fenced block info must be 'mermaid' (got {fence_info!r})"))
        else:
            in_fence = None
            fence_info = None

    if in_fence is not None:
        errors.append(PolicyError(path, "unterminated fenced block"))
    return errors


def check_markdown_line_cap(path: Path, lines: list[str]) -> list[PolicyError]:
    if path.resolve() in EXEMPT_LINE_CAP:
        return []
    if len(lines) <= 200:
        return []
    return [PolicyError(path, f"file has {len(lines)} lines (max 200)")]


def check_docs_files() -> list[PolicyError]:
    errors: list[PolicyError] = []
    for md in iter_md_files(DOCS_ROOT):
        try:
            text = md.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            errors.append(PolicyError(md, "not valid UTF-8"))
            continue
        lines = text.splitlines(True)
        errors.extend(check_markdown_fences(md, lines))
        errors.extend(check_markdown_line_cap(md, lines))
    return errors


def main() -> int:
    if not DOCS_ROOT.exists():
        print("docs/ does not exist", file=sys.stderr)
        return 2

    errors = [*check_docs_directory_structure(), *check_docs_files()]
    if not errors:
        print("docs policy: ok")
        return 0

    for e in errors:
        rel = e.path.resolve().relative_to(REPO_ROOT)
        print(f"docs policy: {rel}: {e.message}")
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
