# Character Find Motions

f, t, F, T character search.

## Overview

Find specific characters on the current line only.
These are exclusive/inclusive motions depending on
the command used.

## Forward Find

### f (Find)

`f{char}` moves cursor forward to the next occurrence
of `{char}` on the current line.

**Inclusive**: Cursor lands ON the character.
When used with `d`, the character is deleted.

### t (Till)

`t{char}` moves cursor forward to just BEFORE the next
occurrence of `{char}` on the current line.

**Inclusive**: Cursor lands one position BEFORE the character.
Useful for deleting up to but not including a character.

## Backward Find

### F (Find back)

`F{char}` moves cursor backward to the previous
occurrence of `{char}` on the current line.

**Exclusive**: Cursor lands ON the character.

### T (Till back)

`T{char}` moves cursor backward to just AFTER the
previous occurrence of `{char}` on the current line.

**Exclusive**: Cursor lands one position AFTER the character.

## Examples

### Forward

On `hello world`, cursor on `h`:
- `fw` moves to `w` in `world`
- `tw` moves to the space before `w`
- `fl` moves to first `l` in `hello`
- `2fl` moves to second `l` in `hello`

### Backward

On `hello world`, cursor on `d`:
- `Fh` moves to `h`
- `Th` moves to `e` (one after `h`)
- `Fo` moves to `o` in `world`
- `2Fo` moves to `o` in `hello`

## With Operators

### Delete

| Sequence | Effect |
|----------|--------|
| `df)` | Delete from cursor through `)` |
| `dt)` | Delete from cursor to before `)` |
| `dF(` | Delete backward through `(` |
| `dT(` | Delete backward to after `(` |

### Yank

| Sequence | Effect |
|----------|--------|
| `yf;` | Yank from cursor through `;` |
| `yt;` | Yank from cursor to before `;` |

### Change

| Sequence | Effect |
|----------|--------|
| `cf"` | Change from cursor through `"` |
| `ct"` | Change from cursor to before `"` |

## Repeat Find

### Same Direction

`;` repeats the last f/t/F/T motion in the same direction.

### Opposite Direction

`,` repeats the last f/t/F/T motion in the opposite direction.

### Example

On `a,b,c,d`: `f,` finds first `,`. Then `;` finds
the second `,`, and `;` again finds the third.
`,` goes back to the second `,`.

## Count

### Multiple Characters

`{count}f{char}` finds the {count}th occurrence.
`3fa` finds the third `a` on the current line.
If fewer than {count} occurrences exist, cursor stays.

## Visual Mode

### Select With f/t

`vf)` selects from cursor through `)`.
`vt)` selects from cursor to before `)`.

### Extend Selection

In visual mode, repeated `;` extends the selection
forward through each next match.

## Line Scope

### Line Only

f/t/F/T only search the current line.
They will NOT cross line boundaries.

### No Match

If the character is not found on the current line:
- Cursor stays in place (no movement)
- No error beep unless `errorbells` is enabled
- Operator is cancelled

## Special Characters

### Finding Punctuation

`f.` finds next period, `f,` finds next comma.
`f<Space>` finds next space character.

### Finding Spaces

`f ` (f followed by a literal space) finds the next
space on the line.

## CJK Characters

### Wide Characters

`f{CJK}` finds CJK characters. Since CJK characters
have display width 2, the cursor positions on the
first column of the character. The `;` repeat works
correctly with wide characters.

### Input

The find character is read as a complete Unicode
grapheme cluster, so CJK input works correctly.

## Common Patterns

### Delete to Character

`dt)` — delete everything before closing paren.
`df,` — delete through next comma (including it).

### Change Inside

`ct"` — change text up to the next quote.
`cT"` — change text back to after the previous quote.

### Quick Navigation

`f{` then `;` to hop through brace-delimited blocks.
`f,` then `;` to hop through comma-separated items.
