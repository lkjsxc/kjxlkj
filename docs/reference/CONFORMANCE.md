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

A docs-only baseline is active. Source and build artifacts are intentionally
absent so implementation can be regenerated from canonical documentation.
Runtime conformance is therefore unverified by design until reconstruction.

## Evidence Summary

| Check | Status | Evidence Date | Evidence |
|---|---|---|---|
| Docs authority and precedence are defined | `verified` | 2026-02-11 | [/docs/README.md](/docs/README.md), [/docs/policy/README.md](/docs/policy/README.md) |
| TODO reconstruction chain is present | `verified` | 2026-02-11 | [/docs/todo/README.md](/docs/todo/README.md), [/docs/todo/checklists/README.md](/docs/todo/checklists/README.md) |
| Implementation workspace is present | `unverified` | 2026-02-11 | intentionally absent in docs-only preparation state |
| Runtime blocker behavior (`Shift+a`, split, explorer) | `unverified` | 2026-02-11 | no executable artifact in current baseline |
| Live E2E screen-oracle closure | `unverified` | 2026-02-11 | pending reconstruction and execution |

## Domain Status

| Domain | Status | Reason |
|---|---|---|
| Input decoding and key normalization | `unverified` | implementation absent in docs-only baseline |
| Window tree and split lifecycle | `unverified` | implementation absent in docs-only baseline |
| Explorer window and actions | `unverified` | implementation absent in docs-only baseline |
| Terminal window integration | `unverified` | implementation absent in docs-only baseline |
| Viewport wrap and cursor safety | `unverified` | implementation absent in docs-only baseline |
| Test harness fidelity | `unverified` | harness absent until reconstruction |
| Source topology and workspace policy | `blocked` | grouped workspace tree not present in docs-only state |

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
