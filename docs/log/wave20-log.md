# Wave 20 Log

Back: [/docs/log/README.md](/docs/log/README.md)

## Summary

Wave 20 implements 8 features: function parameter binding, :sort command,
popup menu rendering with position/scroll, list literals and len(), local
marks in sessions, formatexpr option, :s///c cursor positioning, and
search highlights in operator-pending mode.

## Changes

### New Files
- `ex_sort.rs` (54 lines): `:sort` command with !/r reverse, i case-insensitive, n numeric, u unique flags.
- `wave20_tests.rs` (122 lines): 8 tests covering all wave 20 features.

### Modified Files
- `ex_scripting.rs` (192 lines): `handle_call_function()` parses args from parentheses, binds to `a:param` in options store.
- `ex_dispatch.rs` (189 lines): Added `:sort` match arm; compressed delete/yank/substitute arms.
- `editor_modes.rs` (199 lines): PopupMenu creation with row/col/max_visible/scroll_offset; compute_hlsearch also activates in OperatorPending mode.
- `grid.rs` (196 lines): Added render_popup_menu() with dark bg and blue selected styles.
- `snapshot.rs` (172 lines): PopupMenu struct now has row, col, max_visible, scroll_offset fields.
- `expr_eval.rs` (193 lines): Added `[...]` list literal pass-through; added `len()` builtin for list counting.
- `session.rs` (196 lines): SessionFile has `local_marks: Vec<(char, usize, usize)>`; serialize/deserialize localmark lines.
- `ex_session_cmds.rs` (180 lines): mksession saves local marks per buffer; source restores them.
- `format_ops.rs` (191 lines): formatexpr check calls user function before formatprg.
- `ex_substitute_confirm.rs` (128 lines): sub_confirm_advance moves cursor to target line.
- `ex_substitute.rs` (94 lines): Cursor positioned at first line of range on entering confirm mode.
- `session_tests.rs` (45 lines): Added local_marks field to SessionFile initializer.
- `lib.rs` (157 lines): Added mod ex_sort, mod wave20_tests.

## Test Results

- 305 tests total (5 + 292 + 8): all pass
- 0 clippy warnings
- All files ≤ 200 lines
