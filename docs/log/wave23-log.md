# Wave 23 Log

## Features Implemented

1. **REQ-AUTOLOAD-01** — Autoload function resolution via `#` separator in handle_call_function
2. **REQ-TABBUFASSOC-01** — Tab-specific buffer associations (tab_buffers/tabbuf) in session
3. **REQ-CTXCOMPL-01** — Context-aware completion for mark, register, help commands
4. **REQ-DICTITER-01** — `keys()` and `values()` builtin functions in expr_eval
5. **REQ-VIMINFOAUTOSAVE-01** — Auto-save viminfo on quit, auto-load on startup
6. **REQ-KEYWORDCOUNT-01** — K command with count passed as section argument
7. **REQ-SNIPPETNEST-01** — Nested snippet placeholders via recursive parse_tab_stops_inner
8. **REQ-RANGEEXPRFUNC-01** — Function calls in arithmetic within expression addresses

## Test Results

- 329 tests total: 5 + 316 + 8
- 0 failures
- 0 clippy warnings

## Modified Files (line counts)

| File | Lines |
|---|---|
| expr_eval.rs | 182 |
| ex_scripting.rs | 198 |
| session.rs | 199 |
| session_tests.rs | 49 |
| editor_modes.rs | 199 |
| marks.rs | 192 |
| snippets.rs | 166 |
| ex_parse_ranges.rs | 122 |
| editor_actions.rs | 196 |
| ex_dispatch.rs | 193 |
| cmdline_completion_ctx.rs | 144 |
| editor.rs | 192 |
| dispatch.rs (core-mode) | 196 |
| action.rs (core-types) | 180 |
| lib.rs | 162 |
| wave23_tests.rs | 127 |
