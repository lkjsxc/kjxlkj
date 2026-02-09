# Wave 21 — Reconstruction Log

## Summary

8 features implemented: local variables/return in functions, tab pages in sessions,
Ctrl-N/P popup navigation, dict literals and type(), numbered marks rotation,
equalprg option, snippet placeholders with defaults, backwards range notification.

## Test Results

- **Total tests**: 313 (5 + 300 + 8)
- **Passed**: 313
- **Failed**: 0
- **Clippy warnings**: 0

## Modified Files (with line counts)

| File | Lines | Change |
|------|-------|--------|
| `ex_scripting.rs` | 200 | handle_call_function returns Option<String>, :let l:var, :return, handle_autocmd compressed |
| `ex_dispatch.rs` | 193 | :let dispatch arm, backwards range notify |
| `session.rs` | 189 | tab_count/active_tab fields, tabs serialization |
| `session_tests.rs` | 47 | Added tab fields to initializer |
| `cmdline_dispatch.rs` | 65 | Ctrl-N/P popup navigation |
| `expr_eval.rs` | 198 | Dict literal pass-through, type() function |
| `marks.rs` | 186 | rotate_numbered() method |
| `cursor_ops_lists.rs` | 90 | push_jumplist calls rotate_numbered |
| `op_pending.rs` | 190 | Reindent case in apply_linewise_op |
| `format_ops.rs` | 197 | equalprg check in reindent_lines |
| `snippets.rs` | 152 | ${N:default} placeholder parsing |
| `lib.rs` | 159 | mod wave21_tests |

## New Files

| File | Lines | Purpose |
|------|-------|---------|
| `wave21_tests.rs` | 129 | 8 tests for wave 21 features |
| `wave21.md` | — | Requirements document |
| `wave21-log.md` | — | This log |

## LIMITATIONS.md Updates

8 entries updated with new next actions:
- SCRIPTING → Script-local (s:) variables
- SESSION → Tab page window layout serialization
- CMDCOMPL → Popup menu selection insertion
- REGISTERS → Dict key access, has_key()
- MARKS → Viminfo mark persistence
- TEXTMANIP → keywordprg option for K command
- EDITHELP → Snippet mirror tab-stops, nested placeholders
- RANGEPATTERN → Range expression evaluation

## Commit

Committed as wave21.
