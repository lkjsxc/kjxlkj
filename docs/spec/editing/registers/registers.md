# Registers
Registers store text for yank/delete and for macros.

## Requirements
- Register updates are core-owned and transactional.
- Clipboard interaction is isolated behind a platform boundary; core state remains deterministic.
- Macros replay typed intents (not raw terminal bytes) to preserve reproducibility.

## Register set (normative)

| Register | Name | Access | Description |
|---|---|---|---|
| `"` | Unnamed | read/write | Default target for yank, delete, change. Always updated by those operations. |
| `0` | Yank | read-only | Holds the most recent yank (not delete). |
| `1`–`9` | Numbered | read-only | Rolling history: `1` is the most recent delete of 1+ lines, older entries shift to `2`–`9`. |
| `a`–`z` | Named (replace) | write | Explicitly addressed; contents are replaced on write. |
| `A`–`Z` | Named (append) | write | Maps to `a`–`z` but appends instead of replacing. |
| `-` | Small delete | read-only | Holds the most recent delete of less than one line. |
| `.` | Last insert | read-only | Text inserted in the most recent Insert-mode session. |
| `:` | Last command | read-only | Most recently executed ex-command string. |
| `%` | Current file | read-only | Relative path of the current buffer's file. |
| `+` | System clipboard | read/write | Platform clipboard (via `xclip`, `pbcopy`, `wl-copy`, etc.). |
| `*` | Primary selection | read/write | X11 primary selection; falls back to `+` on non-X11 platforms. |
| `_` | Black hole | write-only | Discards content; reads return empty string. |
| `/` | Last search | read-only | Most recent search pattern string. |
| `=` | Expression | special | Evaluates an expression (reserved; minimal in v1). |

## Clipboard integration (normative)

| Platform | Provider for `+` / `*` |
|---|---|
| macOS | `pbcopy` / `pbpaste` |
| Linux (X11) | `xclip -selection clipboard` / `xclip -selection primary` |
| Linux (Wayland) | `wl-copy` / `wl-paste` |
| Windows | `win32yank` or PowerShell `Set-Clipboard` / `Get-Clipboard` |
| SSH / headless | OSC 52 escape sequence |

The clipboard provider MUST be auto-detected at startup. Detection order: environment variable `DISPLAY` (X11), `WAYLAND_DISPLAY` (Wayland), platform heuristics. If no provider is found, `+` and `*` MUST silently behave like the unnamed register.

## Numbered register rotation (normative)

On a delete or change that removes one or more complete lines:

1. Contents of `9` are discarded.
2. Contents of `8` move to `9`, `7` to `8`, etc.
3. Contents of `1` move to `2`.
4. The deleted text is placed in `1`.

On a delete or change that removes less than one line:

1. The deleted text is placed in `-` (small delete register).
2. Numbered registers are NOT rotated.

Yank operations NEVER rotate numbered registers; they only update `0` and `"`.

## Register specification syntax

A register is specified by typing `"` followed by the register name character before an operator. Examples:

| Key sequence | Meaning |
|---|---|
| `"ayy` | Yank current line into register `a` |
| `"Ayy` | Append current line to register `a` |
| `"ap` | Put contents of register `a` after cursor |
| `"+p` | Put system clipboard after cursor |
| `"_dd` | Delete line without affecting any register |

## Put command details (normative)

| Command | Behavior |
|---|---|
| `p` | Put after cursor (characterwise) or below current line (linewise) |
| `P` | Put before cursor (characterwise) or above current line (linewise) |
| `gp` | Like `p` but cursor moves to end of pasted text |
| `gP` | Like `P` but cursor moves to end of pasted text |

The register content carries a type flag: `Characterwise` or `Linewise`. This flag determines whether `p`/`P` inserts inline or on a new line.

## Related

- Advanced editing (macros): [/docs/spec/editing/operators/advanced.md](/docs/spec/editing/operators/advanced.md)
- Input decoding (register prefix): [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
