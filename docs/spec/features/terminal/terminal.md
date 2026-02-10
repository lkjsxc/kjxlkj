# Integrated Terminal Emulator

Back: [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)

The terminal subsystem is a first-class window type with real PTY behavior.

## Non-Negotiable Contract

| Requirement | Detail |
|---|---|
| Terminal is a window | each terminal instance is a `WindowId` leaf in the shared tree |
| No stub path | PTY lifecycle + parser + screen model must be real runtime behavior |
| Shared navigation | `Ctrl-w` works identically across buffer/explorer/terminal leaves |
| Non-blocking IO | terminal output MUST NOT stall editing in other windows |

## Launch and Wiring

| Trigger | Required Path |
|---|---|
| `:terminal` | command parser -> core action -> terminal service spawn -> leaf insert |
| `<leader>t` | keymap -> same spawn path as `:terminal` |
| `<leader>tv` | vertical split create -> terminal content binding |
| `<leader>th` | horizontal split create -> terminal content binding |

## Runtime Components

| Layer | Responsibility |
|---|---|
| Core-state | terminal leaf metadata, focus, and layout ownership |
| Terminal service | PTY spawn/read/write/resize/cleanup |
| Parser | UTF-8 + VT state machine + CSI/OSC dispatch |
| Screen model | main/alternate buffers, cursor, styles, scrollback |
| Renderer | maps terminal cells into window cell region |

## Input Routing Modes

| Mode | Behavior |
|---|---|
| TerminalInsert | printable/control keys go to PTY input stream |
| TerminalNormal | window commands and editor navigation active |
| Escape chord | `Ctrl-\\ Ctrl-n` transitions TerminalInsert -> TerminalNormal |

While terminal leaf is focused, `Ctrl-w` family MUST remain available in both modes.

## Screen and Wrap Safety

| Rule | Requirement |
|---|---|
| On-screen rendering | terminal cells remain within text area bounds |
| Long output | wrap to continuation rows; no right-edge overflow |
| Wide grapheme safety | width-2 cells never split across rows |
| Continuation cell | width-2 trailing cell is continuation and non-addressable |
| Scrollback bound | enforce `terminal.scrollback_lines` hard cap |

## PTY Lifecycle Rules

| Stage | Requirement |
|---|---|
| Spawn | child starts with configured shell and initial terminal size |
| Read | async reads feed parser without blocking core |
| Write | terminal input writes are ordered and non-blocking |
| Resize | window resize sends PTY resize (`SIGWINCH`) |
| Close | window close triggers hangup/terminate and child reap |

## Failure Handling

| Failure | Required Behavior |
|---|---|
| PTY spawn failure | terminal leaf is not created; user gets explicit error |
| Child unexpected exit | leaf shows exited state; editor remains stable |
| Invalid escape sequence | safely ignored with parser state recovery |
| Output flood | bounded channels/backpressure prevent unbounded memory growth |

## Session Behavior

- terminal leaves are persisted as window nodes
- process state is not snapshotted; restart creates new process instance
- restored leaves MUST re-enter deterministic lifecycle state

## Mandatory Verification

| ID | Scenario |
|---|---|
| `TERM-01R` | `:terminal` launches PTY-backed terminal window |
| `TERM-02R` | `<leader>t` and split variants use same runtime path |
| `TERM-03R` | mixed `Ctrl-w` navigation across buffer/explorer/terminal |
| `TERM-04R` | resize propagates to PTY and preserves cursor visibility |
| `TERM-05R` | close reaps child without zombie leak |
| `TERM-06R` | heavy terminal output while editing adjacent buffer |
| `TERM-07R` | CJK terminal output wraps with no half-cell state |

## Related

- Escape parser: [/docs/spec/features/terminal/escape-parser.md](/docs/spec/features/terminal/escape-parser.md)
- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Split behavior: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- E2E matrix: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
