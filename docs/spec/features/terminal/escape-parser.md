# Terminal Escape Sequence Parser

Back: [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)

Detailed specification of the VT100/xterm escape sequence state machine for the integrated terminal emulator.

## Parser state machine (normative)

The parser MUST be implemented as a finite state machine following the ANSI X3.64 / ECMA-48 model. The parser processes one byte at a time from the PTY output stream.

### States

| State | Description |
|---|---|
| `Ground` | Default. Printable characters (0x20-0x7E, multi-byte UTF-8) are written to the cell grid at the cursor position. Control characters (0x00-0x1F) are handled immediately. |
| `Escape` | Entered on ESC (0x1B). Next byte selects sub-path. |
| `EscapeIntermediate` | Accumulating intermediate bytes (0x20-0x2F) after ESC. |
| `CsiEntry` | Entered on `[` after ESC. Begins CSI parameter collection. Clears parameter buffer. |
| `CsiParam` | Accumulating parameter digits (0x30-0x39) and semicolons (0x3B). |
| `CsiIntermediate` | Accumulating intermediate bytes within a CSI sequence. |
| `CsiIgnore` | Discarding remainder of an invalid CSI sequence. |
| `OscString` | Accumulating OSC string content until ST (ESC `\`) or BEL (0x07). |
| `DcsEntry` | Device Control String entry. |
| `DcsParam` | DCS parameter accumulation. |
| `DcsPassthrough` | DCS body passthrough to a handler. |
| `DcsIgnore` | Discarding invalid DCS. |
| `SosPmApcString` | Accumulating SOS/PM/APC string content (ignored). |

### Key transitions

| From | Byte | To | Action |
|---|---|---|---|
| `Ground` | 0x1B | `Escape` | clear |
| `Ground` | 0x20-0x7E | `Ground` | print character |
| `Ground` | 0x80-0xBF (UTF-8 cont.) | `Ground` | feed UTF-8 decoder |
| `Ground` | 0xC0-0xFD (UTF-8 lead) | `Ground` | start UTF-8 sequence |
| `Escape` | `[` (0x5B) | `CsiEntry` | clear params |
| `Escape` | `]` (0x5D) | `OscString` | clear string |
| `Escape` | `P` (0x50) | `DcsEntry` | clear params |
| `Escape` | 0x20-0x2F | `EscapeIntermediate` | collect |
| `Escape` | 0x30-0x7E | `Ground` | esc_dispatch |
| `CsiEntry` | 0x30-0x39, 0x3B | `CsiParam` | param byte |
| `CsiEntry` | `?` (0x3F) | `CsiParam` | set private marker |
| `CsiEntry` | 0x40-0x7E | `Ground` | csi_dispatch |
| `CsiParam` | 0x30-0x39, 0x3B | `CsiParam` | param byte |
| `CsiParam` | 0x40-0x7E | `Ground` | csi_dispatch |
| `CsiParam` | 0x20-0x2F | `CsiIntermediate` | collect |
| `OscString` | 0x07 (BEL) | `Ground` | osc_dispatch |
| `OscString` | 0x1B, then `\` | `Ground` | osc_dispatch (ST) |

## CSI dispatch table (normative)

The `csi_dispatch` action MUST dispatch based on the final byte and optional private marker (`?`).

| Final | Mnemonic | Parameters | Action |
|---|---|---|---|
| `A` | CUU | rows (default 1) | Move cursor up N rows |
| `B` | CUD | rows (default 1) | Move cursor down N rows |
| `C` | CUF | cols (default 1) | Move cursor right N cols |
| `D` | CUB | cols (default 1) | Move cursor left N cols |
| `E` | CNL | rows (default 1) | Move to beginning of Nth line down |
| `F` | CPL | rows (default 1) | Move to beginning of Nth line up |
| `G` | CHA | col (default 1) | Move to absolute column (1-based) |
| `H` | CUP | row;col (default 1;1) | Move to absolute position (1-based) |
| `J` | ED | mode | Erase in display (0: below, 1: above, 2: all, 3: all+scrollback) |
| `K` | EL | mode | Erase in line (0: right, 1: left, 2: whole) |
| `L` | IL | count (default 1) | Insert N blank lines at cursor |
| `M` | DL | count (default 1) | Delete N lines at cursor |
| `P` | DCH | count (default 1) | Delete N characters at cursor |
| `S` | SU | rows (default 1) | Scroll up N rows |
| `T` | SD | rows (default 1) | Scroll down N rows |
| `X` | ECH | count (default 1) | Erase N characters at cursor |
| `@` | ICH | count (default 1) | Insert N blank characters at cursor |
| `d` | VPA | row (default 1) | Move to absolute row (1-based) |
| `f` | HVP | row;col | Same as CUP |
| `h` | SM | modes | Set mode (with `?`: private mode) |
| `l` | RM | modes | Reset mode (with `?`: private mode) |
| `m` | SGR | attrs | Select graphic rendition |
| `n` | DSR | type | Device status report |
| `r` | DECSTBM | top;bottom | Set scrolling region (1-based, inclusive) |
| `s` | SCP | (none) | Save cursor position |
| `t` | XTWINOPS | ops | Window manipulation (report size) |
| `u` | RCP | (none) | Restore cursor position |

## Private mode table (`?` prefix) (normative)

| Mode | Name | Set action | Reset action |
|---|---|---|---|
| 1 | DECCKM | Application cursor keys | Normal cursor keys |
| 7 | DECAWM | Auto-wrap on | Auto-wrap off |
| 12 | ATT610 | Cursor blink on | Cursor blink off |
| 25 | DECTCEM | Show cursor | Hide cursor |
| 47 | Alternate buffer | Switch to alt screen (no save) | Switch to main |
| 1000 | X11 mouse | Enable button-event mouse | Disable |
| 1002 | Cell motion | Enable cell motion mouse | Disable |
| 1006 | SGR mouse | Enable SGR mouse format | Disable |
| 1049 | Alt screen | Save cursor + switch to alt screen | Restore + switch to main |
| 2004 | Bracketed paste | Enable bracketed paste mode | Disable |

Mouse reporting modes (1000, 1002, 1006) MUST be tracked but MAY be ignored (mouse input is not supported). The mode state MUST still be maintained so DECRST properly clears it.

## SGR parameter parsing (normative)

SGR (`CSI ... m`) parameters are semicolon-separated. Each parameter modifies character attributes.

| Parameter(s) | Action |
|---|---|
| 0 | Reset all attributes to default |
| 1 | Set bold |
| 2 | Set dim (faint) |
| 3 | Set italic |
| 4 | Set underline |
| 7 | Set reverse video |
| 8 | Set hidden (invisible) |
| 9 | Set strikethrough |
| 22 | Reset bold and dim |
| 23 | Reset italic |
| 24 | Reset underline |
| 27 | Reset reverse |
| 29 | Reset strikethrough |
| 30-37 | Set foreground (basic 8) |
| 38 | Set foreground extended (next params: `5;N` for 256, `2;R;G;B` for RGB) |
| 39 | Reset foreground to default |
| 40-47 | Set background (basic 8) |
| 48 | Set background extended |
| 49 | Reset background to default |
| 90-97 | Set foreground (bright 8) |
| 100-107 | Set background (bright 8) |

## UTF-8 decoding (normative)

The parser MUST maintain a UTF-8 accumulation buffer. When a lead byte (0xC0-0xFD) arrives in Ground state, subsequent continuation bytes (0x80-0xBF) are accumulated until a complete code point is formed. The code point is then passed through grapheme cluster segmentation and display width computation before being written to the cell grid.

Invalid UTF-8 sequences MUST be replaced with U+FFFD (replacement character).

## Unrecognized sequences

The parser MUST NOT panic on any input. Unrecognized escape sequences, CSI sequences with unknown final bytes, and malformed parameter lists MUST be silently discarded. The parser returns to `Ground` state.

## Related

- Terminal emulator: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Terminal README: [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)
- Crate: `kjxlkj-service-terminal` in [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
