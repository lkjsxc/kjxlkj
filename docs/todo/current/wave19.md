# Wave 19

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Overview

Wave 19 — function invocation via :call, visual g?/~ operators, ternary expression operator, :delmarks ranges, snippet tab advance, backward search offsets, session window sizes, formatprg external pipe.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-FUNCALL-01 | `:call FuncName(args)` invokes user-defined function body lines | `/docs/spec/scripting/user-functions.md` | `[x]` |
| REQ-VROT13-01 | Visual `g?` applies ROT13 and `~` toggles case on selection | `/docs/spec/modes/visual.md` | `[x]` |
| REQ-TERNARY-01 | Expression evaluator supports `cond ? then : else` ternary | `/docs/spec/scripting/user-functions.md` | `[x]` |
| REQ-DELRANGE-01 | `:delmarks a-d` deletes marks in specified range | `/docs/spec/editing/marks/README.md` | `[x]` |
| REQ-SNIPTAB-01 | Tab key in insert mode advances active snippet session to next stop | `/docs/spec/features/editing/README.md` | `[x]` |
| REQ-BSEARCHOFF-01 | Backward search `?pattern?e` applies search offset | `/docs/spec/editing/search/README.md` | `[x]` |
| REQ-SESSWIN-01 | `:mksession` saves and restores window split sizes | `/docs/spec/features/session/README.md` | `[x]` |
| REQ-FMTPRG-01 | `gq` with `formatprg` set pipes through external command | `/docs/spec/editing/text-manipulation/README.md` | `[x]` |

## Exit Criteria

- All requirements status `[x]`.
- `cargo build` clean.
- `cargo clippy` zero warnings.
- `cargo test` all pass with new wave tests.
- All source files ≤ 200 lines.
- LIMITATIONS.md updated.
