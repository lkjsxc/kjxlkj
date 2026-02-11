# Wave Progress Log

Tracks completion of each wave with evidence.

## Stages 00–03 (Archived)

See [wave-progress-stages-00-02.md](wave-progress-stages-00-02.md) for complete
Stage 00 (Foundation), Stage 01 (Architecture Core), and Stage 02
(Editing and Modes) progress details.
See [wave-progress-stage-03.md](wave-progress-stage-03.md) for Stage 03
(Commands and Ranges) progress details.

- Stage 00 (Waves 000–007): COMPLETE
- Stage 01 (Waves 008–015): COMPLETE, 043b0f78
- Stage 02 (Waves 016–023): COMPLETE, final commit 25bcc66d, 173 tests
- Stage 03 (Waves 024–031): COMPLETE, final commit b8b76664, 252 tests

## Stage 04: Window, Explorer, Terminal

### Waves 032–034 (Archived)

See [wave-progress-stage-04-early.md](wave-progress-stage-04-early.md) for
Waves 032 (Scope Freeze and Input Mapping, 271 tests),
033 (Requirement Extraction, 295 tests), and
034 (State Model and Data Flow, 327 tests).

### Wave 035: Command and Route Wiring
- Status: COMPLETE
- Committed: cc415d57 (impl) + f9f81ec8 (tests)
- Evidence: 348 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Wincmd expanded: W (reverse cycle), H/J/K/L (move-to-edge placeholder), r/R
    (rotate placeholder), x (exchange placeholder)
  - normal_wincmd.rs (137→171): +9 dispatch arms (W/H/J/K/L/r/R/x), +6 unit tests
  - FocusCycleReverse, WindowMoveEdge(Direction), WindowRotate(bool), WindowExchange
    action variants added to kjxlkj-core-types action.rs (112→113)
  - editor_action.rs (194→199): +5 dispatch arms (OpenTerminal, FocusCycleReverse,
    WindowMoveEdge, WindowRotate, WindowExchange)
  - editor_window.rs (157→176): +focus_cycle_reverse() wrapping backward through
    window ID list, +open_terminal() creating ContentKind::Terminal(TerminalId) leaf
    via split_vertical below current window
  - Explorer v/s split-open keys: editor_explorer.rs (123→170) enhanced with
    ExplorerKeyResult enum for split-open routing; v opens selected file in vertical
    split, s opens in horizontal split; directory targets are no-ops
  - ExplorerState.selected_row() accessor added to explorer service lib.rs
  - Explorer service lib.rs split: tests extracted to explorer_state_tests.rs (87 lines)
    to keep lib.rs at 126 lines; +1 new test (selected_row_returns_correct)
  - editor_stage04c_tests.rs (193 lines, NEW): 14 integration tests — wincmd W
    reverse cycle (2), H/J/K/L/r/R/x no-crash (3), terminal window creation (3),
    focus cycle reverse (2), explorer v/s split-open (4)
  - Tier-C docs read: document-links.md, document-symbols.md, references.md,
    type-hierarchy.md, workspace-symbols.md, rename.md, signature-help.md
  - Ledger sync: CONFORMANCE (327→348), LIMITATIONS, DRIFT_MATRIX updated

### Wave 036: Boundary and Error Semantics
- Status: COMPLETE
- Committed: b5245bc6
- Evidence: 374 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Jumplist navigation (Ctrl-o/Ctrl-i) and changelist navigation (g;/g,)
  - PositionList data structure in navlist.rs (128 lines): 100 entry cap,
    go_older/go_newer/push, duplicate consecutive dedup, capacity enforcement
  - Action variants: JumpOlder, JumpNewer, ChangeOlder, ChangeNewer in
    kjxlkj-core-types action.rs (113→114)
  - Key dispatch: Ctrl-o→JumpOlder, Ctrl-i→JumpNewer in normal.rs;
    g;→ChangeOlder, g,→ChangeNewer in normal_g.rs
  - EditorState: jumplist + changelist fields (PositionList), record_jump/
    record_change methods in editor_nav.rs (70 lines)
  - navigate_jumplist/navigate_changelist with buffer-bounds clamping
  - Jump recording on GotoLine/GotoFirstLine/GotoLastLine/SearchNext/SearchPrev/
    StarSearchForward/StarSearchBackward via is_jump_action()
  - Change recording on all text-changing actions via is_text_changing()
  - editor_action.rs compacted (199→191): merged single-statement braced arms
  - normal.rs compacted (202→200): merged test formatting, +2 tests
  - normal_g.rs compacted (201→165): all tests to inline format, +2 tests
  - editor.rs expanded (183→200): +jumplist/changelist fields, +is_jump_action,
    +record_jump/record_change calls
  - editor_stage04d_tests.rs (151 lines, NEW): 16 boundary tests — jumplist
    empty/past-end (4), jumplist recording on G/Ctrl-o (2), changelist empty/
    recording/navigate (4), window close/only/focus boundary (4), explorer close
    when none (1), terminal open (1)
  - navlist.rs unit tests (6): push_and_go_older, go_newer_after_older,
    push_truncates_future, capacity_cap, duplicate_consecutive_ignored,
    empty_list_returns_none
  - Tier-C docs read: changelist.md, jumplist.md, finder.md, flash.md,
    include-search.md
  - Ledger sync: CONFORMANCE (348→374), LIMITATIONS, DRIFT_MATRIX updated

### Wave 037: Unit and Integration Coverage
- Status: COMPLETE
- Committed: f896418a
- Evidence: 391 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Mark system: m{a-z} set mark, '{a-z} goto mark line (first non-blank),
    `{a-z} goto mark exact position
  - MarkStore in marks.rs (88 lines): HashMap<char, MarkPos>, set/get/remove/list,
    lowercase a-z only (uppercase silently ignored). 5 unit tests
  - Action variants: SetMark(char), GotoMarkLine(char), GotoMarkExact(char) in
    kjxlkj-core-types action.rs (114→115)
  - Partial key dispatch: m→SetMark, '→GotoMarkLine, `→GotoMarkExact in
    normal.rs (200→198) and normal_partial.rs (93→108)
  - EditorState: marks field (MarkStore) in editor.rs (200, at limit)
  - editor_nav.rs expanded (70→115): +set_mark_at_cursor, +goto_mark_line (first
    non-blank using find for non-whitespace), +goto_mark_exact (buffer-bounds
    clamping)
  - editor_action.rs (191→194): +SetMark/GotoMarkLine/GotoMarkExact dispatch
  - editor_stage04e_tests.rs (130 lines, NEW): 12 integration tests — mark set
    and goto exact (1), mark set and goto line with first-non-blank (1), goto
    unset mark no-op (2: exact + line), mark overwrite (1), mark persistence
    across insert mode (1), uppercase mark ignored (1), multiple marks
    independent (1), goto exact clamps when lines deleted (1), goto line with
    tabs first-non-blank (1), action API direct (1), goto line on empty line (1)
  - lib.rs (core-state) 64→67 (+marks module, +editor_stage04e_tests)
  - Tier-C docs read: marks.md, quickfix.md, tags.md, session/README.md,
    auto_save.md, ex-commands-detailed.md, expression-register.md
  - Ledger sync: CONFORMANCE (374→391), LIMITATIONS, DRIFT_MATRIX (+R-MARK-01)

### Wave 038: Live E2E and Race Validation
- Status: COMPLETE
- Committed: 3cdbc363
- Evidence: 411 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Macro recording and playback: q{a-z} starts recording, q stops, @{a-z} plays
  - macros.rs (117 lines, NEW): MacroState with recording/buffer, MacroKey struct,
    start/stop/capture/is_recording, keys_to_string serializer. 5 unit tests
  - Action variants: MacroRecordStart(char), MacroRecordStop, MacroPlay(char) in
    kjxlkj-core-types action.rs (115→116)
  - Key dispatch: q→MacroRecord partial, @→MacroPlay partial in normal.rs (198→200)
  - normal_partial.rs (108→115): MacroRecord→MacroRecordStart, MacroPlay→MacroPlay
  - EditorState: macro_state field (MacroState) in editor.rs (200→194, compacted)
  - handle_key integration: stop-q intercept when recording active in Normal mode,
    key capture via macro_state.capture() before dispatch
  - editor_nav.rs (115→171): +start_macro_recording, +stop_macro_recording (saves
    to register via keys_to_string), +play_macro (reads register, parse_macro_keys,
    replays via handle_key), +parse_macro_keys (Ctrl/Esc/BS/Enter/Tab support)
  - editor_action.rs (194→197): +MacroRecordStart/MacroRecordStop/MacroPlay dispatch
  - editor_stage04f_tests.rs (190 lines, NEW): 15 integration tests — macro record
    and play (1), insert mode recording (1), unset register playback noop (1), stop
    without start (1), multiple records overwrite (1), uppercase rejected (1), mode
    switch stress during recording (1), mark+macro interaction (1), jumplist+macro
    interaction (1), split+macro interaction (1), rapid record/stop 100x (1),
    deterministic replay (1), mark/split/jumplist combined stress (1), mode churn
    with marks 50x (1), changelist stress 20 deletes (1)
  - lib.rs (core-state) 67→70 (+macros, +editor_stage04f_tests)
  - wave-progress.md split: waves 032-034 archived to wave-progress-stage-04-early.md
  - Tier-C docs read: macros.md, project-config.md, registers.md (session),
    sessions.md, undo_tree.md, view-management.md, workspaces.md
  - Ledger sync: CONFORMANCE (391→411), LIMITATIONS, DRIFT_MATRIX (+R-MACRO-01)

### Wave 039: Ledger Synchronization and Stage Exit
- Status: COMPLETE
- Committed: b0509062
- Evidence: 442 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Fold commands: zo (open), zc (close), za (toggle), zR (open all), zM (close
    all), zr (reduce fold level), zm (increase fold level), zj (next closed fold),
    zk (prev closed fold)
  - folds.rs (184 lines, NEW): FoldState with indent-based fold computation
    (compute_indent_folds), FoldRegion (start/end/level), fold_level with
    reduce/more, open/close/toggle per-line, open_all/close_all, next_closed/
    prev_closed, is_hidden. indent_level (4-space), fold_end helper. 6 unit tests
  - Action variants: FoldOpen, FoldClose, FoldToggle, FoldOpenAll, FoldCloseAll,
    FoldReduce, FoldMore, FoldNext, FoldPrev in action.rs (116→119)
  - normal_z.rs (43→115): +zo/zc/za/zR/zM/zr/zm/zj/zk dispatch, +9 unit tests
  - editor.rs (194→196): +fold_state: FoldState field
  - editor_action.rs (197→184): +fold dispatch, compacted single-method arms
  - editor_nav.rs (171→157): +fold_open/fold_close/fold_toggle/fold_close_all/
    fold_next/fold_prev/focused_cursor_line, +apply_nav_position shared helper
    (extracted from duplicate jumplist/changelist code), all methods compacted
  - lib.rs (core-state) 70→73 (+folds, +editor_stage04g_tests)
  - editor_stage04g_tests.rs (169 lines, NEW): 16 integration tests — fold
    dispatch (zo/zc/za/zR/zM), fold navigation (zj/zk), zj noop, fold_is_hidden,
    rapid fold toggle 100x, non-fold line noop, empty buffer safety, macro+fold
    interaction, mark+fold interaction, reduce/more cycle, combined stress 20x
  - Tier-C docs read: folding.md, folds-advanced.md, highlight-groups.md,
    colorscheme-creation.md, inlay-hints.md, semantic-tokens.md, syntax/README.md
  - Tree-sitter and expression fold methods deferred (only indent-based)
  - Wave-039 changes: `action.rs` 116→119 (+9 fold variants), `normal_z.rs` 43→115
    (+zo/zc/za/zR/zM/zr/zm/zj/zk dispatch, +9 unit tests), `editor.rs` 194→196
    (+fold_state field), `editor_action.rs` 197→184 (+fold dispatch, compacted
    existing arms), `editor_nav.rs` 171→157 (+fold methods, +apply_nav_position
    shared helper, refactored and compacted), `lib.rs` (core-state) 70→73
    (+folds, +editor_stage04g_tests). New files: `folds.rs` (184, FoldState,
    6 tests), `editor_stage04g_tests.rs` (169, 16 integration tests)
  - Ledger sync: CONFORMANCE (411→442), LIMITATIONS, DRIFT_MATRIX (+R-FOLD-01)

## Stage 05: Services and Features

### Wave 040: Scope Freeze and Input Mapping
- Status: COMPLETE
- Committed: 81b889f5
- Evidence: 473 tests pass, all files ≤ 200 lines
- Key deliverables:
  - VT100/xterm escape parser (13 states): Ground, Escape, EscapeIntermediate,
    CsiEntry, CsiParam, CsiIntermediate, CsiIgnore, OscString, DcsEntry,
    DcsParam, DcsPassthrough, DcsIgnore, SosPmApc
  - CSI dispatch: CUU/CUD/CUF/CUB/CUP/CNL/CPL/CHA/VPA, ED/EL/ECH, SU/SD,
    IL/DL, ICH/DCH, SGR, DECSTBM, cursor save/restore
  - SGR: bold/dim/italic/underline/reverse/strikethrough, basic 8 + bright 8
    + 256-color + RGB for fg/bg
  - Private modes: DECTCEM (cursor visibility), alt screen (47/1049),
    bracketed paste (2004)
  - OSC title (0;/2; prefix), escape dispatch (M/D/E/7/8/c), UTF-8 accumulation
  - Screen model: Cell grid with char/fg/bg/6 style attributes, cursor,
    scroll region, saved cursor, alt-screen, bracketed-paste. Operations:
    put_char (line wrap), linefeed (scroll), CR, BS, tab (8-col), reverse
    index, erase display/line/chars, insert/delete chars, scroll up/down,
    insert/delete lines, save/restore cursor, reset
  - Filetype detection: 15 languages by extension (rs/py/js/jsx/ts/tsx/go/
    c/h/cpp/cc/cxx/hpp/hh/hxx/md/json/yaml/yml/toml/html/htm/css/sh/bash/lua)
    + shebang fallback (python/node/bash/lua)
  - escape_parser.rs split: Parser core (170 lines), CSI dispatch extracted
    to csi.rs (98 lines), tests to parser_tests.rs (51 lines)
  - screen.rs split: Screen model (177 lines), tests to screen_tests.rs (46)
  - lib.rs (terminal) 77→81: +pub mod csi/escape_parser/screen
  - lib.rs (index) 11→12: +pub mod filetype
  - New files: escape_parser.rs (170), csi.rs (98), parser_tests.rs (51),
    screen.rs (177), screen_tests.rs (46), filetype.rs (81)
  - Tier-C docs read: syntax-files.md, syntax.md, dap.md, escape-parser.md,
    remote.md, terminal.md
  - Ledger sync: CONFORMANCE (442→473), LIMITATIONS, DRIFT_MATRIX
    (+R-FILETYPE-01, R-TERM-01 updated, M4 16→17)
