# Bracket Matching

Navigate matching pairs with `%`.

## Overview

The `%` motion jumps between matching bracket pairs. It also works as an operator motion (e.g., `d%` deletes to the matching bracket).

## Supported Pairs (normative)

| Open | Close |
|---|---|
| `(` | `)` |
| `[` | `]` |
| `{` | `}` |

## Behavior (normative)

1. If the cursor is on a bracket character, `%` jumps to its matching partner.
2. If the cursor is NOT on a bracket, `%` searches forward on the current line for the first bracket character, then jumps to its match.
3. The match search respects nesting: `({})` on the outer `(` jumps to the outer `)`.

## Operator Compatibility

`%` is an inclusive characterwise motion. It works with all operators:

| Example | Effect |
|---|---|
| `d%` | Delete from cursor to matching bracket (inclusive) |
| `y%` | Yank to matching bracket |
| `c%` | Change to matching bracket |
| `v%` | Visual select to matching bracket |

## Bracket Text Objects (normative)

| Object | Description |
|---|---|
| `i(` / `ib` | Inside parentheses |
| `a(` / `ab` | Around parentheses (including parens) |
| `i[` | Inside square brackets |
| `a[` | Around square brackets |
| `i{` / `iB` | Inside curly braces |
| `a{` / `aB` | Around curly braces |
| `i<` | Inside angle brackets |
| `a<` | Around angle brackets |

## Match Highlighting

When `showmatch` is enabled, briefly highlight the matching bracket when a closing bracket is typed in Insert mode. The highlight duration is controlled by `matchtime` (in tenths of a second, default 5).

## Jump List

`%` is a jump command: it adds the pre-jump position to the jump list.

## Related

- Motions: [/docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)
- Text objects: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)
