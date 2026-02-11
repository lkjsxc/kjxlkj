# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger records the strongest verified state as of the snapshot date.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | deterministic evidence exists and no high-severity contradiction is known |
| `partial` | behavior exists but verification or reachability is incomplete |
| `blocked` | high-severity mismatch is known and not yet closed |
| `unverified` | no trustworthy evidence currently exists |

## Current Snapshot (2026-02-11)

Implementation artifacts exist in this repository, but conformance is blocked by multiple
user-reported runtime failures in high-risk domains.

Strongest evidence order applied in this snapshot:

1. user-reported live runtime failures
2. deterministic integration/unit tests
3. static code-path inspection

Because high-severity blockers remain open, no affected domain may be marked `verified`.

## Evidence Summary

| Check | Status | Evidence Date | Evidence |
|---|---|---|---|
| Docs authority and precedence are defined | `verified` | 2026-02-11 | [/docs/README.md](/docs/README.md), [/docs/policy/README.md](/docs/policy/README.md) |
| Workspace grouping contract is documented | `verified` | 2026-02-11 | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) |
| Runtime E2E gate for blocker closure | `partial` | 2026-02-11 | matrix defined, implementation evidence incomplete |
| `Shift+a` append semantics | `blocked` | 2026-02-11 | user-reported failure; blocker `LIM-BLOCK-KEY-03` |
| Mixed-window split and navigation correctness | `blocked` | 2026-02-11 | user-reported failure; blockers `LIM-BLOCK-WIN-03` and `LIM-BLOCK-NAV-03` |
| Explorer launch/actions | `blocked` | 2026-02-11 | user-reported failure; blocker `LIM-BLOCK-EXP-03` |
| Terminal leaf lifecycle and integration | `blocked` | 2026-02-11 | user-reported instability; blocker `LIM-BLOCK-TERM-03` |
| Wrap and cursor display stability | `blocked` | 2026-02-11 | user-reported failures; blockers `LIM-BLOCK-WRAP-03` and `LIM-BLOCK-CURSOR-03` |

## Domain Status

| Domain | Status | Reason |
|---|---|---|
| Input decoding and key normalization | `blocked` | `Shift+a` behavior not trusted end-to-end |
| Window tree and split lifecycle | `blocked` | split/focus/close correctness reported broken |
| Mixed-window navigation (`Ctrl-w`) | `blocked` | directional and cyclic behavior unstable |
| Explorer window and actions | `blocked` | launch and actions not working reliably |
| Terminal window integration | `blocked` | lifecycle and mixed-window behavior unstable |
| Viewport wrap safety | `blocked` | long-line behavior reported buggy |
| Cursor visibility and grapheme safety | `blocked` | cursor display issues reported in churn scenarios |
| IME interaction around leader/window commands | `partial` | contract exists; live blocker evidence still required |
| Source topology and workspace policy | `partial` | structure exists but must be revalidated during rebuild |
| Documentation coverage and TODO integrity | `partial` | regenerated in this wave; requires verification gate pass |

## Release Rule

Release conformance is not met while any high-severity limitation is open.

Release status may switch from blocked only when all are true:

1. all high-severity rows in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
   are closed
2. matching `*R` E2E tests in [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
   pass deterministically
3. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md),
   [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md), and
   [/docs/todo/current/README.md](/docs/todo/current/README.md) are synchronized in the same change

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift rows: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Active TODO wave: [/docs/todo/current/README.md](/docs/todo/current/README.md)
