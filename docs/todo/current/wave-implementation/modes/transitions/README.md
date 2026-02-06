# Modes: Transitions (Iteration 34)

Back: [/docs/todo/current/wave-implementation/modes/README.md](/docs/todo/current/wave-implementation/modes/README.md)

## Scope

Implement mode transition rules and invariant enforcement across transitions.

## Defining documents (direct, normative)

- Mode transitions:
  - [/docs/spec/modes/transitions.md](/docs/spec/modes/transitions.md)

## Checklist

### A. Placeholder scaffolding

- [x] Define a single transition table/graph with explicit guards.

### B. Minimal conformance slice

- [x] Implement a minimal transition set and tests covering cancellation and clamping rules.

### C. Full conformance

- [x] Implement all transitions, including terminal-mode transitions when applicable. — done: `transitions.rs` with `is_valid_transition()`, `valid_targets()`, `escape_target()` and 7 tests

### D. Conformance updates

- [x] Update: — done: conformance and limitations entries maintained with each batch
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

