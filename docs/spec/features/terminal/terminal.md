# Integrated Terminal Emulator

Back: [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)

The terminal subsystem is a first-class window type with real PTY behavior.

## Non-Negotiable Contract

| Requirement | Detail |
|---|---|
| Terminal is a window | each terminal instance is a `WindowId` leaf |
| No stub path | PTY lifecycle, parser, and screen model must be runtime-reachable |
| Shared navigation | `Ctrl-w` semantics are identical across all window types |
| Non-blocking IO | terminal output must not stall editing or explorer input |

## Launch and Wiring

| Trigger | Required Path |
|---|---|
| `:terminal` | command parser -> core action -> terminal service spawn -> leaf insert |
| `<leader>t` | mapping resolver -> same spawn path as `:terminal` |
| `<leader>tv` | vertical split create -> terminal binding |
| `<leader>th` | horizontal split create -> terminal binding |

## Runtime Components

| Layer | Responsibility |
|---|---|
| Core-state | terminal leaf metadata, focus, and layout ownership |
| Terminal service | PTY spawn/read/write/resize/cleanup |
| Parser | UTF-8 + VT escape handling |
| Screen model | main/alt buffers, cursor, styles, scrollback |
| Renderer | maps terminal cells into window bounds |

## Input Routing Modes

| Mode | Behavior |
|---|---|
| TerminalInsert | printable/control keys go to PTY input stream |
| TerminalNormal | editor/window commands active |
| Escape chord | `Ctrl-\\ Ctrl-n` transitions TerminalInsert -> TerminalNormal |

`Ctrl-w` commands must stay available in both terminal modes.

## PTY Lifecycle Rules

| Stage | Requirement |
|---|---|
| Spawn | child starts with configured shell and window size |
| Read | async reads feed parser without blocking core |
| Write | input writes preserve order and bounded latency |
| Resize | terminal window resize propagates to PTY (`SIGWINCH`) |
| Close | close triggers hangup/terminate and child reap |

## Screen and Wrap Safety

| Rule | Requirement |
|---|---|
| on-screen rendering | emitted terminal cells stay within viewport bounds |
| long output | wraps to continuation rows without right-edge overflow |
| wide grapheme safety | width-2 cells never split across rows |
| continuation semantics | continuation cells are non-addressable cursor targets |
| scrollback cap | `terminal.scrollback_lines` is hard-bounded |

## Failure Handling

| Failure | Required Behavior |
|---|---|
| PTY spawn failure | no terminal leaf is created; explicit error shown |
| child exits unexpectedly | leaf enters exited state and editor remains stable |
| invalid escape sequence | parser recovers safely without corrupting screen state |
| output flood | bounded queues/backpressure prevent unbounded memory growth |

## Mandatory Verification

| ID | Scenario |
|---|---|
| `TERM-01R` | `:terminal` launches PTY-backed terminal |
| `TERM-02R` | leader launch variants use identical runtime semantics |
| `TERM-03R` | mixed `Ctrl-w` navigation across terminal leaves |
| `TERM-04R` | resize propagation preserves cursor visibility |
| `TERM-05R` | close during output reaps child with no zombie leak |
| `TERM-06R` | output flood while editing adjacent buffer remains responsive |
| `TERM-07R` | CJK terminal output wraps without half-cell states |

## Related

- Escape parser: [/docs/spec/features/terminal/escape-parser.md](/docs/spec/features/terminal/escape-parser.md)
- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- E2E matrix: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
