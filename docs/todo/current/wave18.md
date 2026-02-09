# Wave 18

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Overview

Wave 18 — visual u/U case operators, expression comparison operators, :delmarks! clear all, tab-stop cursor navigation, search offsets, function definitions, lookbehind docs, popup completion menu stub.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-VCASE-01 | Visual u lowercases and U uppercases selection | `/docs/spec/modes/visual.md` | `[x]` |
| REQ-EXPRCMP-01 | Expression evaluator supports ==, !=, <, >, <=, >= comparisons | `/docs/spec/scripting/user-functions.md` | `[x]` |
| REQ-DELMARKSALL-01 | `:delmarks!` clears all local marks for current buffer | `/docs/spec/editing/marks/README.md` | `[x]` |
| REQ-SNIPPAV-01 | Snippet expansion places cursor at first tab-stop | `/docs/spec/features/editing/README.md` | `[x]` |
| REQ-SEARCHOFF-01 | Search offset: /pattern/e, /pattern/s+N, /pattern/+N | `/docs/spec/editing/search/README.md` | `[x]` |
| REQ-FUNCDEF-01 | `function!` / `endfunction` defines user Vimscript functions | `/docs/spec/scripting/user-functions.md` | `[x]` |
| REQ-LBHIND-01 | Document variable-length lookbehind limitation for regex | `/docs/spec/editing/regex/README.md` | `[x]` |
| REQ-POPUPMENU-01 | Cmdline completion shows popup-style candidate list in snapshot | `/docs/spec/commands/cmdline/completion.md` | `[x]` |

## Exit Criteria

- All requirements status `[x]`.
- `cargo build` clean.
- `cargo clippy` zero warnings.
- `cargo test` all pass with new wave tests.
- All source files ≤ 200 lines.
- LIMITATIONS.md updated.
