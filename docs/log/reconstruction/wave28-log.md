# Wave 28 — Reconstruction Log

## Summary
Wave 28 implemented 8 features with 11 new tests (373 total across workspace).

## Features

1. **REQ-REGEXEQ-01**: Regex equivalence classes `\%d123`, `\%x1f`, `\%o177` — translates char-code patterns to literal characters in regex_translate.rs. Added `collect_digits()`, `collect_hex()`, `collect_oct()`, `push_escaped_char()` helpers.

2. **REQ-ECHOERR-01**: `:echoerr` raises error notification; `:throw` sets `last_error` field on EditorState and raises E605 exception notification. Added in ex_dispatch.rs.

3. **REQ-SNIPPETVAR-01**: `expand_snippet_vars()` replaces known snippet variables ($TM_FILENAME, $CLIPBOARD, etc.) from a HashMap context before tab-stop parsing. Added `expand_with_vars()` method to SnippetRegistry.

4. **REQ-RANGEREG-01**: Range expression addresses now have access to `@a..@z` register contents via reg_vars HashMap populated in ex_dispatch.rs and passed through RangeContext.vars.

5. **REQ-MARKSTACK-01**: Mark stack with `push_mark_stack()`/`pop_mark_stack()` (capped at 100) in MarkFile. `g'`/`` g` `` dispatch JumpFromMarkStack action. Jump-to-mark pushes current position before jumping.

6. **REQ-REGFILTER-01**: `:registers abc` filters display to listed register chars. `handle_list_registers_filtered(filter)` applies char filter to unnamed, named, numbered, last-inserted, and readonly registers.

7. **REQ-INDENTPLUG-01**: `load_indent_plugin()` searches for indent/{ft}.vim and ftplugin/{ft}_indent.vim. Falls back to `apply_builtin_indent()` with defaults for 20+ filetypes (shiftwidth/tabstop/expandtab).

8. **REQ-KREMOTE-01**: K command with URL-based keywordprg support. When keywordprg starts with http/https, formats URL with `{keyword}` template or appends word. No process spawn for URL patterns.

## File Changes
- `regex_translate.rs`: 169 lines (added \%d/\%x/\%o, compressed functions)
- `ex_dispatch.rs`: 195 lines (echoerr/throw, register vars, register filter dispatch)
- `editor.rs`: 200 lines (last_error field)
- `snippets.rs`: 197 lines (expand_with_vars, expand_snippet_vars)
- `marks.rs`: 190 lines (mark_stack, push/pop)
- `editor_search_marks.rs`: 196 lines (mark_stack push, jump_from_mark_stack)
- `action.rs`: 181 lines (JumpFromMarkStack variant)
- `dispatch.rs`: 197 lines (g'/g` → JumpFromMarkStack)
- `editor_actions.rs`: 196 lines (JumpFromMarkStack dispatch, URL keywordprg)
- `ex_scripting.rs`: 195 lines (handle_list_registers_filtered with filter)
- `config_loader.rs`: 172 lines (load_indent_plugin, apply_builtin_indent)
- `ex_buffer_cmds.rs`: 123 lines (unchanged after removing duplicate)
- `wave28_tests.rs`: 120 lines (11 tests)
- `wave8_tests.rs`: updated caller to handle_list_registers_filtered

## Tests
- 5 (kjxlkj-core-types) + 360 (kjxlkj-core-state) + 8 (kjxlkj-core) = 373 total
- All pass, zero clippy warnings
- All files ≤ 200 lines
