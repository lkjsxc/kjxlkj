# Exclusive vs Inclusive Motions

How motion boundary type affects operator behavior.

## Definitions

- **Inclusive**: The character at the destination IS included in the operation
- **Exclusive**: The character at the destination is NOT included in the operation

## Inclusive Motions (complete list)

| Motion | Description |
|---|---|
| `e`, `ge` | Word end |
| `E`, `gE` | WORD end |
| `$` | End of line |
| `g_` | Last non-blank of line |
| `f{c}` | Find character forward |
| `F{c}` | Find character backward |
| `` `{mark} `` | Go to mark position |
| `%` | Matching bracket |

## Exclusive Motions (complete list)

| Motion | Description |
|---|---|
| `w`, `b` | Word start |
| `W`, `B` | WORD start |
| `t{c}` | Till character forward |
| `T{c}` | Till character backward |
| `(`, `)` | Sentence |
| `{`, `}` | Paragraph |
| `/pattern`, `?pattern` | Search |
| `n`, `N` | Repeat search |
| `'{mark}` | Go to mark line |
| `0`, `^` | Line start |
| `h`, `l` | Character left/right |
| `G`, `gg` | Go to line (linewise, but exclusive when forced charwise) |

## Till vs Find Example

Given text `hello world` with cursor on `h`:

- `dfw` — deletes `hello w` (inclusive: includes `w`)
- `dtw` — deletes `hello ` (exclusive: stops before `w`)

## Exclusive-to-Inclusive Adjustment

When an exclusive characterwise motion ends at column 0 of a line past the start line, the end is adjusted back to the last character of the previous line (making it inclusive). This prevents accidentally including an empty line.

## `v` Toggle

Placing `v` between operator and motion toggles exclusive/inclusive for characterwise motions:

- `dve` — `e` is normally inclusive → becomes exclusive
- `dvw` — `w` is normally exclusive → becomes inclusive

## Visual Mode Selection

In visual mode, inclusive motions extend the selection to include the destination character. Exclusive motions stop just before it. `o` and `O` swap selection ends.

## Related

- Operator modifiers: [/docs/spec/editing/operators/operator-modifiers.md](/docs/spec/editing/operators/operator-modifiers.md)
- Motion grammar: [/docs/spec/editing/motions/motion-grammar.md](/docs/spec/editing/motions/motion-grammar.md)
