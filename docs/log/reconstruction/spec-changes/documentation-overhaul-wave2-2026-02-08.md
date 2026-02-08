# Spec Changes: Documentation Overhaul Wave 2 (2026-02-08)

Back: [/docs/log/reconstruction/spec-changes/README.md](/docs/log/reconstruction/spec-changes/README.md)

## Objective

Second major documentation improvement wave. Goal: ensure the documentation is complete enough for a full, non-MVP reconstruction by Claude Opus 4.5.

## Areas of improvement

| Area | Change | Status |
|---|---|---|
| TODO redesign | Complete redesign with granular acceptance criteria, anti-shortcut measures, and direct links to every doc | planned |
| Cursor/CJK | Strengthen grapheme-only model, add implementation algorithm detail | planned |
| Terminal emulator | Add VT100 state machine detail, DCS/CSI parsing tables, UTF-8 decoder | planned |
| Viewport wrapping | Add complete wrapping algorithm with tab expansion and fold interaction | planned |
| Session JSON | Add validation rules and migration strategy | planned |
| Render pipeline | Add cell grid algorithm, diff rendering detail | planned |
| Input decoding | Add escape sequence tables and timeout semantics | planned |
| Test specs | Add more edge cases for CJK, terminal, session, and multi-window | planned |
| Spec gap audit | Find and fill thin/empty spec files | planned |
| Log structure | Add improvement proposals and testing ideas | planned |

## Constraints

- Documentation only: no source code changes
- All TODOs unchecked at end (ready for fresh reconstruction)
- No `../` in links
- All files under 200 lines
- Every doc file linked from TODO system
