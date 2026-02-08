# Clipboard Registers

System clipboard integration with `+` and `*` registers.

## Register Overview (normative)

| Register | System | Description |
|---|---|---|
| `"+` | Clipboard | System clipboard (Ctrl+C/V) |
| `"*` | Selection | X11 primary selection (middle-click paste) |

On macOS and Windows, both registers reference the same system clipboard.

## Platform Behavior

### Linux (X11/Wayland)

`"+` uses the system clipboard (via `xclip`/`xsel` or `wl-copy`/`wl-paste`). `"*` uses the X11 primary selection.

### macOS

Both `"+` and `"*` use `pbcopy`/`pbpaste`.

### Windows

Both `"+` and `"*` use the Win32 clipboard API.

## Usage

| Command | Effect |
|---|---|
| `"+y{motion}` | Yank to system clipboard |
| `"+p` | Paste from system clipboard |
| `"+d{motion}` | Delete to system clipboard |
| `"*y{motion}` | Yank to primary selection |
| `"*p` | Paste from primary selection |

## Insert Mode

| Key | Action |
|---|---|
| `<C-r>+` | Insert system clipboard contents |
| `<C-r>*` | Insert primary selection contents |

## Command Line

`<C-r>+` inserts clipboard contents on the command line.

## Sync with Unnamed Register

When `clipboard = "unnamedplus"`, all default yank/delete/put operations use `"+` automatically. See [/docs/spec/features/editing/clipboard.md](/docs/spec/features/editing/clipboard.md).

## X11 Selection vs Clipboard

| Feature | Selection (`"*`) | Clipboard (`"+`) |
|---|---|---|
| Set by | Mouse selection, visual yank | Explicit copy |
| Paste by | Middle-click | Ctrl+V |
| Lifetime | Until selection lost | Until overwritten |

## Related

- Clipboard integration: [/docs/spec/features/editing/clipboard.md](/docs/spec/features/editing/clipboard.md)
- Registers overview: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)
