# Modes: Normal (Iteration 34)

Back: [/docs/todo/current/wave-implementation/modes/README.md](/docs/todo/current/wave-implementation/modes/README.md)

## Scope

Implement Normal mode navigation and command initiation behaviors.

## Defining documents (direct, normative)

- Normal mode spec:
  - [/docs/spec/modes/normal.md](/docs/spec/modes/normal.md)
- Keybindings reference (for expected coverage):
  - [/docs/spec/ux/keybindings/navigation.md](/docs/spec/ux/keybindings/navigation.md)
  - [/docs/spec/ux/keybindings/editing.md](/docs/spec/ux/keybindings/editing.md)

## Checklist

### A. Placeholder scaffolding

- [ ] Define Normal-mode key interpretation layers (raw input → mapping → intent).

### B. Minimal conformance slice

- [ ] Implement a minimal, test-backed subset of Normal-mode keys that is fully specified.
  - Cursor movement (h, j, k, l, w, b, e, 0, ^, $, gg, G)
  - Mode transitions (i, a, A, o, v, V, R, :)
  - Operators (d, y, c) with motions and text objects
  - Line operators (dd, yy, cc, >>, <<)
  - Find char (f, t, F, T, ;, ,)
  - Search (/, ?, n, N)
  - Undo/redo (u, Ctrl-r)
  - Dot repeat (.)

### C. Full conformance

- [ ] Implement all Normal-mode behaviors and keybindings required by the spec (or record limitations).
  - Marks (m, ', `)
  - Registers (", etc.)
  - Macros (q, @, @@)
  - Jump list (Ctrl-o, Ctrl-i)
  - Change list (g;, g,)
  - Sentence/paragraph motions ((, ), {, })
  - Match bracket (%)
  - Join lines (J, gJ)
  - Case operators (~, g~, gU, gu)
  - Increment/decrement (Ctrl-a, Ctrl-x)
  - Block visual mode (Ctrl-v)

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
