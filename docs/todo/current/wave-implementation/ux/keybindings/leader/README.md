# UX: Leader Key and Feature Chords (Iteration 36)

Back: [/docs/todo/current/wave-implementation/ux/keybindings/README.md](/docs/todo/current/wave-implementation/ux/keybindings/README.md)

## Scope

Make the default leader key (`Space`) reliable and ensure leader-based feature chords are reachable.

This checklist exists because leader conflicts can silently break explorer/terminal access even if the underlying features exist.

## Defining documents (direct, normative)

- Leader key contract:
  - [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md)
- Feature chords:
  - [/docs/spec/ux/keybindings/features.md](/docs/spec/ux/keybindings/features.md)
- Which-key style hints (optional but supported):
  - [/docs/spec/features/config/which-key.md](/docs/spec/features/config/which-key.md)
- Current surface + known drift:
  - [/docs/reference/CONFORMANCE_MODES_KEYS.md](/docs/reference/CONFORMANCE_MODES_KEYS.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Checklist

### A. Reserve `Space` as `<leader>` (default)

- [ ] In Normal mode, `Space` MUST act as `<leader>` and MUST NOT perform cursor motion or editing by itself.
- [ ] Pressing `<leader>` alone MUST NOT mutate buffer content; it MAY trigger which-key hints if enabled.
- [ ] Ensure keybinding tables do not bind `Space` to navigation (verify against [/docs/spec/ux/keybindings/navigation.md](/docs/spec/ux/keybindings/navigation.md)).

### B. Implement and test leader chords

- [ ] Ensure leader chords are parsed deterministically and are mode-scoped correctly.
- [ ] Add headless tests for leader chords that mutate editor UI state (when feasible).
- [ ] Add PTY E2E regressions for at least:
  - `<leader>e` opens/closes the explorer view
  - `<leader>t` opens/closes the integrated terminal view

### C. Conformance and limitations updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)

## Related

- PTY E2E harness: [/docs/todo/current/wave-implementation/technical/testing/pty-e2e/README.md](/docs/todo/current/wave-implementation/technical/testing/pty-e2e/README.md)
