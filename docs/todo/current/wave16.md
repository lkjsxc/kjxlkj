# Wave 16

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Overview

Wave 16 — lookahead/lookbehind regex, visual block replace, snippet tab-stops, session option persistence, expression variables, cmdline history prefix filtering, visual gv reselect, visual search (* and #).

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-LOOKAROUND-01 | Translate Vim lookahead/lookbehind `\@=`, `\@!`, `\@<=`, `\@<!` to Rust regex lookaround | `/docs/spec/editing/regex/lookaround.md` | `[x]` |
| REQ-BLOCKR-01 | Visual block `r{char}` — replace each character in block with {char} | `/docs/spec/modes/visual.md` | `[x]` |
| REQ-SNIPTAB-01 | Snippet tab-stop navigation: expand with cursor at $1, Tab advances to $2, etc. | `/docs/spec/features/editing/README.md` | `[x]` |
| REQ-SESSOPTS-01 | Session saves/restores editor options (:set values) | `/docs/spec/features/session/README.md` | `[x]` |
| REQ-EXPRVARS-01 | Expression evaluator supports `g:var` variable references via OptionStore | `/docs/spec/scripting/user-functions.md` | `[x]` |
| REQ-HISTFILT-01 | Cmdline history Up/Down filters by typed prefix | `/docs/spec/commands/cmdline/cmdline-history.md` | `[x]` |
| REQ-GVRESEL-01 | `gv` reselects the last visual selection (range and sub-mode) | `/docs/spec/modes/visual.md` | `[x]` |
| REQ-VSTAR-01 | `*` and `#` in visual mode search for selected text | `/docs/spec/modes/visual.md` | `[x]` |

## Exit Criteria

- All requirements status `[x]`.
- `cargo build` clean.
- `cargo clippy` zero warnings.
- `cargo test` all pass with new wave tests.
- All source files ≤ 200 lines.
- LIMITATIONS.md updated.
