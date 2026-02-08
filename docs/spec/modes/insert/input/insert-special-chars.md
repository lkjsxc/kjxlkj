# Insert Special Characters

Back: [docs/spec/modes/insert/input/README.md](/docs/spec/modes/insert/input/README.md)

Methods for inserting characters not directly typeable.

## Overview

Special characters include control characters, Unicode
code points outside the keyboard layout, and characters
with special meaning in the editor that need literal
insertion. Two primary methods exist: literal insert
(`Ctrl-v`) and digraphs (`Ctrl-k`).

## Literal Insert

### Ctrl-v Key Sequence

After pressing `Ctrl-v`, the next keystroke is inserted
literally, bypassing all mapping and command processing.

### Numeric Entry

| Sequence | Range | Description |
|----------|-------|-------------|
| `Ctrl-v {ddd}` | 0-255 | Decimal byte value (3 digits) |
| `Ctrl-v o{ooo}` | 0-377 | Octal byte value (3 digits) |
| `Ctrl-v x{hh}` | 00-FF | Hex byte value (2 digits) |
| `Ctrl-v u{hhhh}` | 0000-FFFF | Unicode BMP (4 hex digits) |
| `Ctrl-v U{hhhhhhhh}` | Full range | Unicode (8 hex digits) |

### Digit Entry Rules

For decimal entry, typing begins after `Ctrl-v`.
Entry completes when:
- 3 digits have been typed (decimal)
- 3 digits have been typed (octal, after `o`)
- 2 digits have been typed (hex, after `x`)
- 4 digits have been typed (unicode, after `u`)
- 8 digits have been typed (unicode, after `U`)
- A non-digit character is typed (early termination)

Early termination: if fewer than the maximum digits
are typed and the next key is not a valid digit,
the character is formed from the digits entered so
far, and the non-digit key is processed normally.

### Invalid Code Points

| Input | Result |
|-------|--------|
| `Ctrl-v u D800` | Error: surrogate pair not allowed |
| `Ctrl-v U 110000` | Error: beyond Unicode maximum |
| `Ctrl-v 0` | Inserts `<Nul>` (displayed as `^@`) |

### Special Keys After Ctrl-v

| Key | Inserted Character |
|-----|-------------------|
| `Esc` | Literal 0x1B (displayed as `^[`) |
| `Enter` | Literal 0x0D (displayed as `^M`) |
| `Tab` | Literal 0x09 (always tab, ignores expandtab) |
| `Backspace` | Literal 0x08 (displayed as `^H`) |
| `Ctrl-M` | Literal 0x0D |
| `Ctrl-J` | Literal 0x0A (line feed) |

## Status Display

### Visual Feedback

After pressing `Ctrl-v`, the status area MUST display
`^` to indicate literal-input-pending state. As digits
are typed, they are echoed beside the `^`. Once the
character is inserted, the indicator disappears.

### Display of Non-Printable Characters

Non-printable characters are shown in the buffer
using caret notation (`^X`) or hex notation (`<xx>`).

| Range | Display |
|-------|---------|
| 0x00-0x1F | `^@` through `^_` |
| 0x7F | `^?` |
| 0x80-0x9F | `<80>` through `<9f>` |
| Other non-printable | `<xx>` hex notation |

## Digraph Method

### Ctrl-k Sequence

`Ctrl-k {char1}{char2}` inserts a digraph character
defined by the two-character combination. This is
separate from the `Ctrl-v` literal insert method.

See digraphs spec for the full digraph table:
[docs/spec/modes/insert/input/insert-digraphs.md](/docs/spec/modes/insert/input/insert-digraphs.md)

## Bracketed Paste

### Behavior

When the terminal sends a bracketed paste sequence,
text is inserted literally without triggering any
mappings or abbreviations. This is equivalent to
each character being individually `Ctrl-v`-inserted.

### Detection

Bracketed paste is detected by the escape sequences
`\e[200~` (start) and `\e[201~` (end). The editor
enables bracketed paste mode on terminal startup.

## CJK Considerations

### Unicode Entry for CJK

CJK characters are commonly entered via `Ctrl-v u`
with a 4-digit hex code. For example:
- `Ctrl-v u 3042` inserts `あ` (Hiragana A)
- `Ctrl-v u 4e2d` inserts `中` (CJK middle)

### Display Width

CJK characters inserted via any method occupy 2
display columns. The cursor advances by 2 columns
after insertion.

## Related

- Insert digraphs: [docs/spec/modes/insert/input/insert-digraphs.md](/docs/spec/modes/insert/input/insert-digraphs.md)
- Unicode handling: [docs/technical/unicode.md](/docs/technical/unicode.md)
- Literal insert: [docs/spec/modes/insert/input/insert-literal.md](/docs/spec/modes/insert/input/insert-literal.md)
