# Wave 19 Log

Back: [/docs/log/README.md](/docs/log/README.md)

## Summary

Wave 19 implements 8 features: function invocation via `:call`, visual `g?` ROT13 and `~` toggle case operators, ternary expression evaluation, `:delmarks` range notation, Tab-key snippet session advancement, backward search `?pattern?e` offsets, session layout window weights, and `formatprg` external command piping.

## Changes

### REQ-FUNCALL-01: `:call FuncName(args)`
- Added `handle_call_function()` to `ex_scripting.rs`
- Parses function name from `call X(...)`, looks up in FunctionRegistry
- Executes body lines sequentially via `execute_ex_command()`
- Added match arm in `ex_dispatch.rs` (after `call cursor(` special case)

### REQ-VROT13-01: Visual `g?` ROT13 and `~` toggle case
- Added `ToggleCase` and `Rot13` variants to `Operator` enum in `mode.rs`
- `toggle_case_range()`, `rot13_range()`, `replace_range_text()` in `cursor_ops_findchar.rs`
- `toggle_case_lines()`, `rot13_lines()` via generic `transform_lines()` with closures
- `rot13_char()` free function: a-m/A-M → +13, n-z/N-Z → −13
- Added operator arms to `editing_ops_ranges.rs` (charwise and linewise)
- `visual_g_pending` flag in `editor.rs`; `g` prefix and `~` dispatch in `visual_ops.rs`

### REQ-TERNARY-01: Ternary `cond ? then : else`
- Added `try_ternary()` and `find_top_level_char()` to `expr_eval.rs`
- Top-level `?` found outside parens/quotes; splits into condition, then, else
- Condition evaluated; truthy (non-zero, non-empty) selects then-branch

### REQ-DELRANGE-01: `:delmarks a-d` range notation
- Updated `handle_delmarks()` in `ex_scripting.rs`
- Detects 3-char pattern (`char - char`) and iterates through inclusive char range

### REQ-SNIPTAB-01: Tab advances snippet session
- Added `snippet_session: Option<SnippetSession>` to `EditorState`
- Insert-mode Tab in `editor_mode_dispatch.rs` checks for active session
- `advance_snippet()` in `editing_ops_insert.rs` moves cursor to next tab-stop
- Borrow-checker safe: extracts session data then accesses buffer

### REQ-BSEARCHOFF-01: `?pattern?e` backward search offset
- Refactored `parse_search_with_offset()` in `search_types.rs`
- `parse_search_with_offset_sep()` accepts separator char (tries `/` then `?`)
- Only returns offset if meaningful parse result

### REQ-SESSWIN-01: Session window sizes (weights)
- `SessionLayout::Hsplit` and `Vsplit` now carry `Vec<f64>` weights
- `serialize()` writes weights as comma-separated floats
- `deserialize()` parses `layout hsplit 0.6000,0.4000` format
- `parse_weights()` helper function

### REQ-FMTPRG-01: formatprg external pipe
- `format_via_external()` in `format_ops.rs`
- `std::process::Command` with stdin pipe and stdout capture
- Replaces line range with formatted output on success
- `E282` error on spawn failure; exit-status error on non-zero

## Metrics

- Tests: 297 total (5 + 284 + 8), all pass
- Clippy: 0 warnings
- Source files: all ≤ 200 lines
- Wave 19 tests: 9 (call_user_function, visual_rot13, visual_toggle_case, ternary_expression, delmarks_range, snippet_tab_advance, backward_search_offset, session_layout_weights, formatprg_pipe_error)

## Files Modified

- `mode.rs`: Added `ToggleCase`, `Rot13` to Operator
- `cursor_ops_findchar.rs`: ROT13/toggle case operations, transform_lines
- `editing_ops_ranges.rs`: ToggleCase/Rot13 operator arms
- `visual_ops.rs`: g-prefix handling, ~ dispatch
- `editor.rs`: visual_g_pending, snippet_session fields
- `expr_eval.rs`: try_ternary, find_top_level_char
- `ex_scripting.rs`: handle_call_function, compressed delmarks
- `editor_mode_dispatch.rs`: Tab snippet intercept
- `editing_ops_insert.rs`: advance_snippet
- `search_types.rs`: parse_search_with_offset_sep
- `session.rs`: weights in SessionLayout, parse_weights
- `ex_session_cmds.rs`: serialize_layout weights
- `format_ops.rs`: format_via_external
- `ex_dispatch.rs`: :call match arm
- `wave12_tests.rs`–`wave16_tests.rs`: Fixed pre-existing clippy warnings

## Files Created

- `wave19_tests.rs`: 9 tests
- `wave19.md`: Requirements document
- `wave19-log.md`: This file
