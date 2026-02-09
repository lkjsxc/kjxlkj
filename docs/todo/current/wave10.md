# Wave 10 — Yank Marks, Macro Depth, Count Ranges, Expression Register, Last-Inserted Register, Custom Command Completion, formatoptions, Config Sections

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Scope

Auto-set [ ] marks on yank, add recursive macro depth limit,
support count-based ranges, expression register stub,
last-inserted register, user-command completion, formatoptions
option, and config section headers.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-YANKMARKS-01 | [ ] marks set on yank (not just change/delete) | `/docs/spec/editing/marks/automatic-marks.md` | `[x]` |
| REQ-MACRODEPTH-01 | Recursive macro playback depth limit | `/docs/spec/editing/macros/recursive-macros.md` | `[x]` |
| REQ-COUNTRANGE-01 | :{count} interpreted as line range | `/docs/spec/commands/ranges/ranges.md` | `[x]` |
| REQ-EXPREG-01 | = expression register with basic arithmetic | `/docs/spec/editing/registers/expression-register.md` | `[x]` |
| REQ-LASTINS-01 | . read-only register returns last inserted text | `/docs/spec/editing/registers/special-registers.md` | `[x]` |
| REQ-USERCMDCOMPL-01 | Tab-complete user-defined command names | `/docs/spec/commands/cmdline/completion.md` | `[x]` |
| REQ-FMTOPTS-01 | formatoptions option for gq behavior | `/docs/spec/editing/text-manipulation/README.md` | `[x]` |
| REQ-CFGSECT-01 | Config file supports [section] headers | `/docs/spec/features/config/README.md` | `[x]` |

## Exit Criteria

- `cargo build` clean
- `cargo test` passes all tests
- `cargo clippy` zero warnings
- All files ≤ 200 lines
- LIMITATIONS updated
