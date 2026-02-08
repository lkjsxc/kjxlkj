# Insert Special Characters

Back: [/docs/spec/modes/insert/input/README.md](/docs/spec/modes/insert/input/README.md)

Methods for inserting characters that cannot be typed directly.

## Literal insert (normative)

| Key | Action |
|---|---|
| `Ctrl-v` | Insert the next key literally (not interpreted as a command) |
| `Ctrl-v {decimal}` | Insert character by decimal code (3 digits, 0-255) |
| `Ctrl-v o{octal}` | Insert character by octal code (3 digits) |
| `Ctrl-v x{hex}` | Insert character by 2-digit hex code |
| `Ctrl-v u{hex4}` | Insert Unicode character by 4-digit hex code (BMP) |
| `Ctrl-v U{hex8}` | Insert Unicode character by 8-digit hex code (full Unicode) |

## Literal key behavior

After `Ctrl-v`, the very next keystroke is inserted as a literal character. This includes:

| Key | Inserted as |
|---|---|
| `Esc` | Literal escape byte (0x1B) |
| `Enter` | Literal carriage return (0x0D) |
| `Tab` | Literal tab (0x09), even when `expandtab` is set |
| `Ctrl-M` | Literal 0x0D |

## Status display

After pressing `Ctrl-v`, the status line MUST show `^` to indicate literal-input pending state. After the character is inserted, the `^` disappears.

## Digraph method

`Ctrl-k {char1}{char2}` inserts a digraph. See [/docs/spec/modes/insert/input/insert-digraphs.md](/docs/spec/modes/insert/input/insert-digraphs.md).

## Related

- Insert digraphs: [/docs/spec/modes/insert/input/insert-digraphs.md](/docs/spec/modes/insert/input/insert-digraphs.md)
- Insert Unicode: [/docs/spec/modes/insert/input/insert-unicode.md](/docs/spec/modes/insert/input/insert-unicode.md)
- Unicode guidance: [/docs/technical/unicode.md](/docs/technical/unicode.md)

