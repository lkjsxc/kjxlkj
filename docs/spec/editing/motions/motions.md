# Motions
Motions move the cursor and/or define a range when combined with an operator.

## Requirements
- Motions are deterministic, core-owned operations.
- Every motion targets a specific `BufferVersion`.
- When combined with operators, range computation must be stable across platforms and encodings.

## Motion families (normative)

- Character: `h` `j` `k` `l`
- Word/WORD: `w` `W` `e` `E` `b` `B` `ge` `gE`
- Line: `0` `^` `$` `g_` `+` `-` `gm`
- Document: `gg` `G` `<n>G`
- Screen/scroll: `Ctrl-u` `Ctrl-d` `Ctrl-b` `Ctrl-f` `zz` `zt` `zb`
- Find/till: `f{c}` `F{c}` `t{c}` `T{c}` with repeat `;` and reverse `,`
- Structural: `(` `)` `{` `}` `%`

## Counts

All motions accept a count prefix (e.g. `5j`, `3w`, `10G`).

## Cursor semantics

Canonical spec cursor semantics: [docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)

## Related

- Repeat motions: [repeat-motions.md](repeat-motions.md)
- Cursor: [docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)

