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

The workspace and core verification harness exist, and targeted tests pass.
However, user runtime reports on 2026-02-11 indicate split view, explorer,
and `Shift+a` are not working in practical editor sessions. Release conformance
is therefore blocked until stronger live E2E evidence closes those reports.

## Evidence Summary

| Check | Status | Evidence Date | Evidence |
|---|---|---|---|
| Docs authority and precedence are defined | `verified` | 2026-02-11 | [/docs/README.md](/docs/README.md), [/docs/policy/README.md](/docs/policy/README.md) |
| Workspace and crate topology are present | `verified` | 2026-02-11 | root manifests + grouped `src/crates/` tree |
| Targeted high-risk PTY suites currently pass | `partial` | 2026-02-11 | `cargo test -p kjxlkj-test-harness --test key_mode_e2e --test window_nav_e2e --test explorer_terminal_paths_e2e` |
| Split/explorer/`Shift+a` runtime behavior is user-reported broken | `blocked` | 2026-02-11 | direct user runtime report; stronger than trace-only confidence |
| Live E2E fidelity is screen-assertion complete | `blocked` | 2026-02-11 | existing tests are trace-centric; full screen-state oracle is incomplete |

## Domain Status

| Domain | Status | Reason |
|---|---|---|
| Input decoding and key normalization | `blocked` | `Shift+a` has contradictory evidence between current tests and user runtime |
| Window tree and split lifecycle | `blocked` | split view is user-reported non-working |
| Explorer window and actions | `blocked` | explorer is user-reported non-working |
| Terminal window integration | `partial` | covered by tests, but blocked domains share routing and focus paths |
| Viewport wrap safety | `partial` | no direct contradictory report, but mixed-window behavior needs revalidation |
| Cursor and grapheme safety | `partial` | no direct contradictory report, but tied to blocked mode-entry flow |
| Test harness fidelity | `blocked` | required screen-state E2E oracle is not yet the primary closure path |
| Source topology and workspace policy | `verified` | workspace structure exists and is testable |

## Release Rule

Release conformance is not met while any high-severity limitation is open.

A release may proceed only when all are true:

1. all high-severity rows in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) are closed
2. matching `*R` E2E rows in [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) pass using screen-state assertions
3. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md),
   [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md), and
   [/docs/todo/README.md](/docs/todo/README.md) are synchronized in the same change

Current state (2026-02-11): blocked.

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift rows: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction checklist: [/docs/todo/README.md](/docs/todo/README.md)
