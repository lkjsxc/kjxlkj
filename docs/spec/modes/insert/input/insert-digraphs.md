# Insert Digraphs

Back: [docs/spec/modes/insert/input/README.md](docs/spec/modes/insert/input/README.md)

Insert special characters via two-key combinations.

## Overview

Digraphs provide a way to enter characters that are
not on the keyboard by typing two ASCII characters
that visually represent the desired character.

## Entry Method

### Ctrl-k Sequence

1. Press `Ctrl-k` in Insert mode
2. Status line shows `?` to indicate digraph pending
3. Type two characters (the digraph code)
4. The resulting character is inserted

### Example Digraphs

| Code | Character | Description |
|------|-----------|-------------|
| `a:` | `a` | a with diaeresis |
| `o"` | `o` | o with diaeresis |
| `n~` | `n` | n with tilde |
| `Co` | (c) | Copyright sign |
| `Rg` | (R) | Registered sign |
| `TM` | TM | Trademark |
| `14` | 1/4 | One quarter |
| `12` | 1/2 | One half |
| `<<` | left-quote | Left guillemet |
| `>>` | right-quote | Right guillemet |
| `->` | right-arrow | Right arrow |
| `<-` | left-arrow | Left arrow |
| `!=` | not-equal | Not equal sign |
| `>=` | gte | Greater or equal |
| `<=` | lte | Less or equal |
| `Eu` | euro | Euro sign |
| `Pd` | pound | Pound sign |
| `Ye` | yen | Yen sign |

## Digraph Table

### Built-in Digraphs

The editor includes the standard RFC 1345 digraph
table. This provides coverage for Latin, Greek,
Cyrillic, and common symbols.

### Listing Digraphs

`:digraphs` displays all defined digraphs with
their code points and visual representations.

### Custom Digraphs

`:digraph {char1}{char2} {codepoint}` defines a
custom digraph. The code point is specified as a
decimal number.

## Reverse Lookup

### From Character to Digraph

`ga` (get ASCII) on a character shows its code point
and any digraph codes that produce it.

## CJK Digraphs

### Japanese Kana

Common kana digraphs follow the pattern:

| Code Pattern | Range |
|-------------|-------|
| `a5`-`n5` | Hiragana |
| `A5`-`N5` | Katakana |

### Chinese Pinyin Tones

Pinyin with tones can be entered via accent digraphs:
`a'` for first tone, `a!` for fourth tone, etc.

## Normal Mode Digraphs

Digraphs can also be entered in Replace mode using
the same `Ctrl-k` mechanism.

## Status Display

After pressing `Ctrl-k`, the status area shows `?`.
After typing the first digraph character, it shows
that character. After the second character, the
resulting character is inserted and the status clears.

## Error Handling

If the two-character combination does not match any
defined digraph, the second character is inserted
literally and no error is shown.

## Related

- Special characters: [docs/spec/modes/insert/input/insert-special-chars.md](docs/spec/modes/insert/input/insert-special-chars.md)
- Unicode handling: [docs/technical/unicode.md](docs/technical/unicode.md)
