# Modes: Visual (Iteration 34)

Back: [/docs/todo/current/wave-implementation/modes/README.md](/docs/todo/current/wave-implementation/modes/README.md)

## Scope

Implement Visual mode state and key interpretation (selection mechanics are also covered under editing/visual).

## Defining documents (direct, normative)

- Visual mode spec:
  - [/docs/spec/modes/visual.md](/docs/spec/modes/visual.md)
- Visual keybindings:
  - [/docs/spec/ux/keybindings/visual.md](/docs/spec/ux/keybindings/visual.md)

## Checklist

### A. Placeholder scaffolding

- [x] Define Visual mode entry/exit and selection anchoring rules.

### B. Minimal conformance slice

- [x] Implement charwise Visual mode entry/exit with deterministic tests.

### C. Full conformance

- [ ] Implement all Visual mode behaviors and keybindings (including variants) or record limitations.
  - Charwise visual mode (v) - implemented
  - Linewise visual mode (V) - implemented
  - Blockwise visual mode (Ctrl-v) - implemented
  - Visual operators (d, y, c) - implemented

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

