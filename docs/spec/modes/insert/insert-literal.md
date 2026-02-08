# Insert Literal Characters

Insert characters by code point value.

## Overview

`<C-v>` in insert mode begins literal character insertion.
This allows entering characters that are normally interpreted
as commands, or entering characters by their numeric code.

## Decimal Entry

### Syntax

`<C-v>{decimal}` where `{decimal}` is 1-3 digits (0-255).

### Behavior

1. Press `<C-v>`
2. Type up to 3 decimal digits
3. Character with that byte value is inserted

### Example

`<C-v>065` inserts `A` (ASCII 65).
A single digit like `<C-v>9` inserts tab (character 9).

## Octal Entry

### Syntax

`<C-v>o{octal}` where `{octal}` is 1-3 octal digits.

### Range

Valid range: `o0` to `o377` (0-255 decimal).

## Hex Entry

### Two-digit Hex

`<C-v>x{hex}{hex}` inserts a byte (0x00-0xFF).

### Four-digit Hex (Unicode BMP)

`<C-v>u{hex}{hex}{hex}{hex}` inserts a Unicode character
from the Basic Multilingual Plane (U+0000 to U+FFFF).

### Eight-digit Hex (Full Unicode)

`<C-v>U{hex}{hex}{hex}{hex}{hex}{hex}{hex}{hex}` inserts
any Unicode code point (U+00000000 to U+0010FFFF).

## Special Keys

### Literal Control Characters

`<C-v><C-a>` inserts a literal control-A (0x01).
`<C-v><Esc>` inserts a literal escape (0x1B).

### Terminal Keys

| Sequence | Inserts |
|----------|---------|
| `<C-v><Enter>` | Literal carriage return |
| `<C-v><Tab>` | Literal tab (even when expandtab is set) |
| `<C-v><BS>` | Literal backspace character |

### Function Keys

`<C-v><F1>` inserts the raw escape sequence for F1.

## Display

### Non-printable Characters

Non-printable characters are shown with special notation:
- Control characters: `^A` through `^Z`, `^[`, `^\`, `^]`, `^^`, `^_`
- Characters 128-159: `~@` through `~_`
- Character 127 (DEL): `^?`

### Hex Display

Characters without a displayable form show as `<xx>`
where `xx` is the hex value.

### Unicode Display

Multi-byte Unicode characters display normally if the
terminal supports them.

## Digraphs

### Overview

`<C-k>{char1}{char2}` inserts a digraph: a character
defined by a two-character combination.

### Common Digraphs

| Digraph | Character | Description |
|---------|-----------|-------------|
| `Co` | `©` | Copyright |
| `Rg` | `®` | Registered |
| `DG` | `°` | Degree |
| `+-` | `±` | Plus-minus |
| `12` | `½` | One half |
| `<<` | `«` | Left guillemet |
| `>>` | `»` | Right guillemet |
| `Eu` | `€` | Euro sign |
| `a:` | `ä` | a-umlaut |
| `e'` | `é` | e-acute |

### Listing Digraphs

`:digraphs` shows all defined digraphs.
`:digraphs {char1}{char2}` shows a specific digraph.

### Custom Digraphs

`:digraph {char1}{char2} {decimal}` defines a new digraph.

## Paste Mode

### Bracketed Paste

When bracketed paste mode is enabled, pasted text is
inserted literally without triggering abbreviations,
mappings, or auto-indentation.

### Detection

The terminal sends `ESC[200~` before pasted text and
`ESC[201~` after. The editor detects these sequences
and enters paste mode automatically.

## CJK Considerations

### Wide Characters

CJK characters entered via `<C-v>u` have display width 2.
The cursor and column tracking must account for this.

### Input Method

When an input method (IM) is active, `<C-v>` still works
for literal entry. The IM is temporarily bypassed.

## Statusline Indicator

### During Entry

While waiting for digit input after `<C-v>`, the
statusline shows `^` followed by digits typed so far.

### Completion

The indicator disappears once the character is inserted.
