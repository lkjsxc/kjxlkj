# Proposal: Repeatable Profiling Workflow

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem

When a performance regression is detected, developers need a repeatable workflow to profile, identify the bottleneck, and verify the fix.

## Defining specs

- Latency targets: [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- Debugging guidance: [/docs/technical/debugging.md](/docs/technical/debugging.md)

## Conformance claim

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) — Technical section

## Workflow

1. Reproduce the regression with a deterministic test case.
2. Profile using `cargo flamegraph` or equivalent.
3. Identify the hot path.
4. Fix and verify the fix restores target latency.
5. Add a regression test to prevent recurrence.

## Implementation TODO

- [/docs/todo/current/wave-implementation/technical/latency/regression/README.md](/docs/todo/current/wave-implementation/technical/latency/regression/README.md)

## Status

Profiling infrastructure exists in the test suite. Regression tests exercise typing bursts and render coalescing.
