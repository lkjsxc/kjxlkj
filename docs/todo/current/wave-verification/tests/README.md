# Verification: Test Suite Coverage

Back: [/docs/todo/current/wave-verification/README.md](/docs/todo/current/wave-verification/README.md)

## Scope

This checklist verifies that reconstruction has enough tests to prove critical behavior.

Normative source:

- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Current baseline inventory

- 2580 automated tests in `src/crates/**`.
- Existing regression coverage includes cursor boundaries, viewport follow, command parsing, and PTY regressions.

Representative suites:

- `src/crates/kjxlkj-core-state/tests/viewport_regression.rs`
- `src/crates/kjxlkj-core-state/tests/viewport_scroll_probes.rs`
- `src/crates/kjxlkj-core-state/tests/command_parsing.rs`
- `src/crates/kjxlkj-core/tests/e2e.rs`
- `src/crates/kjxlkj-host/src/pty_regressions.rs`

## Checklist

### A. Keep desirable implemented tests (must remain green)

- [x] Preserve append-at-EOL regressions (`append_at_eol_*`) and cursor clamp behavior.
- [x] Preserve wrap defaults and horizontal-scroll invariants.
- [x] Preserve command parsing/range parsing regressions.
- [x] Preserve newline insertion and write/quit persistence workflows.

### B. Add/maintain expert boundary PTY E2E tests

- [ ] `pty_append_eol_mode_churn`: repeated `a` + `Esc` never leaves floating cursor.
- [ ] `pty_wrap_long_cjk_line`: long Japanese line wraps and remains editable.
- [ ] `pty_leader_vs_ime_space`: IME conversion `Space` does not trigger leader mappings.
- [ ] `pty_tmux_detach_resume`: multiplexer attach/detach preserves interactive correctness.
- [ ] `pty_resize_storm_with_wrap`: resize storms with wrapping keep cursor visible.

### C. Stability and reproducibility

- [ ] Ensure PTY tests use deterministic timeouts and bounded retries.
- [ ] Ensure test failures report reproduction context (input script, temp file path, final mode).
- [ ] Ensure all mandatory regressions are linked to defining specs and conformance entries.

## Notes for next implementation pass

Boundary PTY E2E items in section B are intentionally left unchecked as standby work for the next implementation wave.
