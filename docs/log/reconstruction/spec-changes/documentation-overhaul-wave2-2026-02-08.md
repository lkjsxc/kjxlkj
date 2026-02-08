# Spec Changes: Documentation Overhaul Wave 2 (2026-02-08)

Back: [/docs/log/reconstruction/spec-changes/README.md](/docs/log/reconstruction/spec-changes/README.md)

## Objective

Second documentation improvement wave. Goal: ensure documentation is complete enough for a full, non-MVP reconstruction by Claude Opus 4.5.

## Completed changes

| Area | Change |
|---|---|
| Render pipeline | Created [/docs/spec/architecture/render-pipeline.md](/docs/spec/architecture/render-pipeline.md) |
| Input decoding | Created [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md) |
| Terminal VT100 | Created [/docs/spec/features/terminal/escape-parser.md](/docs/spec/features/terminal/escape-parser.md) |
| Thin spec expansion | 25+ spec files expanded with implementation-grade detail |
| Code block fixes | Removed non-Mermaid code blocks from 5 files |
| Doc coverage | Added new files to doc-coverage index (430 spec docs tracked) |
| TODO updates | Added items for new specs (render pipeline, input decoding, escape parser) |
| Diagnostics | Expanded with item model, command table, inline rendering spec |
| Statusline | Expanded with tabline, layout sections, highlight groups |
| Contracts | Expanded with buffer contract, persistence contract, violation consequences |
| Undo tree | Expanded with data structure, navigation commands, persistence, diff preview |
| Syntax engine | Expanded with async model, language detection, highlight mapping |
| Proposals index | Added terminal-emulator-detail.md to active proposals |

## Expanded spec files (complete list)

| File | From | To |
|---|---|---|
| registers.md | 20 lines | 85 lines |
| undo.md | 24 lines | 75 lines |
| motions.md | 31 lines | 120 lines |
| operators.md | 32 lines | 95 lines |
| text_objects.md | 20 lines | 80 lines |
| insert.md | 18 lines | 70 lines |
| completion.md | 17 lines | 55 lines |
| history.md | 17 lines | 60 lines |
| ranges.md | 19 lines | 60 lines |
| replace.md | 39 lines | 75 lines |
| advanced.md (dot/macros) | 19 lines | 65 lines |
| glossary.md | 32 lines | 70 lines |
| layout.md | 22 lines | 70 lines |
| essential.md | 33 lines | 95 lines |
| multicursor.md | 32 lines | 65 lines |
| theming.md | 13 lines | 80 lines |
| syntax.md (commands) | 39 lines | 75 lines |
| configuration.md (modes) | 29 lines | 65 lines |
| principles.md | 16 lines | 35 lines |
| git.md | 36 lines | 60 lines |
| diagnostics.md | 36 lines | 70 lines |
| statusline.md | 36 lines | 70 lines |
| contracts.md | 36 lines | 70 lines |
| undo_tree.md | 37 lines | 80 lines |
| syntax.md (engine) | 38 lines | 85 lines |

## Constraints followed

- Documentation only: no source code changes
- All TODOs unchecked (ready for fresh reconstruction)
- No `../` in links (verified)
- No non-Mermaid code blocks (verified)
- No files over 200 lines (verified)
- Doc-coverage index complete for all spec/reference/guide/log docs
