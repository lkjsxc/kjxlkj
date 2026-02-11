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

### Waves 035–039 (Archived)

See [wave-progress-stage-04-late.md](wave-progress-stage-04-late.md) for
Waves 035 (Command and Route Wiring, 348 tests),
036 (Boundary and Error Semantics, 374 tests),
037 (Unit and Integration Coverage, 391 tests),
038 (Live E2E and Race Validation, 411 tests), and
039 (Ledger Synchronization and Stage Exit, 442 tests).

## Stage 05: Services and Features

### Waves 040–042 (Archived)

See [wave-progress-stage-05-early.md](wave-progress-stage-05-early.md) for
Waves 040 (Scope Freeze and Input Mapping, 473 tests),
041 (Requirement Extraction and Normalization, 493 tests), and
042 (State Model and Data Flow Design, 515 tests).

### Wave 043: Command and Route Wiring
- Status: COMPLETE
- Committed: 0f5e2c7f
- Evidence: 538 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Viewport state model: ViewportState with per-window scrolloff (default 5),
    sidescrolloff, wrap flag, text_rows/text_cols, top_line/left_col. Methods:
    ensure_visible (cursor-follow with vertical scrolloff and horizontal
    sidescrolloff margin clamping), scroll_center (zz), scroll_top (zt),
    scroll_bottom (zb), bottom_line, is_line_visible, clamp_top safety.
    8 unit tests
  - viewport.rs (156 lines, NEW) in kjxlkj-core-ui
  - viewport_tests.rs (64 lines, NEW, extracted tests)
  - Floating window model: FloatAnchor (Editor/Cursor/Window/NW/NE/SW/SE),
    BorderStyle (None/Single/Double/Rounded/Solid/Shadow/Custom), FloatKind
    (Dialog/Tooltip/Preview/Completion), FloatConfig with width/height/row/col/
    anchor/center/border/focusable/enter/zindex/title/footer/kind/close_on_focus_loss,
    dialog() and tooltip() factory constructors. FloatWindow instance with
    window_id/buffer_id/config/creation_order. FloatLayer manager with open/close/
    render_order (ascending zindex, creation tiebreak)/focusable query/count/
    is_empty. 7 unit tests
  - float_win.rs (146 lines, NEW) in kjxlkj-core-ui
  - float_win_tests.rs (72 lines, NEW, extracted tests)
  - Statusline DSL parser: DslToken enum (Literal/Separator/FilePath/
    FilePathAbsolute/Modified/ReadOnly/Line/Column/Percent/FileType/Highlight),
    DslVars struct for variable values. parse_format tokenizer: %f/%F/%m/%r/
    %l/%c/%p/%y variable expansion, %% literal percent, %= separator, %#Group#
    highlight groups. variable_token lookup table. render_tokens with separator
    marker (\x00), conditional [+]/[-] flags. 8 unit tests
  - statusline_dsl.rs (114 lines, NEW) in kjxlkj-core-ui
  - statusline_dsl_tests.rs (89 lines, NEW, extracted tests)
  - lib.rs (kjxlkj-core-ui) 14→17: +pub mod viewport; +pub mod float_win;
    +pub mod statusline_dsl;
  - Tier-C docs read: statusline-dsl.md, statusline.md (re-read), viewport.md,
    window/README.md, floating-windows.md, splits-advanced.md, splits-windows.md
    (re-read)
  - Ledger sync: CONFORMANCE (515→538), LIMITATIONS, DRIFT_MATRIX
    (+R-VIEWPORT-01, +R-FLOAT-01, +R-DSL-01, M4 21→24, M2 6→7)

### Wave 044: Boundary and Error Semantics
- Status: COMPLETE
- Committed: 805a0315
- Evidence: 556 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Tab page model: TabPage (id/layout/active_window/label/modified), TabId,
    TabList with ordered tab management (tab_new inserts after current, tab_close
    refuses last tab, tab_only keeps current, tab_next/tab_prev wrapping,
    tab_goto 1-indexed with range validation, tab_first/tab_last, tab_move
    absolute with clamping, tab_move_relative). Tabline visibility calculation.
    11 unit tests
  - tabs.rs (104 lines, NEW) in kjxlkj-core-ui
  - tabs_tests.rs (94 lines, NEW, extracted tests)
  - Zoom state: ZoomState with saved_layout/zoomed_window, zoom_in saves layout
    tree and replaces with single-window leaf, restore reinstates saved layout
    (with closed-window cleanup via remove_window_from_node/collapse_unary),
    toggle cycles in/out, indicator "[Z]" when zoomed, on_window_closed removes
    windows from saved layout. 7 unit tests
  - zoom.rs (87 lines, NEW) in kjxlkj-core-ui
  - zoom_tests.rs (67 lines, NEW, extracted tests)
  - Tab/zoom ex commands: :tabnew/:tabe/:tabedit, :tabclose/:tabc (with !),
    :tabonly/:tabo, :tabnext/:tabn, :tabprevious/:tabprev/:tabp/:tabNext/:tabN,
    :tabfirst/:tabfir/:tabrewind/:tabr, :tablast/:tabl, :tabmove/:tabm
    (absolute/relative/$/+N/-N), :ZoomToggle, :ZoomHeight, :ZoomWidth
  - Action variants: TabNew(Option<String>), TabClose, TabCloseForce, TabOnly,
    TabNext, TabPrev, TabFirst, TabLast, TabGoto(usize), TabMove(i32),
    ZoomToggle in kjxlkj-core-types action.rs (119→125)
  - command_parse.rs compacted (195→155): collapsed verbose match arms into
    single-line forms, added tab/zoom commands (+parse_tab_goto, +parse_tab_move
    helpers)
  - editor_action.rs (184→189): +tab/zoom action dispatch (deferred to
    integration layer)
  - lib.rs (kjxlkj-core-ui) 17→19: +pub mod tabs; +pub mod zoom;
  - Tier-C docs read: tabs.md, wincmd.md, window-layouts.md, window-presets.md,
    window-resize-modes.md, window-zoom.md, window_resizer.md
  - Ledger sync: CONFORMANCE (538→556), LIMITATIONS, DRIFT_MATRIX
    (+R-TAB-01, +R-ZOOM-01, M4 24→26)

### Wave 045: Unit and Integration Coverage
- Status: COMPLETE
- Committed: 9d9aba78
- Evidence: 586 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Mode configuration model: CursorShape enum (BlockBlink/BlockSteady/
    UnderBlink/UnderSteady/BarBlink/BarSteady/Default) with DECSCUSR codes,
    cursor_shape_for_mode for all 8 modes, mode_indicator for statusline text,
    cursor_restore_sequence. 13 unit tests
  - mode_config.rs (140 lines, NEW) in kjxlkj-core-ui
  - Command-line editing enhanced: Left/Right cursor, Home/End, Ctrl-b/Ctrl-e
    beginning/end, Ctrl-w word-backward delete, Ctrl-u delete-to-start, Ctrl-c
    cancel, Delete under cursor, mid-string character insertion. Tests extracted
    to editor_cmdline_tests.rs. 13 tests (7 new + 6 existing)
  - editor_cmdline.rs (150→122, tests extracted)
  - editor_cmdline_tests.rs (145 lines, NEW)
  - Insert completion model: CompletionSource (8 sources: Lsp/Snippet/Path/
    Buffer/Tag/Dictionary/Line/Include) priority-ordered, CompletionItem,
    CompletionState menu machine with start/next/prev/confirm/dismiss/narrow.
    11 unit tests
  - completion.rs (114 lines, NEW) in kjxlkj-core-ui
  - completion_tests.rs (92 lines, NEW, extracted tests)
  - lib.rs (kjxlkj-core-ui) 19→24: +pub mod mode_config; +pub mod completion;
    +#[cfg(test)] mod completion_tests;
  - lib.rs (kjxlkj-core-state) 73→75: +#[cfg(test)] mod editor_cmdline_tests;
  - Tier-C docs read: modes/README.md, command.md, configuration.md,
    insert/README.md, completion/README.md, insert-abbreviations.md,
    insert-completion-sources.md
  - Ledger sync: CONFORMANCE (556→586), LIMITATIONS, DRIFT_MATRIX
    (+R-MODECONF-01, +R-CMDLINE-01, +R-COMPLETION-01, M4 26→29)
