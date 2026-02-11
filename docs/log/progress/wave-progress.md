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

### Wave 032: Scope Freeze and Input Mapping
- Status: COMPLETE
- Committed: 183769d2
- Evidence: 271 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Ctrl-w window command dispatch via PartialKey::WinCmd two-key prefix
  - normal_wincmd.rs (87 lines): resolves Ctrl-w + second key into window actions —
    h/j/k/l directional focus, w cycle, p previous, s/n split horizontal, v split
    vertical, c/q close window, o window only
  - Directional focus with geometry-based resolution: compute_rects() on layout tree,
    find nearest neighbor in requested direction using Manhattan distance
  - WindowOnly, FocusCycle, FocusPrevious, FocusDirection dispatch wired in
    editor_action.rs
  - window_only(), focus_cycle(), focus_direction() implemented in editor_window.rs
    (128 lines)
  - Split semantics corrected for Vim convention: :split (SplitHorizontal) now creates
    top/bottom layout, :vsplit (SplitVertical) now creates side-by-side layout
  - 7 unit tests in normal_wincmd.rs, 12 integration tests in editor_wincmd_tests.rs
    (144 lines) covering all wincmd paths with directional focus on asymmetric splits
  - normal.rs compacted (200 lines): Ctrl-w branch avoids clearing pending state
  - normal_partial.rs: WinCmd arm delegates to normal_wincmd module
  - pending.rs: WinCmd variant added (181 lines)
  - lib.rs (core-mode) expanded: +normal_wincmd module (147 lines)
  - lib.rs (core-state) expanded: +editor_wincmd_tests module (53 lines)
  - Tier-C docs read: multicursor, snippets, spell, surround, templates, git/README,
    git/diff-mode
  - Ledger sync: CONFORMANCE (252→271), LIMITATIONS, DRIFT_MATRIX updated

### Wave 033: Requirement Extraction and Normalization
- Status: COMPLETE
- Committed: 61039489 (impl) + df018219 (tests)
- Evidence: 295 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Boundary focus: Ctrl-w t (top-left) and b (bottom-right) using geometry-based
    min/max of (y*10000+x) across compute_rects leaf positions
  - Resize dispatch: Ctrl-w +/-/>/<  mapped to WindowResize(ResizeEdge, delta);
    equalize (=) delegates to layout.equalize(); maximize _/| as no-op placeholders
  - ResizeEdge enum added to kjxlkj-core-types (Height, Width)
  - Explorer routing: open_explorer creates ContentKind::Explorer(ExplorerStateId(0))
    leaf via split_horizontal on leftmost window; close_explorer finds explorer by
    ContentKind match; :ExplorerClose ex command added to command_parse.rs
  - action.rs compacted from 200→112 lines (removed per-variant doc comments,
    grouped variants with section comments)
  - layout_resize.rs created (127 lines): equalize(), find_container_info(),
    contains_leaf(), is_in_axis_split() with 3 unit tests
  - normal_wincmd.rs expanded (87→137): +10 dispatch arms, +8 unit tests
  - editor_window.rs expanded then compacted (128→150): +focus_top_left/bottom_right,
    equalize/resize/max placeholders, open/close explorer, leaf_rects() helper
  - editor_stage04_tests.rs created (189 lines): 14 integration tests covering
    boundary focus, resize, equalize, explorer lifecycle, :ExplorerClose command
  - Tier-C docs read: git/git.md, gitsigns.md, merge-conflicts.md, vimdiff.md,
    lsp/README.md, code-actions.md, code-lens.md
  - Ledger sync: CONFORMANCE (271→295), LIMITATIONS, DRIFT_MATRIX updated

### Wave 034: State Model and Data Flow Design
- Status: COMPLETE
- Committed: 38aa6893 (impl) + 1228f52e (tests)
- Evidence: 327 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Explorer service (kjxlkj-service-explorer) rewritten from stub:
    - lib.rs (197 lines): ExplorerState with root_path, tree, expansion_set,
      selected_index, cached visible rows, NodeId-based identity; VisibleRow struct;
      new/alloc_node_id/set_root/visible_rows/row_count/rebuild_visible_rows/flatten/
      clamp_selection; 4 unit tests
    - explorer_tree.rs (95 lines): NodeId(u64), ExplorerNode with id/name/is_dir/
      depth/path/children, file()/dir() constructors, find()/parent_of()/sort_children();
      3 unit tests
    - explorer_nav.rs (181 lines): ExplorerAction enum (MoveDown/MoveUp/CollapseOrParent/
      ExpandOrOpen/Toggle/Close), apply_action/move_down/move_up/collapse_or_parent/
      expand_or_open/toggle; 5 unit tests
  - Terminal service (kjxlkj-service-terminal) upgraded from stub:
    - lib.rs (77 lines): TerminalState with id/shell/title/exited/exit_code/cols/rows,
      new/set_exited/resize; TerminalService stub; 2 unit tests
  - Core-state wiring:
    - editor.rs (183 lines): +explorer_states HashMap, +explorer key interception
    - editor_explorer.rs (123 lines, NEW): focused_explorer_id(), handle_explorer_key()
      mapping j/k/h/l/Enter/o/q to ExplorerAction; 5 unit tests
    - editor_window.rs (157 lines): open_explorer creates ExplorerState, close_explorer
      cleans up explorer_states HashMap
    - lib.rs (58 lines): +editor_explorer, +editor_stage04b_tests modules
  - 13 integration tests in editor_stage04b_tests.rs (168 lines): explorer state
    lifecycle (populate/cleanup/reuse), key dispatch through handle_key
    (j/k/l/h/q/Enter), wincmd from explorer window, buffer key isolation,
    terminal state lifecycle/size
  - Tier-C docs read: completion.md, diagnostics.md, formatting.md, hover.md,
    lsp.md, navigation/README.md, call-hierarchy.md
  - Ledger sync: CONFORMANCE (295→327), LIMITATIONS, DRIFT_MATRIX updated

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
