# Wave 27 — Log

## Features Implemented

1. **REQ-REGEXCOLL-01**: Regex collection `\%[abc]` → `[abc]` and non-capturing `\%(…\)` → `(?:…)` in regex_translate.rs (200 lines)
2. **REQ-TRYCATCH-01**: try/catch/finally/endtry via `try_stack: Vec<bool>` in ex_session_cmds.rs handle_source (199 lines)
3. **REQ-FUZZYPATH-01**: `fuzzy_score()` subsequence matching with boundary bonus; fuzzy fallback in file-path completion. cmdline_completion_ctx.rs (200 lines)
4. **REQ-BITWISEOP-01**: `and()`, `or()`, `xor()` bitwise builtin functions in expr_eval.rs try_builtin_function (199 lines)
5. **REQ-ALTMARK-01**: `set_alternate()`/`get_alternate()` in marks.rs (187 lines); wired into next_buffer/prev_buffer in ex_buffer_cmds.rs (123 lines)
6. **REQ-MACRODOT-01**: `DotRepeat` action replays `last_macro` via `play_macro(r, 1)` in editor_actions.rs (198 lines)
7. **REQ-SNIPPETTX-01**: `${N/regex/replace/flags}` transformation parsing in snippets.rs parse_tab_stops_inner (192 lines)
8. **REQ-RANGEFUNC-01**: User function calls in expression addresses via `call_fn` callback on RangeContext; `find_matching_paren()` for nested paren support; closure in ex_dispatch.rs evaluates function body return statements. ex_parse_ranges.rs (143 lines), ex_dispatch.rs (193 lines)

## Files Modified

- regex_translate.rs (200)
- ex_session_cmds.rs (199)
- cmdline_completion_ctx.rs (200)
- expr_eval.rs (199)
- marks.rs (187)
- ex_buffer_cmds.rs (123)
- editor_actions.rs (198)
- snippets.rs (192)
- ex_parse_ranges.rs (143)
- ex_dispatch.rs (193)
- lib.rs (169)

## Files Created

- wave27_tests.rs (108 lines, 9 tests)

## Test Results

- Total: 362 tests (5 + 349 + 8)
- All passing, zero clippy warnings
- All files ≤ 200 lines

## Notes

- RangeContext gained `call_fn: Option<&dyn Fn(&str) -> Option<String>>` field — all callers updated with `call_fn: None` or wired closure
- `find_matching_paren()` added for nested-paren expression addresses like `(GetLine())`
- `fuzzy_score` and `parse_tab_stops` made pub for testing
