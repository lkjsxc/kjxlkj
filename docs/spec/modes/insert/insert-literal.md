# Insert Literal

Back: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)

Insert literal characters or special codes in insert mode.

## Overview

`<C-v>` in insert mode allows inserting characters by their code point or inserting key codes literally without triggering mappings.

## Character by Code

| Sequence | Description |
|---|---|
| `<C-v>{decimal}` | Insert character by decimal code (0-255) |
| `<C-v>o{octal}` | Insert character by octal code |
| `<C-v>x{hex}{hex}` | Insert character by hex byte (00-FF) |
| `<C-v>u{hex}{hex}{hex}{hex}` | Insert Unicode by 4-digit hex |
| `<C-v>U{8 hex digits}` | Insert Unicode by 8-digit hex |

## Literal Key Insert

`<C-v>` followed by a special key inserts the key's terminal code literally:

| Sequence | Inserts |
|---|---|
| `<C-v><Esc>` | Literal escape byte (0x1B) |
| `<C-v><CR>` | Literal carriage return |
| `<C-v><Tab>` | Literal tab (even with expandtab) |

## Digraph Alternative

`<C-k>{char1}{char2}` inserts a digraph. See digraph documentation.

## Display

Non-printable characters are displayed using special notation (e.g., `^[` for escape, `<80>` for byte 0x80).

## Use Cases

| Scenario | Command |
|---|---|
| Insert literal tab | `<C-v><Tab>` |
| Insert Unicode emoji | `<C-v>U0001F600` |
| Insert control character | `<C-v><C-a>` (inserts byte 0x01) |
| Escape in string | `<C-v><Esc>` |

## Related

- Insert mode: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
- Digraphs: [/docs/spec/features/editing/digraphs.md](/docs/spec/features/editing/digraphs.md)
- Unicode: [/docs/technical/unicode.md](/docs/technical/unicode.md)
