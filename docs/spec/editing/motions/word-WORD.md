# Word vs WORD Distinction

Two word boundary definitions for motions and text objects.

## word (lowercase)

A **word** is a sequence of letters (`a-z`, `A-Z`), digits (`0-9`), and underscores (`_`), OR a sequence of other non-blank characters. Sequences of different classes form separate words.

For CJK text, each CJK ideograph is treated as a separate word.

### word Motions

| Motion | Description | Type |
|---|---|---|
| `w` | Forward to next word start | Exclusive |
| `e` | Forward to current/next word end | Inclusive |
| `b` | Backward to word start | Exclusive |
| `ge` | Backward to previous word end | Inclusive |

### word Text Objects

| Object | Description |
|---|---|
| `iw` | Inner word (word characters only) |
| `aw` | A word (includes trailing/leading whitespace) |

## WORD (uppercase)

A **WORD** is a sequence of non-blank characters. Only spaces, tabs, and newlines separate WORDs.

### WORD Motions

| Motion | Description | Type |
|---|---|---|
| `W` | Forward to next WORD start | Exclusive |
| `E` | Forward to current/next WORD end | Inclusive |
| `B` | Backward to WORD start | Exclusive |
| `gE` | Backward to previous WORD end | Inclusive |

### WORD Text Objects

| Object | Description |
|---|---|
| `iW` | Inner WORD (non-blank characters) |
| `aW` | A WORD (includes surrounding whitespace) |

## Comparison

Given the text: `func(arg1, arg2)`

- **words**: `func`, `(`, `arg1`, `,`, `arg2`, `)` — 6 words
- **WORDs**: `func(arg1,`, `arg2)` — 2 WORDs

Given a path: `/home/user/.config`

- **words**: `/`, `home`, `/`, `user`, `/`, `.`, `config` — 7 words
- **WORDs**: `/home/user/.config` — 1 WORD

## When to Use Each

- **word** (`w`, `e`, `b`): Editing individual identifiers, navigating code tokens
- **WORD** (`W`, `E`, `B`): Navigating paths, URLs, qualified names, moving faster through code

## Empty Line Behavior

An empty line is treated as a word boundary. `w` and `W` both stop at empty lines.

## Related

- Motion grammar: [/docs/spec/editing/motions/motion-grammar.md](/docs/spec/editing/motions/motion-grammar.md)
- Text objects overview: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)
