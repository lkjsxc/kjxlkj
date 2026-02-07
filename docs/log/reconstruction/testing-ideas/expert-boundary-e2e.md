# Expert Boundary E2E Ideas

Back: [/docs/log/reconstruction/testing-ideas/README.md](/docs/log/reconstruction/testing-ideas/README.md)

This file tracks high-value PTY E2E tests that target failures often missed by headless tests.

## Priority scenarios

| Test ID | Goal | Spec link |
|---|---|---|
| `pty_append_eol_mode_churn` | Prevent floating cursor after repeated `a` + `Esc` | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) |
| `pty_wrap_long_cjk_line` | Ensure long CJK line wraps and remains editable | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) |
| `pty_leader_vs_ime_space` | Ensure IME conversion does not trigger leader mapping | [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md) |
| `pty_tmux_detach_resume` | Validate multiplexer detach/attach session continuity | [/docs/spec/features/terminal/tmux.md](/docs/spec/features/terminal/tmux.md) |
| `pty_resize_storm_with_wrap` | Ensure cursor visibility invariants under rapid resize | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) |

## Harness requirements

| Requirement | Reason |
|---|---|
| Verify via saved file or serialized state | avoids brittle terminal scraping |
| Deterministic timeout bounds | prevents flake and hidden hangs |
| Record final mode and cursor in failure output | speeds triage |

## Placement guidance

Tests should live with PTY harness code under:

- `src/crates/kjxlkj-host/src/pty_regressions.rs`

Support helpers should remain in:

- `src/crates/kjxlkj-host/src/pty_harness.rs`
