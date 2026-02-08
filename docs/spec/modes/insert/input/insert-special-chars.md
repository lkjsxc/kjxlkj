# Insert Special Characters

Back: [/docs/spec/modes/insert/input/README.md](/docs/spec/modes/insert/input/README.md)

Methods for inserting special and non-printable characters.

## Overview

Insert mode provides several mechanisms for entering characters that cannot be typed directly.

## Literal Insert

`<C-v>` followed by a key inserts the raw character code. See insert-literal for numeric code entry.

## Digraph Method

`<C-k>{char1}{char2}` inserts a digraph (two-character mnemonic for special characters). Example: `<C-k>Co` inserts `©`.

## Bracketed Paste

When bracketed paste is active (modern terminals), pasted text is handled specially — no auto-indent, no abbreviation expansion, no mapping processing.

## CJK Considerations

CJK input methods are supported via the terminal's IME integration. The editor handles composed characters as grapheme clusters.

## Related

- Insert literal: [/docs/spec/modes/insert/input/insert-literal.md](/docs/spec/modes/insert/input/insert-literal.md)
- Digraphs: [/docs/spec/editing/text-manipulation/digraphs.md](/docs/spec/editing/text-manipulation/digraphs.md)
- Insert mode: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
