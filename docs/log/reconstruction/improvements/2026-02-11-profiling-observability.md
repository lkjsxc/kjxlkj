# 2026-02-11 Profiling Observability Closure

Back: [README.md](README.md)

## Objective

Close deferred profiling limitation `LIM-DEF-PERF-01` by wiring opt-in runtime observability with deterministic PTY verification.

## Implemented

- Added opt-in profiling module in `src/crates/app/kjxlkj/src/profiling.rs`.
- Wired per-cycle metric collection in `src/crates/app/kjxlkj/src/main.rs`.
- Added live PTY suite `src/crates/app/kjxlkj-test-harness/tests/profiling_e2e.rs` with `PERF-01R`..`PERF-03R`.
- Synchronized `LIMITATIONS`, `CONFORMANCE`, `DRIFT_MATRIX`, and TODO matrices.

## Source Line-Limit Note

A transient generated version of `src/crates/app/kjxlkj/src/profiling.rs` reached 279 lines during implementation.
The final committed module was reduced to 156 lines to satisfy the source layout policy (`<=200`).

## Follow-up Ideas

- Add percentile buckets for snapshot and render durations to improve trend visibility.
- Add optional per-window profiling counters for multi-pane churn scenarios.
- Add CI artifact capture for `PROFILE` lines on failed `PERF-*R` runs.
