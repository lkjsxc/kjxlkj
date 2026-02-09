# Wave 15 Reconstruction Log

## Summary

Wave 15 implemented 8 features: regex-aware search count, session global marks persistence, visual block paste, wildmenu scrolling, expression cmdline prompt, formatprg option, magic mode branch alternation, and snippets registry.

## Features Implemented

| Req ID | Feature | Files Changed |
|--------|---------|---------------|
| REQ-REGEXCOUNT-01 | Regex search count for `\v` patterns | `incsearch.rs` |
| REQ-SESSMARKS-01 | Global marks in session save/load | `ex_session_cmds.rs` |
| REQ-BLOCKPASTE-01 | Visual paste with block column-wise | `visual_ops.rs`, `visual_paste.rs` (new) |
| REQ-WILDSCROLL-01 | Wildmenu scrolling for large completions | `grid.rs` |
| REQ-EXPRCMD-01 | `=` cmdline prompt for expression eval | `ex_dispatch.rs`, `insert_register.rs` |
| REQ-FORMATPRG-01 | `formatprg` option detection in gq | `format_ops.rs` |
| REQ-REGEXBRANCH-01 | `\|` alternation in magic mode search | `editor_search_marks.rs` |
| REQ-SNIPPETS-01 | Snippets registry with expand/list | `snippets.rs` (new) |

## Test Results

- **Total tests**: 265 (252 core-state + 5 core-edit + 8 core-mode)
- **All passing**: Yes
- **Clippy warnings**: 0
- **Files over 200 lines**: 0

## New Files

- `visual_paste.rs` (~74 lines): `visual_paste()` and `visual_block_paste()`
- `snippets.rs` (~75 lines): `SnippetRegistry` with add/get/expand/list/remove/clear
- `wave15_tests.rs` (~105 lines): 8 tests

## File Size Fixes

- `visual_ops.rs`: 201 → 200 (removed blank line)
- `editor_search_marks.rs`: 210 → 197 (condensed helpers)
- `ex_dispatch.rs`: 207 → under 200 (extracted `handle_mark_command` to `ex_session_cmds.rs`)

## Commit

Wave 15: 265 tests
