# Wave 17

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Overview

Wave 17 — visual J/= operators, :marks command, session search history, expression built-in functions, confirmation prompt for substitute, multi-line search atoms, macro edit via registers, ftplugin type detection.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-VJOIN-01 | Visual J joins selected lines; visual = reindent selected lines | `/docs/spec/modes/visual.md` | `[x]` |
| REQ-MARKSLIST-01 | `:marks` command lists all defined marks with positions | `/docs/spec/editing/marks/README.md` | `[x]` |
| REQ-SESSHIST-01 | Session saves/loads search history (:mksession/:source) | `/docs/spec/features/session/README.md` | `[x]` |
| REQ-EXPRFN-01 | Expression evaluator supports built-in functions: strlen, line, col | `/docs/spec/scripting/user-functions.md` | `[x]` |
| REQ-SUBCONF-01 | `:s/pat/rep/c` shows confirmation prompt (y/n/a/q/l) | `/docs/spec/commands/ranges/ranges.md` | `[x]` |
| REQ-MLINE-01 | Search supports `\n` to match line boundaries for multi-line patterns | `/docs/spec/editing/regex/multiline-patterns.md` | `[x]` |
| REQ-MACROEDIT-01 | Macro registers editable via put/yank: `"ap` pastes macro, `"ayy` copies line to macro | `/docs/spec/editing/macros/README.md` | `[x]` |
| REQ-FTDETECT-01 | Filetype detection from file extension sets `filetype` option | `/docs/spec/features/config/README.md` | `[x]` |

## Exit Criteria

- All requirements status `[x]`.
- `cargo build` clean.
- `cargo clippy` zero warnings.
- `cargo test` all pass with new wave tests.
- All source files ≤ 200 lines.
- LIMITATIONS.md updated.
