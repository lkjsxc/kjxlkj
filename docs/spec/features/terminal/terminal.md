# Integrated Terminal Emulator

Back: [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)

Full-scratch terminal emulator built into the editor. Terminals are managed as windows in the editor's window tree.

## Architecture

The terminal emulator is a first-class window type. Each terminal instance:

- Is a window in the editor's window tree (split, float, or tab)
- Owns a PTY child process
- Maintains an internal character grid (the terminal screen buffer)
- Parses VT100/xterm escape sequences from the PTY output
- Forwards keyboard input from the editor to the PTY

## Terminal as window (normative)

Terminal panes MUST be managed as windows using the same window tree, layout, and navigation system as editor buffer windows. This means:

| Requirement | Detail |
|---|---|
| Window identity | Each terminal has a `WindowId` in the window tree. |
| Navigation | `Ctrl-w h/j/k/l` navigates between terminal and buffer windows identically. |
| Splitting | Terminal windows can be the source or target of split operations. |
| Resizing | Window resize applies to terminals; the terminal MUST send `SIGWINCH` to the PTY. |
| Closing | Closing a terminal window sends `SIGHUP` to the PTY process. |
| Session save | Terminal window positions are saved in session state (process state is not). |

## Terminal screen buffer

Each terminal maintains an internal grid representing the terminal display:

| Field | Type | Description |
|---|---|---|
| `rows` | integer | Number of rows in the grid |
| `cols` | integer | Number of columns in the grid |
| `cells` | 2D array of `Cell` | The character grid |
| `cursor_row` | integer | Current cursor row (0-based) |
| `cursor_col` | integer | Current cursor column (0-based) |
| `scrollback` | ring buffer of rows | Lines scrolled off the top |
| `scroll_region` | `(top, bottom)` | Current scroll region bounds |

### Cell model

Each cell in the grid:

| Field | Type | Description |
|---|---|---|
| `grapheme` | string | The displayed grapheme cluster (empty string for continuation cells of wide chars) |
| `width` | 1 or 2 | Display width of this cell's grapheme |
| `fg` | color | Foreground color (indexed or RGB) |
| `bg` | color | Background color (indexed or RGB) |
| `attrs` | bitfield | Bold, italic, underline, blink, reverse, strikethrough, dim |
| `is_wide_continuation` | boolean | True if this cell is the second column of a width-2 character |

## Escape sequence parsing (normative)

The terminal MUST implement a state machine parser for VT100/xterm escape sequences. The parser MUST handle:

| Category | Sequences |
|---|---|
| Cursor movement | CUU, CUD, CUF, CUB, CUP, HVP, save/restore cursor (DECSC/DECRC) |
| Erase | ED (erase display), EL (erase line), ECH (erase characters) |
| Scroll | SU (scroll up), SD (scroll down), DECSTBM (set scroll region) |
| Character attributes | SGR (select graphic rendition): colors, bold, italic, underline, reverse, reset |
| Mode setting | DECSET/DECRST for alternate screen, cursor visibility, bracketed paste, mouse reporting (ignored) |
| OSC | Window title (OSC 0/2), clipboard (OSC 52) |
| Character sets | G0/G1 designation, SI/SO shift |
| Tabs | HTS (set tab), TBC (clear tabs), CHT (cursor horizontal tab) |

### SGR color support

| Format | Requirement |
|---|---|
| Basic 8 colors (30-37, 40-47) | MUST support |
| Bright 8 colors (90-97, 100-107) | MUST support |
| 256-color (38;5;N, 48;5;N) | MUST support |
| 24-bit RGB (38;2;R;G;B, 48;2;R;G;B) | MUST support |

## PTY management

| Operation | Requirement |
|---|---|
| Spawn | Fork PTY using `openpty`/`forkpty` or equivalent; exec the configured shell. |
| Read | Async read from PTY master fd; feed bytes into escape sequence parser. |
| Write | Forward keystrokes from editor input as raw bytes to PTY master fd. |
| Resize | On window geometry change, call `ioctl(TIOCSWINSZ)` and send `SIGWINCH`. |
| Cleanup | On terminal close, send `SIGHUP`, wait for process, reclaim fd. |

## Activation and keybindings

| Key | Action |
|---|---|
| `<leader>t` | Toggle default terminal |
| `<leader>tf` | Floating terminal window |
| `<leader>th` | Horizontal split terminal |
| `<leader>tv` | Vertical split terminal |
| `<leader>tn` | Create new named terminal |
| `Ctrl-\` | Quick toggle terminal |

## Terminal mode

When a terminal window is focused and in terminal-insert mode, keystrokes are forwarded to the PTY.

| Key | Action |
|---|---|
| `Ctrl-\ Ctrl-n` | Exit to Normal mode (terminal scrollback navigation) |
| `Esc Esc` | Exit to Normal mode |
| `Ctrl-w h/j/k/l` | Navigate to adjacent window |
| `Ctrl-w c` | Close terminal window |

## Scrollback navigation

In terminal Normal mode, the scrollback buffer is navigable with standard Vim motions (`j`, `k`, `Ctrl-u`, `Ctrl-d`, `G`, `gg`). Visual mode selection and yank from scrollback MUST work.

## Sending text

| Key | Action |
|---|---|
| `<leader>ts` | Send current line to terminal |
| `<leader>tS` | Send visual selection to terminal |
| `<leader>tc` | Prompt for command and send to terminal |

## Configuration

| Setting | Default | Description |
|---|---|---|
| `terminal.shell` | `$SHELL` or `/bin/sh` | Shell executable path |
| `terminal.scrollback_lines` | `10000` | Maximum scrollback lines |
| `terminal.size` | `15` | Default split size (rows or columns) |
| `terminal.float_size` | `0.8` | Float window size as fraction of editor |
| `terminal.start_insert` | `true` | Enter terminal-insert mode on open |
| `terminal.env` | `{}` | Additional environment variables for shell |

## Acceptance criteria

| Criterion | Requirement |
|---|---|
| Non-blocking | Noisy terminal output MUST NOT freeze editing in other windows. |
| Color fidelity | Programs using 256-color or 24-bit color MUST render correctly. |
| Resize correctness | Resizing the terminal window MUST update the PTY and re-render. |
| Process cleanup | Closing a terminal MUST terminate the child process. |
| Crash resilience | If a PTY process crashes, the terminal window MUST show an error, not freeze. |
| Wide character support | CJK characters in terminal output MUST render with correct width. |

## Related

- Terminal README: [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)
- Window management: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)
- Editor windows: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
