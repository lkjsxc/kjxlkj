# Wave 33 — Log

## Summary

Implemented 8 features: \%# cursor atom, :for/:endfor loops, visual block I/A count, popup menu truncation, printf/split/join functions, :move/:copy range commands, multi-language spell checking, per-directory trust store.

## Changes

### regex_translate.rs (199→199)
- Added `Some('#')` arm to runtime constraint atom match for \%# cursor position

### editor.rs (199→192)
- Added `ForLoopAcc` struct with var, list_expr, body fields
- Added `for_loop_acc: Option<ForLoopAcc>` field on EditorState
- Removed doc comments on struct fields for compression

### ex_dispatch.rs (191→198)
- Added :for/:endfor accumulation logic (checks for_loop_acc state)
- Added :move/:copy/:trust command dispatch
- Merged echo/echon/echomsg/echohl/echoerr into dispatch_echo() call

### ex_jump_noh.rs (85→115)
- Added `dispatch_echo()` — consolidated echo variant handler using strip_prefix
- Added `handle_for_start()` — parses `:for var in list` syntax
- Added `execute_for_loop()` — iterates list items, sets variable, runs body commands

### visual_block_insert.rs (55→60)
- Added count-based repetition: `op_count.max(1)` → `text.repeat(count)`

### grid.rs (119→121)
- Added popup menu max-width truncation based on `grid.width()`
- Items exceeding available width truncated with `…` suffix

### expr_string_funcs.rs (84→143)
- Added `expr_printf()` — %s/%d format specifiers
- Added `expr_split()` — split string by pattern into list
- Added `expr_join()` — join list items with separator

### expr_eval.rs (200→200)
- Added printf/split/join dispatch via merged tr|escape|printf|split|join arm
- Fixed dict access bug: `["key"]` check now requires `b > 0` to avoid matching list literals
- Merged match/substitute, keys/values, map/filter arms for compression

### ex_buffer_cmds.rs (140→191)
- Added `handle_move_range()` — moves lines to after dest with adjusted index after deletion
- Added `handle_copy_range()` — copies lines to after dest

### spell.rs (127→135)
- Multi-language support via `spelllang` option (comma-separated)
- Added `load_spell_for_lang()` per language
- Fixed borrow issue by cloning `self.spell.lang` before iterating

### config_loader.rs (135→166)
- Added `trust_store_path()` → XDG config `kjxlkj/trust`
- Added `is_directory_trusted()` — reads trust file, checks if cwd listed
- Added `handle_trust_directory()` — appends cwd to trust file
- Added `:trust` command dispatch

### wave33_tests.rs (new, ~80 lines)
- 10 tests covering all 8 features

## Bugs Fixed

- Dict access `["key"]` matching list literals `["a","b","c"]` when b=0 — added b>0 guard
- Borrow issue in spell.rs — self.spell.lang cloned before mutable iteration
- Move range dest 1-indexed → 0-indexed conversion with saturating_sub(1)

## Metrics

- Tests: 423 (5+410+8), all passing
- New clippy warnings: 0
- Files at 200: text_objects.rs, expr_eval.rs, cmdline_completion_ctx.rs
- Max file: 200 lines (constraint satisfied)
