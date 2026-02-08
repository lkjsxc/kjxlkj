# Character Find Motions

Back: [/docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)

Find a character on the current line and move the cursor to it.

## Commands

| Key | Type | Description |
|---|---|---|
| `f{char}` | Inclusive | Move forward to the next occurrence of `{char}` on the line |
| `F{char}` | Inclusive | Move backward to the previous occurrence of `{char}` on the line |
| `t{char}` | Exclusive | Move forward to just before the next `{char}` |
| `T{char}` | Exclusive | Move backward to just after the previous `{char}` |
| `;` | - | Repeat the last `f`/`F`/`t`/`T` in the same direction |
| `,` | - | Repeat the last `f`/`F`/`t`/`T` in the opposite direction |

## Behavior

Detailed semantics of character-find motions.

### Scope

Character find motions operate within the current line only. They do not cross line boundaries.

### Count

A count prefix finds the Nth occurrence. `3fa` finds the 3rd `a` on the line.

### Failure

If the character is not found on the line, the cursor does not move and a bell is emitted.

### Inclusivity with operators

| Motion | With `d` | Example |
|---|---|---|
| `df{char}` | Deletes from cursor through `{char}` (inclusive) | `dfa` on `abcabc` at `a` deletes `abca` |
| `dt{char}` | Deletes from cursor to just before `{char}` (exclusive) | `dta` on `abcabc` at col 0 would delete up to but not including the second `a` |

### CJK characters

`f` and `t` match by grapheme cluster. `f{cjk}` finds the next CJK character. The cursor lands on the grapheme boundary.

## Repeat

The last character find command and its target character are remembered. `;` repeats in the original direction and `,` repeats in the reverse direction.

After `fa` on `abracadabra`, pressing `;` finds the next `a`, and `,` finds the previous `a`.

## Operator-pending

In operator-pending mode, `f`/`F`/`t`/`T` define the range for the operator.

| Command | Effect |
|---|---|
| `df)` | Delete from cursor to and including `)` |
| `ct"` | Change from cursor to just before `"` |
| `yF(` | Yank from cursor backward to and including `(` |

## Related

- Motions overview: [/docs/spec/editing/motions/motions.md](/docs/spec/editing/motions/motions.md)
- Repeat motions: [/docs/spec/editing/motions/repeat-motions.md](/docs/spec/editing/motions/repeat-motions.md)
- Search motions: [/docs/spec/editing/motions/search-motions.md](/docs/spec/editing/motions/search-motions.md)
