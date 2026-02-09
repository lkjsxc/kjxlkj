# Integrated Terminal Emulator

Back: [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)

The terminal subsystem MUST be a serious, full-scratch implementation and a first-class window type.

## Product Requirements

| Requirement | Detail |
|---|---|
| First-class window | Every terminal instance MUST be represented as a `WindowId` in the same layout tree as buffer windows. |
| Full-scratch emulator | The project MUST implement its own parser, screen model, cursor model, and PTY process lifecycle. |
| Modal interoperability | Window navigation and editor mode transitions MUST work identically across buffer and terminal windows. |
| Non-blocking behavior | Terminal IO MUST NOT stall editor input in other windows. |

## Terminal Window Contract

| Capability | Requirement |
|---|---|
| Open | `:terminal` and terminal key chords create a window with terminal content source |
| Split integration | Terminals MUST open in horizontal/vertical/floating/tab contexts |
| Navigation | `Ctrl-w h/j/k/l/w/W/p` MUST cross between terminal and buffer windows |
| Resize | Terminal resize MUST update window geometry and send PTY `SIGWINCH` |
| Close | Closing terminal window MUST send `SIGHUP` and reap child process |
| Session | Session restore MUST restore terminal windows as terminal windows (new process spawn) |

## Runtime Architecture

| Layer | Responsibility |
|---|---|
| Core-state | Window ownership, focus, mode routing, terminal-window metadata |
| Terminal service | PTY process control, async read/write, parser feed, screen buffer updates |
| Renderer | Convert terminal screen cells to editor frame cells |
| Input path | Route terminal-insert keystrokes to PTY; route escape chords to mode/window control |

## Screen Model Requirements

| Element | Requirement |
|---|---|
| Cell grid | Must include grapheme, style attributes, and width/continuation metadata |
| Scrollback | Must be bounded by configuration with deterministic truncation |
| Cursor | Must track row/column and visibility/style from escape state |
| Wide chars | Width-2 graphemes must mark continuation cells consistently |
| Alternate screen | Must support main/alternate screen switching and restoration |

## Escape Parser Requirements

Parser details are defined in [/docs/spec/features/terminal/escape-parser.md](/docs/spec/features/terminal/escape-parser.md).

Minimum required support:

- Cursor movement and addressing
- Erase and clear operations
- Scroll region and scrolling commands
- SGR attributes (basic, 256-color, 24-bit)
- OSC title and clipboard forms
- DECSET/DECRST modes needed by common TUI programs

## Terminal Modes and Key Routing

| Context | Behavior |
|---|---|
| Terminal insert | Printable and control keys route to PTY input stream |
| Escape chord | `Ctrl-\\ Ctrl-n` exits terminal insert and returns to Normal mode |
| Window commands | `Ctrl-w` commands remain available while terminal window is focused |
| Leader commands | Leader sequences should target terminal features only when composition state is idle |

## Reliability Requirements

| Scenario | Required Behavior |
|---|---|
| Child process exit | Window shows terminated state; editor remains stable |
| PTY read burst | Backpressure handling prevents unbounded memory growth |
| Resize storms | Final grid and PTY size match last geometry |
| Noisy output + editing | Buffer editing in adjacent windows remains responsive |

## Mandatory Verification

| ID | Scenario |
|---|---|
| TERM-01 | Open terminal from command and from keybinding |
| TERM-02 | Split terminal + window navigation with `Ctrl-w` family |
| TERM-03 | PTY process lifecycle (spawn, output, resize, close) |
| TERM-04 | Alternate screen enter/exit with a full-screen terminal app |
| TERM-05 | CJK/wide-char output rendering correctness |
| TERM-06 | Concurrent terminal output while editing buffer in adjacent window |

## Related

- Escape parser: [/docs/spec/features/terminal/escape-parser.md](/docs/spec/features/terminal/escape-parser.md)
- Windows model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Split behavior: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- Known current limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
