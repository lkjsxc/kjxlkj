# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger reports the strongest verified state as of the snapshot date.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | deterministic evidence exists and no high-severity contradiction is known |
| `partial` | behavior exists but reachability or evidence quality is incomplete |
| `blocked` | high-severity mismatch is known and not yet closed |
| `unverified` | no trustworthy evidence currently exists |

## Current Snapshot (2026-02-11)

Workspace reconstructed with 20 crates. Runtime conformance is partially verified
through 60 deterministic unit and integration tests covering key normalization,
mode dispatch, cursor motion, text buffer operations, layout tree, and editor state.
Multi-task runtime architecture implemented (input/core/render tasks with bounded
channels, signal handlers, proper shutdown).
All source files comply with ≤ 200 line limit.
PTY-level E2E verification pending harness reconstruction.

## Evidence Summary

| Check | Status | Evidence Date | Evidence |
|---|---|---|---|
| Docs authority and precedence are defined | `verified` | 2026-02-11 | [/docs/README.md](/docs/README.md), [/docs/policy/README.md](/docs/policy/README.md) |
| TODO reconstruction chain is present | `verified` | 2026-02-11 | [/docs/todo/README.md](/docs/todo/README.md), [/docs/todo/waves/README.md](/docs/todo/waves/README.md) |
| Implementation workspace is present | `verified` | 2026-02-11 | 20-crate workspace, `cargo check --workspace` and `cargo test --workspace` (60 pass) |
| Runtime blocker behavior (`Shift+a`, split, explorer) | `partial` | 2026-02-11 | T1 headless harness tests pass; T2 PTY harness pending |
| Live E2E screen-oracle closure | `unverified` | 2026-02-11 | PTY harness not yet reconstructed |

## Domain Status

| Domain | Status | Reason |
|---|---|---|
| Input decoding and key normalization | `partial` | Shift+a normalization implemented and unit-tested; T2 pending |
| Window tree and split lifecycle | `partial` | layout tree with split/close/rebalance implemented and unit-tested; T2 pending |
| Explorer window and actions | `unverified` | stub crate only; explorer state model not yet implemented |
| Terminal window integration | `unverified` | stub crate only; PTY not yet implemented |
| Viewport wrap and cursor safety | `unverified` | basic cursor motion; wrap not yet implemented |
| Test harness fidelity | `partial` | T1 headless harness with step dumps; T2 PTY harness pending |
| Source topology and workspace policy | `verified` | 20-crate grouped tree matches spec; all files ≤ 200 lines; multi-task runtime |

## Release Rule

Release conformance is not met while any high-severity limitation is open.

A release may proceed only when all are true:

1. all high-severity rows in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) are closed
2. matching `*R` E2E rows in [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) pass using screen-state assertions
3. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md),
   [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md), and
   [/docs/todo/README.md](/docs/todo/README.md) are synchronized in the same change

Current state (2026-02-11): blocked (docs-only baseline).

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift rows: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction checklist: [/docs/todo/README.md](/docs/todo/README.md)
