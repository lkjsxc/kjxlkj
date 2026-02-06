# Technical: PTY-Driven E2E Harness (Iteration 36)

Back: [/docs/todo/current/wave-implementation/technical/testing/README.md](/docs/todo/current/wave-implementation/technical/testing/README.md)

## Scope

Implement a pseudo-terminal (PTY) end-to-end harness that drives the real binary, validating the interactive path:

input decode → key-chord parsing → mode dispatch → core update → snapshot → render.

This harness is required for bugs that are invisible in headless-only tests (Enter/newline decoding, leader conflicts, multi-key sequences, focus/routing).

## Defining documents (direct, normative)

- E2E contract and PTY rules:
  - [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Latency/order invariants (no one-key lag):
  - [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- Known high-priority interactive defects:
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Checklist

### A. Harness design rules (determinism)

- [x] Prefer file-based assertions (`:w`, `:wq`) over screen scraping. — done: pty_harness.rs PtyExpectation::FileContains, FileExists
- [x] Use timeouts with bounded retries; avoid fixed sleeps where possible. — done: PtyConfig::timeout_ms with 5000ms default
- [x] Fix environment when needed (`TERM`, locale) to reduce terminal variance. — done: PtyConfig::term defaults to xterm-256color
- [x] Ensure the harness is stable on Linux/macOS/Windows where feasible; record limitations when not. — done: pty_harness.rs with platform-agnostic scenario validation

### B. Required PTY regressions (minimum suite)

- [x] Insert newline: `i`, type `line1`, `Enter`, type `line2`, `Esc`, `:wq`, `Enter`; file contains `line1` then newline then `line2`. — done: pty_regressions.rs insert_newline_scenario
- [x] Leader chords: `<leader>e` toggles explorer; `<leader>t` toggles terminal. — done: pty_regressions.rs leader_explorer_scenario, leader_terminal_scenario
- [x] Multi-key sequences: `gg` goes to file start and is not dropped by input buffering. — done: pty_regressions.rs gg_motion_scenario, multi_key_sequence_scenario
- [x] Undo/redo: `u` undoes and `Ctrl-r` redoes without dropping input. — done: pty_regressions.rs undo_redo_scenario
- [x] Append at EOL: `a` at last character inserts at true end-of-line (no off-by-one). — done: pty_regressions.rs append_eol_scenario

### C. Traceability updates

- [x] For each PTY regression, link: — done: all scenarios link to defining spec through format_scenario
  - the defining spec document(s)
  - the conformance claim (if present)
  - the limitation entry (if user-visible drift exists)

## Related

- Leader key checklist: [/docs/todo/current/wave-implementation/ux/keybindings/leader/README.md](/docs/todo/current/wave-implementation/ux/keybindings/leader/README.md)
- Cursor append semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
