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

Docs-only baseline is active. Implementation artifacts are intentionally absent.

Repository evidence in this snapshot:

- `src/` is absent
- `Cargo.toml`, `Cargo.lock`, and `rust-toolchain.toml` are absent
- `/docs` control-plane and canonical specs are present

Because runtime artifacts are absent, runtime behavior domains cannot be marked `verified`.

## Evidence Summary

| Check | Status | Evidence Date | Evidence |
|---|---|---|---|
| Docs authority and precedence are defined | `verified` | 2026-02-11 | [/docs/README.md](/docs/README.md), [/docs/policy/README.md](/docs/policy/README.md) |
| Docs-only baseline is active | `verified` | 2026-02-11 | root source/workspace artifacts absent by design |
| Reconstruction requirements are specified | `verified` | 2026-02-11 | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md), [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) |
| Runtime E2E gate for blocker closure is defined | `partial` | 2026-02-11 | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) |
| Runtime feature conformance | `unverified` | 2026-02-11 | runtime artifacts intentionally absent |

## Domain Status

| Domain | Status | Reason |
|---|---|---|
| Input decoding and key normalization | `unverified` | runtime implementation absent |
| Window tree and split lifecycle | `unverified` | runtime implementation absent |
| Mixed-window navigation (`Ctrl-w`) | `unverified` | runtime implementation absent |
| Explorer window and actions | `unverified` | runtime implementation absent |
| Terminal window integration | `unverified` | runtime implementation absent |
| Viewport wrap safety | `unverified` | runtime implementation absent |
| Cursor visibility and grapheme safety | `unverified` | runtime implementation absent |
| IME interaction around leader/window commands | `unverified` | runtime implementation absent |
| Source topology and workspace policy | `blocked` | required source/workspace tree not reconstructed |
| Documentation coverage and TODO integrity | `partial` | docs present; implementation-gate verification pending |

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
