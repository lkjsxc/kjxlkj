# Proposal: CPU/Latency Regression Harness

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem

Performance regressions (typing latency, scroll jank, resize storms) must be detectable before they reach users.

## Defining specs

- Latency targets: [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- Memory constraints: [/docs/spec/technical/memory.md](/docs/spec/technical/memory.md)

## Conformance claim

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) — Technical section

## Limitations

- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) — performance entry

## Implementation TODO

- [/docs/todo/current/wave-implementation/technical/latency/regression/README.md](/docs/todo/current/wave-implementation/technical/latency/regression/README.md)

## Test requirements

- Given a typing burst of 100 characters, when rendered, then total latency is below the target.
- Given a resize storm of 50 events, when coalesced, then only the final size is rendered.
- Given a scroll burst, when rendered, then frame timing remains within budget.

## Gating strategy

Performance regression tests run as part of `cargo test` using assertion-based timing checks with conservative thresholds.

## Status

Implemented and tested in kjxlkj-render and kjxlkj-host crates.
