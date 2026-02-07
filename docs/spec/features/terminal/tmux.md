# Terminal Multiplexer Contract

Back: [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)

This document defines the required behavior when `kjxlkj` is used with an external terminal multiplexer.

## Scope

`kjxlkj` MUST be fully usable in two layouts:

- standalone terminal session
- nested inside a multiplexer session (for example tmux or WezTerm)

This contract is normative for reconstruction.

## Required multiplexer capabilities

The multiplexer used with `kjxlkj` MUST support all capabilities below.

| Capability | Required behavior |
|---|---|
| Free layout changes | Users can split, close, resize, and rebalance panes during editing without restarting `kjxlkj`. |
| Multiple editor and terminal panes | Users can keep several `kjxlkj` processes and shell panes open concurrently. |
| Browser-like tabs | Users can create, close, rename, reorder, and switch tabs/windows quickly. |
| Virtual displays | Users can maintain multiple named workspaces (session-like units) and switch between them without losing state. |
| Attach and detach | Users can disconnect and later reconnect to the same running workspace from another terminal/client. |

## Reference mapping (tmux terminology)

| Contract concept | tmux concept |
|---|---|
| Browser-like tabs | window |
| Pane layout | pane + layout |
| Virtual display | session |
| Attach/detach | client attach/detach |

## Keybinding collision policy

| Rule | Requirement |
|---|---|
| Leader protection | Multiplexer prefix MUST NOT use `Space` (reserved for `kjxlkj` leader). |
| Window command protection | Multiplexer prefix SHOULD NOT use `Ctrl-w`. |
| Conflict resolution | If a collision exists, prefer remapping multiplexer keys instead of remapping core editor keys. |

## Terminal behavior requirements

When running inside a multiplexer, the combined environment MUST satisfy:

| Topic | Requirement |
|---|---|
| Escape latency | Meta/Escape chords MUST remain responsive; large escape delay settings are not acceptable. |
| Color fidelity | 24-bit color SHOULD be available so theme semantics remain stable. |
| Resize/focus routing | Resize and focus events SHOULD propagate; if focus events are unavailable, correctness MUST still hold. |
| Cursor semantics | Cursor shape differences between modes SHOULD be visible; missing shape support MUST NOT break editing logic. |
| Clipboard path | Clipboard integration SHOULD support OSC52-compatible copy flows in local/remote sessions. |

## Layout responsibility split

| Use case | Preferred layer |
|---|---|
| Multiple views in one editor process | `kjxlkj` internal windows/splits |
| Multiple editor instances, long-lived shells, project dashboards | multiplexer panes/windows/sessions |

## Canonical workflow patterns

| Pattern | Recommended structure |
|---|---|
| Session-per-project | One multiplexer session per repository or task family. |
| Tab-per-context | Separate tabs/windows for edit, test, logs, and repl. |
| Pane-per-operation | Short-lived pane splits for commands; collapse when done. |
| Detached continuity | Detach frequently; treat reconnect as normal workflow rather than exception. |

## Nested terminal usage

If a shell in an integrated terminal starts a multiplexer, three key layers coexist:

- multiplexer prefix layer
- `kjxlkj` key layer
- shell/terminal raw key layer

The implementation MUST keep key parsing deterministic so nested usage never causes random mode changes.

## Required automated tests

The reconstruction test suite MUST include at least one PTY E2E multiplexer smoke:

| Scenario | Expected result |
|---|---|
| Launch `kjxlkj` inside multiplexer, perform `i`, text input, `Esc`, `:wq` | Persisted file content is correct and process exits cleanly. |
| Split/resize multiplexer pane during active editor session | Editor remains responsive; no crash or stuck input state. |
| Attach/detach while editor process remains alive | Session resumes with correct mode and cursor state. |

If platform constraints prevent automated multiplexer tests, the limitation MUST be recorded in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) with an explicit plan to close the gap.

## Related

- Integrated terminal service: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Window and tabs model: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)
- Technical testing requirements: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
