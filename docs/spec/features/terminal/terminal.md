# Integrated Terminal Emulator

Back: [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)

The terminal subsystem MUST be a full-scratch implementation and a first-class
window type.

## Non-Negotiable Contract

| Requirement | Detail |
|---|---|
| Terminal is a window | Every terminal instance is a `WindowId` in the shared window tree |
| No simplified stub | PTY lifecycle, parser, screen model, and rendering MUST be real runtime behavior |
| Shared navigation | `Ctrl-w` navigation works identically across buffer, explorer, and terminal windows |
| Non-blocking IO | Heavy terminal output MUST NOT stall editing in other windows |

## Launch and Wiring

| Trigger | Required Path |
|---|---|
| `:terminal` | command parser -> core action -> terminal service spawn -> window tree insert |
| `<leader>t` | keymap -> action resolver -> same spawn path as `:terminal` |
| `<leader>tv` | keymap -> vertical split creation -> terminal content binding |
| `<leader>th` | keymap -> horizontal split creation -> terminal content binding |

`terminal` features MUST NOT be considered implemented unless these triggers reach
real PTY-backed behavior.

## Runtime Components

| Layer | Responsibility |
|---|---|
| Core-state | Owns terminal window metadata and focus/layout state |
| Terminal service | PTY process spawn/read/write/resize/cleanup |
| Parser | UTF-8 decode + VT state machine + CSI/OSC dispatch |
| Screen model | Main/alternate grids, cursor, scrollback, style attrs |
| Renderer | Maps terminal cells to window cells in frame output |

## Screen and Overflow Rules

| Rule | Requirement |
|---|---|
| On-screen rendering | Cell output MUST stay within the window text area |
| Long output lines | Output that exceeds window width MUST wrap to continuation rows |
| Wide graphemes | Width-2 graphemes MUST never split across rows |
| Continuation cell | Width-2 trailing cell MUST be marked continuation and non-addressable |
| Scrollback bound | Scrollback MUST be capped by `terminal.scrollback_lines` |

## PTY Lifecycle Rules

| Stage | Requirement |
|---|---|
| Spawn | Child process starts with window size and configured shell |
| Read | Async reads feed parser without blocking core task |
| Write | Terminal-insert keys forward to PTY input stream |
| Resize | Window resize sends PTY resize (`SIGWINCH`) and grid recompute |
| Close | Closing terminal window sends hangup/terminate and reaps child |

## Interaction Rules

| Context | Behavior |
|---|---|
| Terminal insert | Printable/control keys go to PTY, except dedicated escape chord |
| Escape chord | `Ctrl-\\ Ctrl-n` exits terminal insert to Normal mode |
| Window commands | `Ctrl-w` family remains available while terminal focused |
| Session restore | Restores terminal window nodes; process restarts (no process snapshot restore) |

## Failure Handling

| Failure | Required Behavior |
|---|---|
| PTY spawn failure | Window creation fails with explicit error notification |
| Child exit | Window shows exited state without crashing editor |
| Parser invalid sequence | Invalid escape sequence is ignored safely |
| Output flood | Backpressure prevents unbounded memory growth |

## Mandatory Verification

| ID | Scenario |
|---|---|
| TERM-01 | `:terminal` launches PTY-backed window |
| TERM-02 | `<leader>t` and split variants use same runtime path |
| TERM-03 | `Ctrl-w` navigation across buffer/explorer/terminal |
| TERM-04 | Resize sends PTY resize and preserves cursor visibility |
| TERM-05 | Close reaps child process without zombie leak |
| TERM-06 | Concurrent terminal output while editing adjacent buffer |
| TERM-07 | CJK terminal output wraps correctly with no half-cell cursor state |

## Related

- Escape parser: [/docs/spec/features/terminal/escape-parser.md](/docs/spec/features/terminal/escape-parser.md)
- Windows model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Split behavior: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- E2E tests: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
