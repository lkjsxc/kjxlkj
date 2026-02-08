# Motions
Motions move the cursor and/or define a range when combined with an operator.

## Requirements
- Motions are deterministic, core-owned operations.
- Every motion targets a specific `BufferVersion`.
- When combined with operators, range computation must be stable across platforms and encodings.

## Motion families (normative)

### Character motions

| Key | Motion | Inclusive/Exclusive |
|---|---|---|
| `h` | Left one grapheme cluster | exclusive |
| `l` | Right one grapheme cluster | exclusive |
| `j` | Down one line (preserving desired column) | linewise |
| `k` | Up one line (preserving desired column) | linewise |

### Word/WORD motions

| Key | Motion | Inclusive/Exclusive |
|---|---|---|
| `w` | Forward to start of next word | exclusive |
| `W` | Forward to start of next WORD | exclusive |
| `e` | Forward to end of word | inclusive |
| `E` | Forward to end of WORD | inclusive |
| `b` | Backward to start of word | exclusive |
| `B` | Backward to start of WORD | exclusive |
| `ge` | Backward to end of previous word | inclusive |
| `gE` | Backward to end of previous WORD | inclusive |

**Word** delimiters: alphanumeric sequences, underscore sequences, and punctuation sequences form separate words. **WORD** delimiters: only whitespace separates WORDs.

### Line motions

| Key | Motion | Description |
|---|---|---|
| `0` | Go to first column | Exclusive |
| `^` | Go to first non-blank | Exclusive |
| `$` | Go to end of line | Inclusive |
| `g_` | Go to last non-blank | Inclusive |
| `g0` | Go to first screen column (when wrapped) | Exclusive |
| `g$` | Go to last screen column (when wrapped) | Inclusive |
| `gm` | Go to middle of screen line | Exclusive |
| `+` / `Enter` | Down to first non-blank | Linewise |
| `-` | Up to first non-blank | Linewise |

### Document motions

| Key | Motion |
|---|---|
| `gg` | Go to first line (or line N with count) |
| `G` | Go to last line (or line N with count) |
| `{count}G` | Go to line `{count}` |

### Scroll motions

| Key | Motion |
|---|---|
| `Ctrl-u` | Scroll up half page |
| `Ctrl-d` | Scroll down half page |
| `Ctrl-b` | Scroll up full page |
| `Ctrl-f` | Scroll down full page |
| `Ctrl-y` | Scroll up one line (cursor stays) |
| `Ctrl-e` | Scroll down one line (cursor stays) |
| `zz` | Center cursor line on screen |
| `zt` | Scroll cursor line to top |
| `zb` | Scroll cursor line to bottom |
| `H` | Move cursor to top of screen |
| `M` | Move cursor to middle of screen |
| `L` | Move cursor to bottom of screen |

### Find/till motions

| Key | Motion | Inclusive/Exclusive |
|---|---|---|
| `f{c}` | Forward to character `{c}` on current line | inclusive |
| `F{c}` | Backward to character `{c}` on current line | exclusive |
| `t{c}` | Forward till before character `{c}` | inclusive |
| `T{c}` | Backward till after character `{c}` | exclusive |
| `;` | Repeat last f/F/t/T in same direction | same as original |
| `,` | Repeat last f/F/t/T in opposite direction | same as original |

### Structural motions

| Key | Motion |
|---|---|
| `(` | Previous sentence |
| `)` | Next sentence |
| `{` | Previous paragraph (empty line boundary) |
| `}` | Next paragraph (empty line boundary) |
| `%` | Matching bracket/brace/parenthesis |

## Counts

All motions accept a count prefix (e.g., `5j`, `3w`, `10G`). The motion is repeated `count` times, or the motion interprets the count directly (as `G` does with a line number).

## Desired column (normative)

When moving vertically with `j` or `k`, the cursor remembers a "desired column." If the target line is shorter than the desired column, the cursor lands on the last grapheme. The desired column is reset by any horizontal motion (e.g., `h`, `l`, `w`, `0`, `$`).

For CJK (width-2) graphemes, the desired column is measured in display columns, not grapheme indices.

## Inclusive vs exclusive (normative)

When combined with an operator:

| Type | Range includes |
|---|---|
| Inclusive | The character under the cursor at both start and end positions |
| Exclusive | The character at the end position is NOT included |
| Linewise | Both start and end lines are fully included |

## Cursor semantics

Canonical cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)

## Related

- Repeat motions: [repeat-motions.md](repeat-motions.md)
- Cursor: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Operators: [/docs/spec/editing/operators/operators.md](/docs/spec/editing/operators/operators.md)
- Text objects: [/docs/spec/editing/text-objects/text_objects.md](/docs/spec/editing/text-objects/text_objects.md)
