# Modes: Normal (Iteration 33)

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

### C. Full conformance

- [ ] Implement all Normal-mode behaviors and keybindings required by the spec (or record limitations).

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
## Scope

Implement configuration options that affect modes and mode behavior.

## Defining documents (direct, normative)

- Mode configuration:
  - [/docs/spec/modes/configuration.md](/docs/spec/modes/configuration.md)

## Checklist

### A. Placeholder scaffolding

- [ ] Enumerate all configuration items that affect mode behavior.

### B. Minimal conformance slice

- [ ] Implement a minimal configuration set with deterministic tests.

### C. Full conformance

- [ ] Implement all configuration options in the doc and ensure they match runtime behavior.

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
