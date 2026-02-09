# Wave 22 Log

## Features Implemented

1. **REQ-SCRIPTLOCAL-01** — `s:` variable namespace + `function("name")` references in expr_eval.rs
2. **REQ-TABWINLAYOUT-01** — Tab page window layouts in session serialization (tablayout entries)
3. **REQ-POPUPINSERT-01** — Enter on popup inserts selected candidate, stays in Command mode
4. **REQ-DICTACCESS-01** — `dict["key"]` access + `has_key(dict, "key")` builtin
5. **REQ-VIMINFOMARKS-01** — Viminfo global mark persistence (serialize_viminfo/load_viminfo)
6. **REQ-KEYWORDPRG-01** — K command keyword lookup via keywordprg option
7. **REQ-SNIPPETMIRROR-01** — Snippet mirror tab-stops via defaults HashMap
8. **REQ-RANGEEXPR-01** — Expression addresses `(expr)` in range parsing

## Test Results

- 321 tests total: 5 + 308 + 8
- 0 failures
- 0 clippy warnings

## Modified Files (line counts)

| File | Lines |
|---|---|
| expr_eval.rs | 150 |
| ex_scripting.rs | 200 |
| session.rs | 198 |
| session_tests.rs | 48 |
| editor_modes.rs | 199 |
| marks.rs | 192 |
| action.rs | 180 |
| dispatch.rs | 196 |
| editor_actions.rs | 179 |
| snippets.rs | 171 |
| ex_parse_ranges.rs | 122 |
| ex_dispatch.rs | 193 |
| cmdline_dispatch.rs | 65 |
| lib.rs | 161 |
| wave22_tests.rs | 130 |
