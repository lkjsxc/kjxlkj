# Text Objects
Text objects define a region for an operator to act upon.

## Requirements
- Text object selection is deterministic and based on buffer snapshots.
- Language-aware text objects (e.g., function, class) may exist but must be async-backed and cached.

## Core text objects (normative)

### Word objects

| Key | Object | Description |
|---|---|---|
| `iw` | Inner word | Select the word under the cursor (no surrounding whitespace). |
| `aw` | A word | Select the word under the cursor plus trailing whitespace (or leading if at end of line). |
| `iW` | Inner WORD | Select the WORD under the cursor (whitespace-delimited). |
| `aW` | A WORD | Select the WORD plus surrounding whitespace. |

### Quote objects

| Key | Object | Description |
|---|---|---|
| `i"` | Inner double-quote | Text between the nearest enclosing `"` pair on the current line. |
| `a"` | A double-quote | Text between `"` pair including the quotes themselves. |
| `i'` | Inner single-quote | Text between the nearest enclosing `'` pair on the current line. |
| `a'` | A single-quote | Text between `'` pair including the quotes. |
| `` i` `` | Inner backtick | Text between backtick pair. |
| `` a` `` | A backtick | Text between backtick pair including backticks. |

### Bracket/brace objects

| Key | Object | Description |
|---|---|---|
| `i(` / `i)` / `ib` | Inner parentheses | Text between `(` and `)`, excluding the delimiters. |
| `a(` / `a)` / `ab` | A parentheses | Text between `(` and `)`, including the delimiters. |
| `i[` / `i]` | Inner brackets | Text between `[` and `]`, excluding. |
| `a[` / `a]` | A brackets | Text between `[` and `]`, including. |
| `i{` / `i}` / `iB` | Inner braces | Text between `{` and `}`, excluding. |
| `a{` / `a}` / `aB` | A braces | Text between `{` and `}`, including. |
| `i<` / `i>` | Inner angle brackets | Text between `<` and `>`, excluding. |
| `a<` / `a>` | A angle brackets | Text between `<` and `>`, including. |

### Paragraph/sentence objects

| Key | Object | Description |
|---|---|---|
| `ip` | Inner paragraph | Contiguous non-blank lines. |
| `ap` | A paragraph | Like `ip` plus trailing blank lines. |
| `is` | Inner sentence | The sentence under the cursor. |
| `as` | A sentence | The sentence plus trailing whitespace. |

### Tag objects

| Key | Object | Description |
|---|---|---|
| `it` | Inner tag | Content between matched HTML/XML open and close tags. |
| `at` | A tag | Content including the open and close tags themselves. |

## Bracket matching algorithm (normative)

For bracket text objects, the algorithm MUST:

1. Search outward from the cursor position to find the nearest enclosing matching pair.
2. Handle nested pairs correctly by tracking bracket depth.
3. For `i(` with cursor on `(` or `)`, select the inner contents of that pair.
4. For `a(` with cursor inside, select including the delimiters.
5. Brackets inside string literals and comments SHOULD be ignored if syntax information is available; otherwise, naive matching is acceptable.

## Quote matching algorithm (normative)

For quote text objects:

1. Search only on the current line.
2. If the cursor is inside a quoted string, select that string.
3. If the cursor is outside any quoted string, select the next quoted string to the right on the same line (if any).
4. Escaped quotes (backslash-preceded) MUST NOT be treated as delimiters.

## Count behavior

Counts on text objects select successively larger enclosing structures. For example, `2i(` selects the contents of the second-level enclosing parentheses.

## Related

- Motions: [/docs/spec/editing/motions/motions.md](/docs/spec/editing/motions/motions.md)
- Operators: [/docs/spec/editing/operators/operators.md](/docs/spec/editing/operators/operators.md)
- Syntax engine: [/docs/spec/features/syntax/syntax.md](/docs/spec/features/syntax/syntax.md)
