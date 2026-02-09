# Wave 25 — Log

## Summary

8 features implemented, 8 tests added, all 345 tests pass, zero clippy warnings.

## Features

1. **Atomic groups / non-greedy** (`regex_translate.rs` 184→184): `\@>` → `(?:` non-capturing group approximation; `\{-}` → `*?` non-greedy quantifier.
2. **if/else/endif** (`ex_session_cmds.rs` 183→191): Conditional execution in `:source` scripts via `skip_stack` Vec<bool>. Supports if/elseif/else/endif nesting.
3. **Session auto-save** (`editor_actions.rs` +6, `editor.rs` +2): `autosaveinterval` option; `tick_autosave()` called on every action; triggers `handle_mksession(None)` when counter reaches interval.
4. **Visual filter** (`ex_dispatch.rs` +1, `ex_sort.rs` +30): `:{range}!{cmd}` filters range through external shell via `handle_filter_shell()`. Uses sh -c with piped stdin/stdout.
5. **Path ~ expansion** (`cmdline_completion_ctx.rs` +7): `~` at start of path input expands to `$HOME` before directory listing.
6. **match/substitute funcs** (`expr_eval.rs` +4 dispatch, `expr_string_funcs.rs` new 47 lines): `match(str, pat)` returns byte index or -1; `substitute(str, pat, rep, flags)` with `g` flag for global replace. Uses regex crate.
7. **Macro stepping** (`editor_actions.rs` +16, `editor.rs` +2): `:debug @{reg}` via `handle_debug_macro()` queues macro keys; `macro_step_next()` executes one key at a time showing remaining count.
8. **ftplugin** (`config_loader.rs` +13, `editor.rs` +1): `load_ftplugin(ft)` searches XDG config dir and local `ftplugin/{ft}.vim` on filetype detection in `open_file`.

## Files Modified

- `regex_translate.rs` (184 lines)
- `ex_session_cmds.rs` (191 lines)
- `editor_actions.rs` (198 lines)
- `editor.rs` (199 lines)
- `ex_dispatch.rs` (196 lines)
- `ex_sort.rs` (~84 lines)
- `cmdline_completion_ctx.rs` (~155 lines)
- `expr_eval.rs` (200 lines)
- `config_loader.rs` (~126 lines)
- `lib.rs` (~167 lines)

## New Files

- `expr_string_funcs.rs` (47 lines) — match/substitute expression functions
- `wave25_tests.rs` (~116 lines) — 8 tests

## Test Count

5 + 332 + 8 = 345 total tests (all passing)
