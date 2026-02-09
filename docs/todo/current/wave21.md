# Wave 21

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Overview

Wave 21 — function local variables and return, tab pages in sessions,
popup menu keyboard navigation, dict literals and type(), numbered marks,
equalprg option, snippet placeholder editing, backwards range confirmation.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-LOCALVAR-01 | Functions support `l:` local variables via `:let l:var = expr` and `:return expr` stops execution and returns value | `/docs/spec/scripting/user-functions.md` | `[x]` |
| REQ-TABSESS-01 | Tab pages serialized in session with `tab N` entries; :source restores tab layout | `/docs/spec/features/session/README.md` | `[x]` |
| REQ-POPUPNAV-01 | Ctrl-N / Ctrl-P navigate popup menu items during command-line completion | `/docs/spec/commands/cmdline/completion.md` | `[x]` |
| REQ-DICTLIT-01 | Expression evaluator supports `{"k":"v"}` dict literals and `type()` returns "0"=number, "1"=string, "3"=list, "4"=dict | `/docs/spec/scripting/user-functions.md` | `[x]` |
| REQ-NUMMARKS-01 | Numbered marks 0-9 auto-set: mark 0 at last position before jump, marks 1-9 rotate previous jump positions | `/docs/spec/editing/marks/README.md` | `[x]` |
| REQ-EQUALPRG-01 | `equalprg` option: if set, `=` operator pipes range through external program instead of reindent | `/docs/spec/editing/text-manipulation/README.md` | `[x]` |
| REQ-SNIPPLACEHOLDER-01 | Snippet tab-stops with default text `${1:default}` are expanded and Tab navigates between them selecting the placeholder | `/docs/spec/features/editing/README.md` | `[x]` |
| REQ-BACKRANGE-01 | When backwards range detected (start > end), notify user and auto-swap to forward range for execution | `/docs/spec/commands/ranges/ranges.md` | `[x]` |

## Exit Criteria

- All requirements status `[x]`.
- `cargo build` clean.
- `cargo clippy` zero warnings.
- `cargo test` all pass with new wave tests.
- All source files ≤ 200 lines.
- LIMITATIONS.md updated.
