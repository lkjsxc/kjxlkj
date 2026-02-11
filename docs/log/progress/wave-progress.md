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
