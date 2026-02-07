# Verification: Test Suite Coverage

Back: [/docs/todo/current/wave-verification/README.md](/docs/todo/current/wave-verification/README.md)

## Scope

This checklist verifies that reconstruction has enough tests to prove critical behavior.

Normative source:

- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Current baseline inventory

- 1041 automated tests in `src/crates/**`.
- Existing regression coverage includes cursor boundaries, viewport follow, command parsing, PTY regressions, E2E modes/operations, and multi-window scenarios.

Representative suites:

- `src/crates/kjxlkj-core/tests/e2e_modes.rs`
- `src/crates/kjxlkj-core/tests/e2e_ops.rs`
- `src/crates/kjxlkj-core-state/tests/search_tests.rs`
- `src/crates/kjxlkj-core-state/tests/command_tests.rs`
- `src/crates/kjxlkj-host/tests/pty_boundary_tests.rs`
- `src/crates/kjxlkj-host/tests/pty_multiwindow_tests.rs`
- `src/crates/kjxlkj-host/src/pty_regressions.rs`

## Checklist

### A. Keep desirable implemented tests (must remain green)

- [x] Preserve append-at-EOL regressions (`append_at_eol_*`) and cursor clamp behavior.
- [x] Preserve wrap defaults and horizontal-scroll invariants.
- [x] Preserve command parsing/range parsing regressions.
- [x] Preserve newline insertion and write/quit persistence workflows.

### B. Add/maintain expert boundary PTY E2E tests

- [x] `pty_append_eol_mode_churn`: repeated `a` + `Esc` never leaves floating cursor.
- [x] `pty_wrap_long_cjk_line`: long Japanese line wraps and remains editable.
- [x] `pty_leader_vs_ime_space`: IME conversion `Space` does not trigger leader mappings.
- [x] `pty_tmux_detach_resume`: multiplexer attach/detach preserves interactive correctness.
- [x] `pty_resize_storm_with_wrap`: resize storms with wrapping keep cursor visible.

### C. Add/maintain multi-window practical-utilization PTY E2E tests

- [x] `pty_split_edit_write_non_primary`: split, move to non-primary window, edit, `:wq`, and verify persisted content.
- [x] `pty_tabs_cycle_edit_persist`: create tabs, switch tabs, edit in each, and verify no state loss on save/quit.
- [x] `pty_window_terminal_focus_roundtrip`: open terminal pane from split layout, return focus, continue edit/write flow.
- [x] `pty_window_close_rebalance_persistence`: close one split after edits and verify remaining window/tab state and file output are coherent.
- [x] `pty_multiplexer_multi_window_resume`: after tmux detach/attach, split/tab layout and cursor focus remain consistent.

### D. Stability and reproducibility

- [x] Ensure PTY tests use deterministic timeouts and bounded retries.
- [x] Ensure test failures report reproduction context (input script, temp file path, final mode).
- [x] Ensure all mandatory regressions are linked to defining specs and conformance entries.

## Notes

All boundary PTY E2E items in sections B, C, and D have been implemented and are green.
