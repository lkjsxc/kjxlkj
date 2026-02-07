# Continue to the Next Iteration (Standby)

Back: [/docs/todo/current/wave-recursion/README.md](/docs/todo/current/wave-recursion/README.md)

## Purpose

Queue the next execution cycle after documentation hardening.

## Read first (normative)

- Multi-window behavior:
  - [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
  - [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)
- Multiplexer and terminal behavior:
  - [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)
  - [/docs/spec/features/terminal/tmux.md](/docs/spec/features/terminal/tmux.md)
- Verification contract:
  - [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
  - [/docs/todo/current/wave-verification/tests/README.md](/docs/todo/current/wave-verification/tests/README.md)

## Standby checklist

### A. Kickoff

- [ ] Start next implementation iteration and execute unchecked boundary PTY E2E tasks.

### B. Multi-window practical utilization (required before close)

- [ ] Enforce at least one default app workflow that edits and writes from a non-primary split (not only single-window path).
- [ ] Enforce at least one tab workflow (`:tabnew`/tab switch) in the standard edit-save flow.
- [ ] Enforce terminal-pane usage from within a multi-window layout (open terminal pane, return focus to editor split, continue editing).
- [ ] Ensure the above paths are reachable by documented commands/keybindings, not only internal APIs.

### C. Verification and traceability

- [ ] Implement/green all unchecked multi-window and boundary PTY tests in:
  - [/docs/todo/current/wave-verification/tests/README.md](/docs/todo/current/wave-verification/tests/README.md)
- [ ] Re-run full verification gate after implementation updates.
- [ ] Update conformance and limitations ledgers to match implemented behavior.

### D. Documentation topology hygiene

- [ ] Regenerate `/docs/todo/doc-coverage/` if documentation topology changes.
- [ ] Split line-count-exempt reference docs into <=200-line topology and remove exceptions from policy/checker.

### E. Completion handshake

- [ ] Invoke `Ask` once the next iteration is fully complete and verification is green.
