# Sentence and Paragraph Motions

Navigate by text structure units.

## Sentence Motions

| Motion | Description | Type |
|---|---|---|
| `)` | Forward to next sentence start | Exclusive |
| `(` | Backward to previous sentence start | Exclusive |

### Sentence Definition

A sentence ends at `.`, `!`, or `?` followed by:

- End of line, OR
- At least one space or tab

Optionally, closing characters `)`, `]`, `"`, `'` may appear between the punctuation and the whitespace/EOL.

A blank line also ends a sentence.

### Sentence Text Objects

| Object | Description |
|---|---|
| `is` | Inner sentence (no trailing whitespace) |
| `as` | A sentence (includes trailing whitespace) |

## Paragraph Motions

| Motion | Description | Type |
|---|---|---|
| `}` | Forward to next paragraph boundary (blank line) | Exclusive |
| `{` | Backward to previous paragraph boundary | Exclusive |

### Paragraph Definition

Paragraphs are separated by blank lines (lines containing only whitespace). In code, blank lines between function definitions act as paragraph boundaries.

### Paragraph Text Objects

| Object | Description |
|---|---|
| `ip` | Inner paragraph (lines, excluding surrounding blank lines) |
| `ap` | A paragraph (includes trailing blank lines) |

## Section Motions

| Motion | Description |
|---|---|
| `]]` | Forward to next section start (`{` at column 0) |
| `[[` | Backward to previous section start |
| `][` | Forward to next section end (`}` at column 0) |
| `[]` | Backward to previous section end |

Sections are defined by `{` or `}` at column 0, which typically correspond to C/Rust function boundaries.

## Operator Interaction

All sentence/paragraph motions work with operators:

| Example | Effect |
|---|---|
| `d)` | Delete to next sentence start |
| `dap` | Delete entire paragraph including trailing blanks |
| `gqap` | Format paragraph to `textwidth` |
| `y{` | Yank to previous paragraph boundary |

## Related

- Word motions: [/docs/spec/editing/motions/word-WORD.md](/docs/spec/editing/motions/word-WORD.md)
- Motion grammar: [/docs/spec/editing/motions/motion-grammar.md](/docs/spec/editing/motions/motion-grammar.md)
