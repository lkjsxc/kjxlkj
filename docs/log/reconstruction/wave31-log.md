# Wave 31 — Completion Log

## Features Implemented

1. **REQ-REGEXVV-01**: `\%V` inside-visual-area atom — silently consumed in regex_translate.rs (runtime context no-op in static regex). Added to `\%` match arm.

2. **REQ-EXECUTE-01**: `:execute`/`:exe` — evaluates string expression via eval_expression, runs result as ex command. Added to ex_dispatch.rs match arms.

3. **REQ-VBLOCKPAINT-01**: Visual block cursor-column painting — `is_block_cursor_column()` in grid_window.rs highlights cursor column within block range with subtle dark blue background. Compressed cell grid operations to fit.

4. **REQ-WILDSCROLL-01**: Wildmenu scroll indicators — `<` left arrow when scrolled past start, `>` right arrow when more items beyond visible area. Yellow highlight. Added to render_wildmenu() in grid.rs. Compressed statusline/cmdline rendering.

5. **REQ-STRFUNCS-01**: `toupper()`/`tolower()` string functions — added to try_builtin_function in expr_eval.rs. Uses Rust's to_uppercase()/to_lowercase().

6. **REQ-RETAB-01**: `:retab[!] [new_tabstop]` — converts leading whitespace between tabs and spaces. Respects expandtab/tabstop. Calculates visual column width, rebuilds leading whitespace. Added to ex_buffer_cmds.rs.

7. **REQ-SPELLSUGGEST-01**: `z=` spell suggest — SpellChecker.suggest() generates suggestions via Levenshtein edit distance on good_words + common transformations (single-char deletion, adjacent swap, single-char replacement). spell_suggest() displays up to 10 candidates. edit_distance() helper function.

8. **REQ-MODELINEML-01**: Modeline multiple options — extract_modeline() now returns full options string (no premature colon split). parse_modeline() splits on whitespace AND colons. resolve_option_abbrev() handles 15 common vim abbreviations (ts/sw/et/tw/ai/si/ic/scs/hls/is/nu/rnu/so/ft/fo).

## Files Modified
- `regex_translate.rs` (194→195): \%V atom
- `ex_dispatch.rs` (199→187): :execute/:exe, :retab dispatch, compressed arms
- `expr_eval.rs` (199→200): toupper/tolower
- `ex_buffer_cmds.rs` (123→169): handle_retab
- `spell.rs` (100→145): suggest(), spell_suggest(), edit_distance()
- `ex_jump_noh.rs` (66→68): modeline multi-option parsing, extract_modeline fixed
- `options.rs` (161→178): resolve_option_abbrev()
- `grid_window.rs` (200→190): is_block_cursor_column(), compressed cells
- `grid.rs` (196→116): wildmenu scroll indicators, compressed statusline/cmdline
- `lib.rs` (174→176): mod wave31_tests
- `wave31_tests.rs` (new, ~120): 10 tests

## Test Results
- 403 tests total (5+390+8), all pass
- Zero clippy warnings
- All files ≤200 lines
