# Wave 20

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Overview

Wave 20 — function parameters and local variables, visual sort/filter, popup menu rendering, list literals in expressions, mark persistence across sessions, formatexpr option, interactive :s///c confirmation, search highlight in operator-pending.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-FUNCPARAM-01 | `:call FuncName(arg1,arg2)` passes arguments to function parameters as local `a:` variables | `/docs/spec/scripting/user-functions.md` | `[x]` |
| REQ-VSORT-01 | `:'<,'>sort` sorts selected lines; `:'<,'>sort!` reverse sort; `:'<,'>sort i` case-insensitive | `/docs/spec/commands/ranges/ranges.md` | `[x]` |
| REQ-POPUP-01 | Popup menu struct rendered in snapshot with positioned items, selection highlight, and scroll offset | `/docs/spec/ui/popup-menu.md` | `[x]` |
| REQ-LISTLIT-01 | Expression evaluator supports `[1,2,3]` list literals and `len()` function on lists | `/docs/spec/scripting/user-functions.md` | `[x]` |
| REQ-MARKPERSIST-01 | Local marks (a-z) for each buffer saved and restored in session files | `/docs/spec/features/session/README.md` | `[x]` |
| REQ-FMTEXPR-01 | `formatexpr` option: if set and non-empty, `gq` calls user function instead of internal wrap | `/docs/spec/editing/text-manipulation/README.md` | `[x]` |
| REQ-SUBCONFIRM-01 | :s///c cursor positioning: move cursor to current match line during confirmation | `/docs/spec/commands/buffer/substitute.md` | `[x]` |
| REQ-OPHIGHLIGHT-01 | During operator-pending mode, search pattern matches are highlighted in render pipeline | `/docs/spec/editing/search/README.md` | `[x]` |

## Exit Criteria

- All requirements status `[x]`.
- `cargo build` clean.
- `cargo clippy` zero warnings.
- `cargo test` all pass with new wave tests.
- All source files ≤ 200 lines.
- LIMITATIONS.md updated.
