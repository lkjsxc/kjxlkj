# Wave 24 — Log

## Summary

All 8 features implemented, tested (337 total: 5 + 324 + 8), clippy clean, all files ≤ 200 lines.

## Features

| REQ | Feature | Files Modified |
|---|---|---|
| REQ-SCRIPTSOURCE-01 | :source with `finish` early exit | `ex_session_cmds.rs` |
| REQ-ARGLIST-01 | Arglist persistence in sessions | `session.rs`, `session_tests.rs` |
| REQ-FTCOMPL-01 | Filetype-specific completion | `cmdline_completion_ctx.rs` |
| REQ-MAPFILTER-01 | map/filter/extend builtins | `expr_eval.rs` |
| REQ-VIMINFOMERGE-01 | Viminfo merge on quit | `editor_actions.rs` |
| REQ-VISUALK-01 | Visual K keyword lookup | `visual_ops.rs` |
| REQ-SNIPPETCHOICE-01 | Snippet choice nodes | `snippets.rs` |
| REQ-RANGEVARS-01 | Range variable references | `ex_parse_ranges.rs`, `ex_dispatch.rs`, test files |

## Key Implementation Details

- `split_two_args()`: Top-level comma finder respecting `[]`, `{}`, `()` nesting for multi-arg builtins
- Viminfo merge: `save_viminfo` loads existing file, creates merged MarkFile, then writes combined
- Visual K: `visual_keyword_lookup()` extracts selected text, exits visual mode, runs keywordprg
- Snippet choice: `${1|opt1,opt2|}` parsed in `parse_tab_stops_inner`, first option used as default
- Range vars: `RangeContext.vars` field piped through to `eval_expression_with_vars` in `(expr)` addresses
- Variable fallback: bare variable names checked in `vars` HashMap before attempting arithmetic parse

## Files at/near 200-line limit

| File | Lines |
|---|---|
| `expr_eval.rs` | 200 |
| `editor_search_marks.rs` | 200 |
| `grid_window.rs` | 200 |
| `text_objects.rs` | 200 |
| `session.rs` | 199 |
| `editor_modes.rs` | 199 |
| `editor_actions.rs` | 198 |
| `ex_scripting.rs` | 198 |
