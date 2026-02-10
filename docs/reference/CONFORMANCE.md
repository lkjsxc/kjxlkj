# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger records what is currently verified with deterministic evidence.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | confirmed by deterministic evidence in current repo state |
| `partial` | partly available but not yet release-safe |
| `unverified` | no trustworthy runtime evidence is currently available |
| `blocked` | known high-severity mismatch remains open |

## Current Snapshot (2026-02-10)

The repository is now in a docs-only standby baseline.

- source code and workspace manifests are intentionally absent
- CI workflow for reconstructed profile is intentionally absent
- runtime behavior claims are deferred until reconstruction

## Verification Evidence Available

| Check | Status | Evidence Date | Evidence |
|---|---|---|---|
| Docs-only repository shape | `verified` | 2026-02-10 | source/workspace artifacts removed from baseline |
| TODO doc coverage completeness | `verified` | 2026-02-10 | direct-link inventory covers all markdown docs |
| Markdown link integrity | `verified` | 2026-02-10 | no broken internal markdown links |
| Runtime build/tests | `unverified` | 2026-02-10 | workspace absent by design in standby baseline |

## Domain Summary

| Domain | Status | Note |
|---|---|---|
| Input decoding and key normalization | `unverified` | implementation absent in docs-only baseline |
| Cursor semantics and display | `unverified` | implementation absent in docs-only baseline |
| Window tree and split management | `unverified` | implementation absent in docs-only baseline |
| Explorer window and actions | `unverified` | implementation absent in docs-only baseline |
| Terminal window integration | `unverified` | implementation absent in docs-only baseline |
| Wrapping and viewport safety | `unverified` | implementation absent in docs-only baseline |
| Service integrations (LSP/Git/Index/FS) | `unverified` | implementation absent in docs-only baseline |
| Source topology and workspace layout | `unverified` | grouped crate-root layout is documented but not yet regenerated |
| Documentation and TODO integrity | `verified` | blocker-first reconstruction controls are active |

## Release Readiness Rule

The implementation MUST NOT be considered conformant for release until all critical blockers in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) are closed with:

1. reconstructed runtime behavior from real input paths
2. deterministic regression and live E2E evidence
3. synchronized `CONFORMANCE`, `LIMITATIONS`, `DRIFT_MATRIX`, and TODO updates

## Related

- Open blockers: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction plan: [/docs/todo/current/README.md](/docs/todo/current/README.md)
