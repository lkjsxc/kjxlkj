# Technical: Profiling (Iteration 34)

Back: [/docs/todo/current/wave-implementation/technical/README.md](/docs/todo/current/wave-implementation/technical/README.md)

## Scope

Implement profiling hooks and workflows so performance regressions are detectable.

## Defining documents (direct, normative)

- Profiling:
  - [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md)

## Checklist

- [x] Define how profiling is enabled/disabled without changing behavior. — done: `profiling.rs` with `Profiler::enable()/disable()`, no-op when disabled
- [x] Add profiling instrumentation for core loop and rendering. — done: `ProfilingSpan`, `Counter`, `begin_span()/end_span()`, `count()`, `report()` aggregation, 6 tests
- [x] Establish a repeatable profiling workflow recorded in docs/log. — done: profiling_workflow.rs (services) with ProfileTarget, ProfileConfig, compute_stats, format_report, meets_budget

