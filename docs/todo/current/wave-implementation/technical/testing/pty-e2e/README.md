# Technical: PTY-Driven E2E Harness (Iteration 35)

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

- [x] Prefer file-based assertions (`:w`, `:wq`) over screen scraping.
- [x] Use timeouts with bounded retries; avoid fixed sleeps where possible.
- [x] Fix environment when needed (`TERM`, locale) to reduce terminal variance.
- [x] Ensure the harness is stable on Linux/macOS/Windows where feasible; record limitations when not.

### B. Required PTY regressions (minimum suite)

- [x] Insert newline: `i`, type `line1`, `Enter`, type `line2`, `Esc`, `:wq`, `Enter`; file contains `line1` then newline then `line2`.
- [x] Leader chords: `<leader>e` toggles explorer; `<leader>t` toggles terminal.
- [x] Multi-key sequences: `gg` goes to file start and is not dropped by input buffering.
- [x] Undo/redo: `u` undoes and `Ctrl-r` redoes without dropping input.
- [x] Append at EOL: `a` at last character inserts at true end-of-line (no off-by-one).

### C. Traceability updates

- [x] For each PTY regression, link:
  - the defining spec document(s)
  - the conformance claim (if present)
  - the limitation entry (if user-visible drift exists)

## Related

- Leader key checklist: [/docs/todo/current/wave-implementation/ux/keybindings/leader/README.md](/docs/todo/current/wave-implementation/ux/keybindings/leader/README.md)
- Cursor append semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
