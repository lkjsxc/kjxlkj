# Wave 18 — Log

Back: [/docs/log/README.md](/docs/log/README.md)

## Summary

Wave 18 implements 8 features: visual u/U case operators, expression comparison operators, `:delmarks!`, snippet tab-stop cursor navigation, search offsets, user-defined function registry, variable-length lookbehind documentation, and popup completion menu snapshot integration.

## Changes

### REQ-VCASE-01 — Visual u/U case operators

- `visual_ops.rs`: Added `u` → `Operator::Lowercase` and `U` → `Operator::Uppercase` to `char_to_operator()`.
- Charwise and linewise paths already handle Lowercase/Uppercase via `case_range()`, `lowercase_lines()`, `uppercase_lines()`.

### REQ-EXPRCMP-01 — Expression comparison operators

- `expr_eval.rs`: Added `try_comparison()` and `find_comparison_op()` for ==, !=, <, >, <=, >=.
- Returns "1" for true, "0" for false. Numeric comparison first, string fallback.
- File compressed from 246 → 168 lines.

### REQ-DELMARKSALL-01 — :delmarks! clear all

- `ex_dispatch.rs`: Added `"delmarks!"` match arm calling `self.marks.clear_buffer(bid)`.

### REQ-SNIPPAV-01 — Snippet tab-stop cursor

- `snippets.rs`: Added `expand_at(trigger, base_line, base_col)` → `(String, SnippetSession)`.
- `SnippetSession`: Added `current_offset()` and `advance()` methods.

### REQ-SEARCHOFF-01 — Search offsets

- `snapshot.rs` (core-ui): Added `SearchOffset` enum (None, Lines, End, Start) and `offset` field to `SearchState`.
- `search_types.rs`: Added `parse_search_with_offset()`, re-exports `SearchOffset` from UI.
- `editor_search_marks.rs`: Added `apply_search_offset()`, wired into `search_next()`/`search_prev()`.
- `ex_dispatch.rs`: Search cmdline now calls `parse_search_with_offset()`.

### REQ-FUNCDEF-01 — User-defined functions

- New `user_functions.rs` (87 lines): `UserFunction`, `FunctionRegistry`, `parse_function_header()`.
- `editor.rs`: Added `functions: FunctionRegistry`, `function_body_acc: Option<FunctionBodyAcc>`, `FunctionBodyAcc` struct.
- `ex_dispatch.rs`: `function!`/`endfunction` body accumulation and definition.

### REQ-LBHIND-01 — Lookbehind limitation docs

- `docs/spec/editing/regex/lookaround.md`: Added "Implementation limitation" section.

### REQ-POPUPMENU-01 — Popup completion menu

- `snapshot.rs` (core-ui): Added `PopupMenu` struct (items, selected).
- `EditorSnapshot`: Added `popup_menu: Option<PopupMenu>`.
- `editor_modes.rs`: Snapshot builder populates `popup_menu` from cmdline completion candidates.

## Tests

- `wave18_tests.rs`: 8 new tests (visual_lowercase_selection, visual_uppercase_selection, expr_comparison_operators, delmarks_bang_clears_local, snippet_expand_at_first_tabstop, search_offset_parsing, function_definition_via_ex, popup_menu_from_completion).
- Total: 288 tests (280 prior + 8 new), all passing.

## Metrics

- Files modified: 11 source, 2 docs
- Files created: 2 (user_functions.rs, wave18_tests.rs)
- Max file size: 200 lines (editor_search_marks.rs)
- Clippy warnings: 0
