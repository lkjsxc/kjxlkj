# Proposal: Full-specification Terminal Emulator

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem

The terminal emulator spec in [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) describes escape sequences at a high level but lacks the implementation-grade detail needed for a from-scratch VT100/xterm emulator. Previous reconstruction produced a simplistic terminal that did not handle edge cases.

## Required additions

### VT100 state machine

The escape sequence parser MUST be implemented as a finite state machine with explicit states and transitions. The recommended model follows the ANSI X3.64 / ECMA-48 parser topology:

| State | Description |
|---|---|
| Ground | Default state. Printable characters are written to the cell grid. |
| Escape | Entered on `0x1B`. Next byte selects sub-path. |
| EscapeIntermediate | Collecting intermediate bytes (`0x20-0x2F`) after ESC. |
| CsiEntry | Entered on `[` after ESC. Begins CSI parameter collection. |
| CsiParam | Collecting parameter bytes (`0x30-0x3B`). |
| CsiIntermediate | Collecting intermediate bytes in CSI sequence. |
| CsiIgnore | Discarding invalid CSI sequence. |
| OscString | Collecting OSC string content until ST or BEL. |
| DcsEntry | Device Control String entry. |
| DcsParam | DCS parameter collection. |
| DcsIntermediate | DCS intermediate bytes. |
| DcsPassthrough | DCS body passthrough. |
| DcsIgnore | Discarding invalid DCS. |
| SosPmApcString | Collecting SOS/PM/APC string content. |

### UTF-8 decoding in parser

The terminal parser MUST handle multi-byte UTF-8 input from the PTY. A UTF-8 decoder state machine MUST sit alongside the escape parser, accumulating bytes for multi-byte characters before passing complete grapheme data to the cell grid.

### CSI dispatch table

The implementation MUST dispatch on the CSI final byte:

| Final byte | Mnemonic | Parameters | Action |
|---|---|---|---|
| `A` | CUU | rows (default 1) | Move cursor up |
| `B` | CUD | rows (default 1) | Move cursor down |
| `C` | CUF | cols (default 1) | Move cursor right |
| `D` | CUB | cols (default 1) | Move cursor left |
| `E` | CNL | rows (default 1) | Move to next line start |
| `F` | CPL | rows (default 1) | Move to prev line start |
| `G` | CHA | col (default 1) | Move cursor to absolute column |
| `H` | CUP | row;col (default 1;1) | Move cursor to absolute position |
| `J` | ED | mode (0/1/2/3) | Erase display |
| `K` | EL | mode (0/1/2) | Erase line |
| `L` | IL | count (default 1) | Insert blank lines |
| `M` | DL | count (default 1) | Delete lines |
| `P` | DCH | count (default 1) | Delete characters |
| `S` | SU | rows (default 1) | Scroll up |
| `T` | SD | rows (default 1) | Scroll down |
| `X` | ECH | count (default 1) | Erase characters |
| `d` | VPA | row (default 1) | Move cursor to absolute row |
| `f` | HVP | row;col | Same as CUP |
| `h` | SM | modes | Set mode |
| `l` | RM | modes | Reset mode |
| `m` | SGR | attrs | Select graphic rendition |
| `r` | DECSTBM | top;bottom | Set scroll region |
| `s` | SCP | | Save cursor position |
| `t` | XTWINOPS | ops | Window manipulation |
| `u` | RCP | | Restore cursor position |

### Private mode table (`?` prefix)

| Mode | Name | Set action | Reset action |
|---|---|---|---|
| `1` | DECCKM | Application cursor keys | Normal cursor keys |
| `7` | DECAWM | Auto-wrap mode on | Auto-wrap off |
| `12` | Blink | Cursor blink on | Cursor blink off |
| `25` | DECTCEM | Show cursor | Hide cursor |
| `47` | Alt buffer | Switch to alternate screen | Switch to main screen |
| `1000` | Mouse button | Enable mouse button reporting | Disable |
| `1002` | Mouse motion | Enable mouse motion reporting | Disable |
| `1049` | Alt screen | Save cursor + switch to alt screen | Restore cursor + switch to main |
| `2004` | Bracketed paste | Enable bracketed paste mode | Disable |

## Acceptance criteria

- The escape parser handles all sequences in the CSI and private mode tables above.
- UTF-8 multi-byte characters from PTY output render correctly as grapheme clusters in the terminal cell grid.
- Wide characters (CJK) in terminal output occupy 2 cells with correct continuation marking.
- The parser never panics on malformed input; unrecognized sequences are silently discarded.

## Related

- Terminal spec: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
