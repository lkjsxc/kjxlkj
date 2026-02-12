# Clipboard Integration

System clipboard integration across platforms.

## Clipboard Registers (normative)

| Register | Description |
|---|---|
| `"+` | System clipboard (Ctrl+C/V equivalent) |
| `"*` | Primary selection (X11 middle-click; aliased to `"+` on non-X11) |

### Usage

- `"+y{motion}` — yank to system clipboard
- `"+p` — paste from system clipboard
- `"*y{motion}` — yank to primary selection
- `"*p` — paste from primary selection

## Configuration

Clipboard behavior settings.

### Sync with Unnamed Register

When `clipboard = "unnamedplus"` is set, all yank/delete/change operations use `"+` as the default register instead of `""`. This makes `y`, `d`, `p` work directly with the system clipboard.

| Setting | Effect |
|---|---|
| `clipboard = "unnamed"` | Sync `""` with `"*` |
| `clipboard = "unnamedplus"` | Sync `""` with `"+` |
| (default) | No sync; clipboard requires explicit `"+`/`"*` |

## Platform Detection (normative)

kjxlkj auto-detects the clipboard provider at startup in this priority order:

1. **Wayland**: `wl-copy` / `wl-paste` (from wl-clipboard package)
2. **X11**: `xclip` or `xsel` (prefers xclip)
3. **macOS**: `pbcopy` / `pbpaste` (built-in)
4. **Windows**: Win32 clipboard API (built-in)
5. **OSC 52**: Terminal escape sequence (fallback for SSH/containers)
6. **Internal**: In-memory clipboard (no system integration)

If the preferred provider fails at runtime, fallback to the next level.

## OSC 52 Terminal Clipboard

OSC 52 enables clipboard access through the terminal emulator itself, working over SSH and inside containers without needing X11 forwarding.

### Supported Terminals

Kitty, Alacritty, iTerm2, WezTerm, Windows Terminal, foot, xterm (with allowWindowOps).

### Configuration

Enable with `clipboard_osc52 = true`. When enabled, OSC 52 is used alongside or instead of the platform provider.

## X11 Selection Types

| Selection | Behavior |
|---|---|
| Primary (`"*`) | Set automatically when text is selected; paste with middle-click |
| Clipboard (`"+`) | Set by explicit copy action (Ctrl+C or yank command) |

On Wayland and macOS, primary and clipboard are identical.

## Large Content Handling

Content exceeding 1 MB is transferred in chunks. Extremely large clipboard content (> 10 MB) is rejected with a warning to prevent memory issues.

## Security

Paste from external clipboard in command-line mode shows a confirmation prompt when content contains newlines (prevents command injection). Configurable via `clipboard_paste_confirm = true`.

## Commands

| Command | Description |
|---|---|
| `:clipboard` | Show clipboard register content |
| `:clipboard clear` | Clear clipboard registers |

## Insert Mode Paste

| Key | Action |
|---|---|
| `<C-r>+` | Insert from system clipboard |
| `<C-r>*` | Insert from primary selection |

## Troubleshooting

- **Clipboard empty**: Verify provider is installed (`xclip`, `wl-clipboard`)
- **SSH**: Enable OSC 52 in config and verify terminal supports it
- **tmux**: Set `set -g set-clipboard on` in tmux config

## Related

- Clipboard registers: [/docs/spec/editing/registers/clipboard-registers.md](/docs/spec/editing/registers/clipboard-registers.md)
- Registers overview: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)
