# Wave 32 — Completion Log

## Features Implemented

1. **REQ-REGEXLC-01**: `\%l`, `\%c`, `\%Nl`, `\%Nc` line/column constraint atoms — silently consumed in regex_translate.rs. Digit sequences consumed before trailing `l`/`c`/`v`. Added digit-prefixed variant handling in `\%` match arm.

2. **REQ-NORMAL-01**: `:normal`/`:norm`/`:normal!`/`:norm!` — executes key string as if typed in normal mode. Saves and restores mode. Added `handle_normal_command()` in ex_jump_noh.rs with dispatch in ex_dispatch.rs.

3. **REQ-VBLOCKDOLLAR-01**: Visual block `$` — sets `block_dollar` flag on EditorState for end-of-line selection per line in block mode. Added field to EditorState, intercepts `$` in visual_ops.rs for VisualKind::Block.

4. **REQ-FNAMEONLY-01**: Wildmenu filename-only display — strips directory paths from completion items using `rsplit('/')` in render_wildmenu() in grid.rs. Shows only the filename portion.

5. **REQ-TRESCAPE-01**: `tr(string, from, to)` transliteration and `escape(string, chars)` backslash-insertion functions. Added `expr_tr()` and `expr_escape()` in expr_string_funcs.rs, dispatched from try_builtin_function in expr_eval.rs.

6. **REQ-ALIGNMENT-01**: `:center [width]`, `:left [indent]`, `:right [width]` — text alignment commands. Added `handle_alignment()` in ex_buffer_cmds.rs with dispatch in ex_dispatch.rs. Respects textwidth option for default width.

7. **REQ-DICTSPELL-01**: `SpellChecker.load_dictionary()` — loads word files with hunspell .dic format support (affix stripping). `try_load_spell_dictionary()` auto-searches XDG config and cwd `spell/` directories on toggle_spell(true).

8. **REQ-SECUREEXRC-01**: Secure exrc sandboxing — `secure` option enables `handle_source_secure()` which skips dangerous commands (!, autocmd, write, wq, source, call system()) when sourcing local exrc files.

## Files Modified
- `regex_translate.rs` (195→199): \%l/\%c/\%Nl/\%Nc atoms, consume_digits helper
- `ex_dispatch.rs` (187→191): :normal/:norm dispatch, :center/:left/:right dispatch
- `ex_jump_noh.rs` (68→85): handle_normal_command
- `visual_ops.rs` (197→199): block $ handling, block_dollar reset
- `expr_eval.rs` (200→200): tr()/escape() dispatch, merged toupper/tolower
- `expr_string_funcs.rs` (44→84): expr_tr(), expr_escape()
- `ex_buffer_cmds.rs` (169→140): handle_alignment, compressed delete/yank_range
- `spell.rs` (145→127): load_dictionary, try_load_spell_dictionary, compressed methods
- `config_loader.rs` (198→135): handle_source_secure, load_local_exrc secure mode, compressed
- `editor.rs` (196→199): block_dollar field
- `grid.rs` (116→119): filename-only wildmenu display
- `lib.rs` (176→178): mod wave32_tests
- `wave32_tests.rs` (new, ~86): 10 tests

## Test Results
- 413 tests total (5+400+8), all pass
- Zero clippy warnings
- All files ≤200 lines
