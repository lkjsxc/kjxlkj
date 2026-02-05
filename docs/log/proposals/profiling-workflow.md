# Proposal: Profiling Workflow and Regression Triage

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem statement

The performance posture in the canonical spec requires the editor to remain responsive under stress and to avoid idle busy-loop rendering.

Without a repeatable profiling workflow, regressions tend to reappear (especially around rendering I/O, snapshot building, long-line handling, and resize storms).

This proposal describes a workflow that is:

- deterministic enough to run in CI (when CI exists)
- actionable for local triage (when CI is absent)
- aligned with the normative observability requirements

## Defining documents

- Normative observability requirements:
  - [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md)
- Ordering/latency invariants:
  - [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- Large-file and long-line posture:
  - [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
  - [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Regression harness proposal (trend + probes):
  - [/docs/log/proposals/performance-regression-harness.md](/docs/log/proposals/performance-regression-harness.md)

## Proposed workflow (repeatable)

### A. Enable instrumentation (opt-in)

The implementation SHOULD provide:

| Mechanism | Requirement |
|---|---|
| Build-time toggle | Instrumentation MUST be opt-in and MUST have near-zero overhead when disabled. |
| Runtime toggle | Profiling MUST be enable-able at runtime to avoid requiring separate binaries for everyday debugging. |
| Output sink | Profiling output MUST be machine-parseable and MUST support at least one local sink (stderr or a file path). |

The specific toggles (feature flag names, env vars, CLI flags) are implementation-defined, but MUST be recorded in the verification gate docs when an implementation exists (see [/docs/reference/CI.md](/docs/reference/CI.md)).

### B. Record per-cycle metrics (spec-driven)

For each input/update cycle, the implementation MUST be able to emit the minimum metrics set defined in:

- [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md)

The workflow treats three numbers as primary regression detectors:

| Detector | Why it matters |
|---|---|
| Snapshot duration | Catches hidden O(file) work in snapshot generation. |
| “Materialized lines” | Strong signal for viewport-boundedness violations. |
| Cells written / dirty region | Catches full redraw storms and excessive terminal I/O. |

### C. Run probes (turn “it feels slow” into “what broke”)

The workflow is to run probes that correspond directly to normative requirements:

| Probe | What it detects | Canonical requirement |
|---|---|---|
| Idle CPU probe | Continuous redraw/snapshot loop while idle | [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md) |
| Long-line probe | Per-frame work proportional to total line length | [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md) |
| Large-file probe | Snapshot/render work proportional to total buffer size | [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md) |

### D. Classify regressions by “where the time went”

When a regression is observed, classify it into one of these buckets:

| Bucket | Typical root causes |
|---|---|
| Input decode | terminal event normalization, key-sequence buffering, repeated parsing |
| Core update | accidental O(N) scans, expensive regex/search, undo churn |
| Snapshot | materializing too much text, reflowing long lines, rebuilding decorations |
| Render | full redraw, diff algorithm regressions, too many flushes/syscalls |
| I/O | sync FS access on hot path, small writes, excessive metadata/stat calls |
| Services | unbounded queues, cancellation not honored, stale results applied |

The classification MUST drive the next step: add or tighten a deterministic test that fails when the regression returns (see [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)).

## CI integration (trend, not brittle budgets)

When CI is present, profiling SHOULD be integrated as:

- deterministic probes that hard-fail on algorithmic regressions (viewport-boundedness, idle busy-loop)
- trend reporting for timing metrics (avoid flaky absolute thresholds unless stabilized)

## Acceptance criteria (Given/When/Then)

| Scenario | Requirement |
|---|---|
| Idle for an extended period | Idle probe MUST report no continuous redraw/snapshot loop. |
| Render a very long line | “Materialized lines” MUST remain viewport-bounded; render MUST not do work proportional to total line length. |
| Open and scroll a large buffer | Snapshot and render metrics MUST not scale with total buffer size. |

## Related

- Performance regression harness proposal: [/docs/log/proposals/performance-regression-harness.md](/docs/log/proposals/performance-regression-harness.md)
- Profiling requirements (normative): [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md)
