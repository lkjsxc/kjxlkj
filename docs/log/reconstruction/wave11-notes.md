# Wave 11 Reconstruction Notes

## Completed Features

1. **REQ-ROREG-01**: Read-only registers %, :, / — wired into `read_register()` for put operations and `handle_list_registers()` for `:registers` display
2. **REQ-CHANGELIST-01**: Changelist navigation via g;/g, — new Action variants `ChangelistOlder`/`ChangelistNewer`, pushed on Insert→Normal transition
3. **REQ-RANGERR-01**: Range validation — E493 backwards range, E20 mark not set in `execute_ex_command`
4. **REQ-SESSCUR-01**: Session cursor persistence — `:mksession` writes `call cursor(line, col)`, `:call cursor()` handler added
5. **REQ-BLOCKREN-01**: Visual block info in snapshot — `VisualSelection` struct in core-ui, populated in `snapshot()` for focused window
6. **REQ-MACERR-01**: Macro error halt — `macro_error` flag set by `notify_error()` during macro depth > 0, checked in `play_macro` loop
7. **REQ-VMAGIC-01**: Very-magic `\v` prefix — `strip_vmagic()` extracts regex pattern, `find_pattern()`/`rfind_pattern()` use regex crate
8. **REQ-HLRENDER-01**: hlsearch render — `highlight_ranges` passed through `build_grid` → `render_window` → `render_line_content`, yellow-on-black style applied

## Files Modified

- `editor.rs`: Added `last_ex_command`, `changelist`, `changelist_idx`, `macro_error` fields
- `editor_modes.rs`: Compressed transitions, inject visual selection into snapshot, push changelist on Insert→Normal
- `editor_actions.rs`: Added ChangelistOlder/ChangelistNewer dispatch
- `editing_ops_yank.rs`: Split `read_register` into read_special_register for %, :, /, .
- `ex_dispatch.rs`: Added range validation (E493, E20), `call cursor()` routing, last_ex_command tracking
- `ex_scripting.rs`: Added `append_readonly_regs()` for `:registers` display
- `ex_session_cmds.rs`: Enhanced mksession to save cursor, added `handle_call_cursor`
- `cursor_ops.rs`: Added `push_changelist`, `changelist_older`, `changelist_newer`
- `macros.rs`: Added `macro_error` check in play_macro loop
- `notify.rs`: Set `macro_error = true` when error during macro playback
- `editor_search_marks.rs`: Refactored search to use `find_pattern`/`rfind_pattern` with `\v` support
- `grid.rs`, `grid_window.rs`: Pass and apply `highlight_ranges` for hlsearch rendering
- `snapshot.rs`: Added `VisualSelection` struct, `visual_selection` field to WindowSnapshot
- `dispatch.rs` (core-mode): Added g;/g, → ChangelistOlder/ChangelistNewer
- `action.rs` (core-types): Added ChangelistOlder/ChangelistNewer variants

## New Files

- `wave11_tests.rs`: 10 tests covering all 8 requirements

## Test Count

- Before: 220 tests (Wave 10)
- After: 230 tests (10 new wave11 tests)
