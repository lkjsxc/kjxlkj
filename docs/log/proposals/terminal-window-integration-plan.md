# Proposal: Terminal and Window Integration Plan

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem

Current runtime has partial terminal and window integration. Terminal PTY logic exists but is not
fully wired to user-facing actions and layout behavior.

## Proposal

1. Implement `Action::SpawnTerminal` end-to-end in core-state dispatch and runtime orchestration.
2. Represent terminal panes in the same layout graph as buffer and explorer windows.
3. Route resize and close events to PTY lifecycle (`SIGWINCH`, `SIGHUP`, reap).
4. Add PTY E2E tests for spawn/output/resize/close and mixed-window navigation.

## Acceptance Criteria

- `:terminal` opens a real PTY-backed pane.
- `Ctrl-w` navigation works across buffer/terminal/explorer windows.
- Closing terminal pane terminates child process without zombie leaks.
- Test evidence is recorded in conformance and verification logs.

## Related

- Target terminal spec: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Window spec: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
