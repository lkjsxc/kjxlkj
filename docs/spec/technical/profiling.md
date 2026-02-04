# Profiling and Performance Observability

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)
Normative observability requirements for diagnosing latency and performance regressions.

## Goals (normative)

The implementation MUST make it possible to answer:

- Where is time spent (input decode, core update, snapshot, render, I/O, services)?
- Is per-frame work bounded to the viewport (or is there hidden O(file) behavior)?
- Is the host busy-looping while idle (unnecessary CPU usage)?

## Instrumentation requirements (normative)

Instrumentation MUST be:

- opt-in (disabled by default)
- low overhead when enabled (sufficient for dev profiling)
- zero/near-zero overhead when disabled
- deterministic with respect to core behavior (instrumentation must not change edit semantics)

## Required metrics (normative)

The implementation MUST be able to record, at minimum, the following per input/update cycle:

| Metric | Meaning | Why it matters |
|---|---|---|
| Input events processed | Count of decoded inputs applied | detects drops and batching behavior |
| Core update count | Number of state transitions applied | correlates with intents |
| Snapshot duration | Time spent producing a snapshot | detects O(file) snapshot work |
| Render duration | Time spent producing terminal output | detects full redraw storms |
| Snapshot “materialized lines” | Count of buffer lines materialized into the snapshot | strong signal for viewport-boundedness |

If the renderer is cell-based, it SHOULD also record:

| Metric | Meaning |
|---|---|
| Cells written | Number of terminal cells written per frame |
| Dirty region size | Area of the screen that changed |

## Required probes (normative)

The implementation MUST support probes that can be exercised by tests/benchmarks:

| Probe | Requirement |
|---|---|
| Idle CPU probe | Must detect redraw/snapshot busy-loop while idle (no input, no animation). |
| Long-line probe | Must detect per-frame work proportional to total line length when only a slice is visible. |
| Large-file probe | Must detect snapshot/render work proportional to total buffer size. |

## Acceptance criteria (Given/When/Then)

1. Given instrumentation is enabled, when the editor processes a burst of input, then the implementation MUST produce a record that includes snapshot duration and render duration for the burst.
2. Given a very large buffer, when producing a snapshot, then the recorded “materialized lines” MUST be bounded by the viewport height (plus a small constant margin).
3. Given no input for an extended period, when instrumentation is enabled, then the idle CPU probe MUST report no continuous redraw loop.

## Related

- Latency ordering: [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- Large-file posture: [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
- Memory posture: [/docs/spec/technical/memory.md](/docs/spec/technical/memory.md)
- Test strategy: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

