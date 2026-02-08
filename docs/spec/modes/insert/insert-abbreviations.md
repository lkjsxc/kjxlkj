# Insert Mode Abbreviations

Back: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)

Abbreviations expand short text into longer replacements when triggered.

## Overview

Abbreviations are defined with `:abbreviate` (or `:ab`). When a trigger character (space, punctuation, `<Esc>`) is typed after an abbreviation keyword, the keyword is replaced with the expansion.

## Definition Commands

| Command | Description |
|---|---|
| `:ab[breviate] {lhs} {rhs}` | Define abbreviation: typing `{lhs}` expands to `{rhs}` |
| `:ia[bbrev] {lhs} {rhs}` | Insert-mode only abbreviation |
| `:ca[bbrev] {lhs} {rhs}` | Command-line mode only abbreviation |

## Removing Abbreviations

| Command | Description |
|---|---|
| `:una[bbreviate] {lhs}` | Remove abbreviation |
| `:iuna[bbrev] {lhs}` | Remove insert-mode abbreviation |
| `:abc[lear]` | Remove all abbreviations |

## Trigger

Abbreviations are triggered when a non-keyword character is typed after the abbreviation text. Trigger characters include: space, `<CR>`, `.`, `,`, `;`, `)`, `<Esc>`.

The abbreviation must be preceded by a non-keyword character or be at the start of the line.

## Types

| Type | Rule | Example |
|---|---|---|
| Full-id | All keyword characters | `teh` → `the` |
| End-id | Last character is keyword, not first | `#i` → `#include` |
| Non-id | Last character is not keyword | N/A |

## Matching

Abbreviations are matched case-sensitively. `Teh` does not trigger an abbreviation for `teh`.

The match requires a word boundary before the abbreviation text.

## Expression Abbreviations

| Command | Description |
|---|---|
| `:iab <expr> {lhs} {expr}` | The `{expr}` is evaluated at trigger time |

Expression abbreviations allow dynamic expansion (e.g., inserting current date).

## Special Keys

Key notation can be used in the right-hand side:

| Notation | Key |
|---|---|
| `<CR>` | Enter/newline |
| `<Tab>` | Tab |
| `<BS>` | Backspace |
| `<C-o>` | Insert-normal escape |

## Listing

| Command | Description |
|---|---|
| `:ab` | List all abbreviations |
| `:iab` | List insert-mode abbreviations |
| `:cab` | List command-line abbreviations |

Output shows the mode flag, LHS, and RHS.

## Buffer-Local

| Command | Description |
|---|---|
| `:iab <buffer> {lhs} {rhs}` | Abbreviation only for current buffer |

Buffer-local abbreviations take priority over global ones.

## No-Remap

| Command | Description |
|---|---|
| `:inoreab {lhs} {rhs}` | Non-recursive abbreviation |

Non-recursive abbreviations do not trigger further abbreviation expansion inside the replacement text.

## Undo

`<C-]>` triggers abbreviation expansion without inserting a character. Pressing `u` after expansion undoes the expansion and restores the original text.

## Related

- Insert mode: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
- Completion abbreviations: [/docs/spec/modes/insert/completion/insert-abbreviations.md](/docs/spec/modes/insert/completion/insert-abbreviations.md)
