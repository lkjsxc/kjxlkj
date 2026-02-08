# Operator Modifiers

Force motion type when applying operators.

## Motion Types

Every motion has a default type: characterwise, linewise, or blockwise. Modifiers override this default.

## Force Modifiers (normative)

| Modifier | Between operator and motion | Effect |
|---|---|---|
| `v` | `d v j` | Force characterwise |
| `V` | `d V j` | Force linewise |
| `<C-v>` | `d <C-v> j` | Force blockwise |

The modifier key is pressed between the operator and the motion.

## Force Characterwise (`v`)

Makes a linewise motion act characterwise. The operation affects from cursor position to the exact destination position.

Example: `j` is normally linewise. `dvj` deletes from cursor position to the same column on the next line (characterwise), rather than deleting both entire lines.

## Force Linewise (`V`)

Makes a characterwise motion act linewise. The operation affects entire lines covered by the motion.

Example: `w` is normally characterwise. `dVw` deletes the entire line(s) covered by the word motion.

## Force Blockwise (`<C-v>`)

Creates a rectangular selection for the operator. The block spans from cursor to destination position.

Example: `d<C-v>2j` deletes a column block spanning the cursor column across 3 lines.

## Force Impact Table

| Motion | Default Type | + `v` | + `V` | + `<C-v>` |
|---|---|---|---|---|
| `j`, `k` | Linewise | Characterwise | Linewise | Blockwise |
| `w`, `e`, `b` | Characterwise | Characterwise | Linewise | Blockwise |
| `$` | Characterwise | Characterwise | Linewise | Blockwise |
| `G` | Linewise | Characterwise | Linewise | Blockwise |
| `/pattern` | Characterwise | Characterwise | Linewise | Blockwise |
| `f{c}` | Characterwise | Characterwise | Linewise | Blockwise |

## Yank Type Affects Paste

The type used during yank determines paste behavior. A block yank (`y<C-v>j`) pastes as a block.

## Related

- Exclusive/inclusive: [/docs/spec/editing/operators/exclusive-inclusive.md](/docs/spec/editing/operators/exclusive-inclusive.md)
- Forced motion types: [/docs/spec/editing/operators/forced-motion-types.md](/docs/spec/editing/operators/forced-motion-types.md)
