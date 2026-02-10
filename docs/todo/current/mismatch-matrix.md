# Mismatch Matrix

Back: [/docs/todo/current/phases/phase-0-foundation.md](/docs/todo/current/phases/phase-0-foundation.md)

Compares spec requirements against implementation and test coverage.

## Mismatch Classes

| Class | Description |
|---|---|
| C1 | Spec says X, implementation does X — no mismatch |
| C2 | Spec says X, implementation partially does X — gap exists |
| C3 | Spec says X, implementation only scaffolded — major gap |
| C4 | Spec says X, no implementation exists — unaddressed |

## Editing Mismatches

| Req ID | Spec | Implementation | Tests | Class |
|---|---|---|---|---|
| R-EDIT-01 | grapheme cursor | grapheme-based CursorPosition | SM-01–SM-04, WR-01–WR-02 | C1 |
| R-EDIT-02 | end-inclusive Normal | cursor_leave_insert clamps | WR-02, WR-08 | C1 |
| R-EDIT-03 | end-exclusive Insert | append moves past last grapheme | WR-01, state_tests | C1 |
| R-EDIT-04 | operators + motions | apply_operator in core-edit | edit_tests | C1 |
| R-EDIT-05 | text objects | word/sentence/paragraph | edit_tests | C1 |
| R-EDIT-06 | undo/redo tree | UndoTree with checkpoints | undo_tests | C1 |
| R-EDIT-07 | named registers | RegisterSet with named/numbered/special registers | register tests, gap_tests | C1 |

## Mode Mismatches

| Req ID | Spec | Implementation | Tests | Class |
|---|---|---|---|---|
| R-MODE-01 | normal dispatch | normal.rs with count/register | mode_tests | C1 |
| R-MODE-02 | insert input | insert_char, delete | mode_tests | C1 |
| R-MODE-03 | visual selection | visual dispatch + anchor tracking + operator application | gap_tests | C1 |
| R-MODE-04 | command-line mode | cmdline_ops.rs | state_tests | C1 |
| R-MODE-05 | replace mode | ReplaceChar/ReplaceBackspace with restore stack | gap_tests | C1 |
| R-MODE-06 | IME composition | ImeComposition model, route_ime_key | JP-01–JP-05 | C1 |

## Window Mismatches

| Req ID | Spec | Implementation | Tests | Class |
|---|---|---|---|---|
| R-WIN-01 | window tree | WindowTree with TabPage | state_tests | C1 |
| R-WIN-02 | h/v splits | split_horizontal/vertical | WR-05, HE-05 | C1 |
| R-WIN-03 | Ctrl-w nav | dispatch_window_key | WR-06 | C1 |
| R-WIN-04 | close + rebalance | close_active + rebuild_layout | state_tests | C1 |
| R-WIN-05 | stable WindowId | monotonic ID allocation | — | C1 |

## Terminal Mismatches

| Req ID | Spec | Implementation | Tests | Class |
|---|---|---|---|---|
| R-TERM-01 | PTY window | do_terminal_open creates TerminalId | PE-01, WR-03, HE-06 | C1 |
| R-TERM-02 | VT parser | ECMA-48 state machine | ST-01–ST-12 | C1 |
| R-TERM-03 | resize | terminal_size tracked + TerminalService resize propagation | PE-02, gap_tests | C1 |
| R-TERM-04 | TerminalInsert | mode transition on terminal open | WR-04 | C1 |

## Explorer Mismatches

| Req ID | Spec | Implementation | Tests | Class |
|---|---|---|---|---|
| R-EXP-01 | explorer wiring | toggle/reveal in window_ops | WR-05, HE-04 | C1 |
| R-EXP-02 | j/k/h/l nav | ExplorerState move/expand/collapse + dispatch_explorer_key | gap_tests | C1 |
| R-EXP-03 | open file | do_explorer_open_file | HE-04, HE-05 | C1 |
| R-EXP-04 | file ops | do_explorer_create_file/rename/delete | gap_tests | C1 |

## UI / Viewport Mismatches

| Req ID | Spec | Implementation | Tests | Class |
|---|---|---|---|---|
| R-UI-01 | scrolloff | scroll_to_cursor with scrolloff | state_tests | C1 |
| R-UI-02 | horizontal follow | scroll_horizontal with sidescrolloff | state_tests | C1 |
| R-UI-03 | wrap + padding | wrap_line with width-2 padding | BD-01, BD-02, BD-10 | C1 |
| R-UI-04 | zz/zt/zb | viewport_center/top/bottom | mode_tests | C1 |
| R-UI-05 | chrome views | paint_chrome.rs | render_tests | C1 |

## Session Mismatches

| Req ID | Spec | Implementation | Tests | Class |
|---|---|---|---|---|
| R-SESS-01 | session JSON | SessionData serde | session_ops tests | C1 |
| R-SESS-02 | terminal/explorer in session | window_to_layout_node | session_ops tests | C1 |
| R-SESS-03 | auto-session | auto_save_session/auto_load_session in app.rs | gap_tests | C1 |

## Service Mismatches

| Req ID | Spec | Implementation | Tests | Class |
|---|---|---|---|---|
| R-SVC-01 | LSP | LspService with lifecycle, crash recovery, request dispatch | lsp_tests (8 tests) | C1 |
| R-SVC-02 | Git | GitService with status cache, hunk navigation, signs | git_tests (8 tests) | C1 |
| R-SVC-03 | Finder/Index | IndexService with fuzzy matching, finder queries | index_tests (12 tests) | C1 |
| R-SVC-04 | Syntax | Language detection, keyword/string/comment/number highlighting | syntax_tests (18 tests) | C1 |

## Summary

| Class | Count | Percentage |
|---|---|---|
| C1 | 40 | 100% |
| C2 | 0 | 0% |
| C3 | 0 | 0% |
| C4 | 0 | 0% |
