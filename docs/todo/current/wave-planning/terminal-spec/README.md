# Integrated Terminal Spec (Iteration 34)

Back: [/docs/todo/current/wave-planning/README.md](/docs/todo/current/wave-planning/README.md)

## Objective

Ensure the integrated terminal feature is fully defined:

- PTY lifecycle and supervision expectations
- Input routing between editor and terminal pane
- Scrollback behavior and navigation
- Rendering constraints and performance expectations
- Exit behavior and persistence rules

## Defining documents (direct, normative)

- Terminal feature spec:
  - [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)
  - [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
  - [/docs/spec/features/terminal/tmux.md](/docs/spec/features/terminal/tmux.md)
  - [/docs/spec/features/terminal/wm-integration.md](/docs/spec/features/terminal/wm-integration.md)
- Window/pane model:
  - [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)
- Viewport invariants (scrollback is a viewport problem):
  - [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)

## Coverage traversal (to avoid missing docs)

- Terminal subtree:
  - [/docs/todo/doc-coverage/spec/features/terminal/README.md](/docs/todo/doc-coverage/spec/features/terminal/README.md)

## Tasks

### A. Clarify mode behavior

- [ ] Define which mode(s) apply when focus is in a terminal pane.
- [ ] Define how `Ctrl-\\ Ctrl-n` transitions terminal focus back to Normal.
- [ ] Define how terminal focus interacts with editor focus in splits/tabs.

### B. Define scrollback and viewport interaction

- [ ] Terminal panes MUST have deterministic scrollback.
- [ ] Terminal panes MUST follow the same viewport invariants as editor windows.
- [ ] Define scrollback navigation keys and whether they are shared with editor scrolling.

### C. Define failure and recovery behavior

- [ ] Terminal process crashes MUST surface visible diagnostics.
- [ ] Core editing MUST remain responsive under terminal output load.
- [ ] Define restart/reconnect rules (if any) and what is persisted.

### D. Define lifecycle, persistence, and exit behavior

- [ ] Define:
  - pane creation and close semantics
  - persistence rules (session restore, if specified)
  - exit codes and what the UI shows
