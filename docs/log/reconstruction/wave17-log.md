# Wave 17 Log

Back: [/docs/log/README.md](/docs/log/README.md)

## Summary

Wave 17 implemented 8 features: visual J/= operators, :marks listing, session history, expr built-in functions, substitute confirmation, multi-line search, macro edit via yank, filetype detection.

## Changes

### New files
- `ex_substitute_confirm.rs` — Substitute confirm handler and shared helper fns
- `wave17_tests.rs` — 8 tests for Wave 17 features

### Modified files
- `visual_ops.rs` — Added J and = dispatch in visual mode
- `visual_replace.rs` — Added `visual_join()` and `visual_reindent()` methods
- `format_ops.rs` — Added `reindent_lines()` method
- `ex_scripting.rs` — Improved `:marks` to show line/col positions
- `ex_session_cmds.rs` — Saves cmdline history and search pattern; restores on source
- `expr_eval.rs` — `try_builtin_function()` for strlen/line/col
- `ex_parse_substitute.rs` — Added `confirm` flag parsing for 'c'
- `ex_substitute.rs` — Refactored to use shared fns from confirm module
- `editor_mode_dispatch.rs` — Sub-confirm key intercept
- `editor.rs` — Added `sub_confirm` field, `SubConfirmState` struct, filetype on open
- `editing_ops_yank.rs` — `sync_register_to_macro` on named register store
- `config_loader.rs` — `detect_filetype()` for 30+ extensions
- `options.rs` — Added `filetype` default option
- `search_engine.rs` — Multi-line pattern support in `build_all_matches`
- `lib.rs` — Added `ex_substitute_confirm`, `wave17_tests` modules

## Test Count

281 total (268 core-state + 5 core-edit + 8 core-mode)

## Verification

- `cargo build` clean
- `cargo clippy` zero warnings
- `cargo test` 281 pass
- All files ≤ 200 lines
