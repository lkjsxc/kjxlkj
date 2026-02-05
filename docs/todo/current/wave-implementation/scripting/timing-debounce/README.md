# Scripting: Timing and Debounce (Iteration 34)

Back: [/docs/todo/current/wave-implementation/scripting/README.md](/docs/todo/current/wave-implementation/scripting/README.md)

## Scope

Implement timing/debounce rules so automation is deterministic and does not introduce lag.

## Defining documents (direct, normative)

- Timing and debounce:
  - [/docs/spec/scripting/timing-debounce.md](/docs/spec/scripting/timing-debounce.md)

## Checklist

- [x] Placeholder scaffolding: define scheduler/timer abstraction boundaries.
- [x] Minimal slice: implement one deterministic debounce path with tests.
- [x] Full conformance: implement all timing rules, including cancel/merge semantics.

