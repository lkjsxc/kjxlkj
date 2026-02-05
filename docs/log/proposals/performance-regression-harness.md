# Proposal: Performance Regression Harness

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem

Need a harness to detect CPU/latency regressions and gate changes that degrade performance.

## Defining specs

- [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- [/docs/reference/CI.md](/docs/reference/CI.md)

## Status

Placeholder - to be refined in verification wave.

## Acceptance criteria

- Given a performance test suite, when running, then baseline latencies MUST be recorded.
- Given a change, when running perf tests, then regressions MUST be flagged.

## Test strategy

- Latency regression tests
- CPU profiling integration
