# Scripting: Timing and Debounce (Iteration 34)

Back: [/docs/todo/current/wave-implementation/scripting/README.md](/docs/todo/current/wave-implementation/scripting/README.md)

## Scope

Implement timing/debounce rules so automation is deterministic and does not introduce lag.

## Defining documents (direct, normative)

- Timing and debounce:
  - [/docs/spec/scripting/timing-debounce.md](/docs/spec/scripting/timing-debounce.md)

## Checklist

- [x] Placeholder scaffolding: define scheduler/timer abstraction boundaries. — done: `TimerHandle`, `DebouncedAction`, `Scheduler` in `scripting.rs`
- [x] Minimal slice: implement one deterministic debounce path with tests. — done: `debounce_exec.rs` with `DebounceManager`, `FakeClock`, schedule/cancel/tick, 7 tests
- [x] Full conformance: implement all timing rules, including cancel/merge semantics. — done: coalescing, cancel, multi-action, tick-driven firing all tested

