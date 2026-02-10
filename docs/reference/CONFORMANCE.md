# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger records what is currently verified.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | confirmed by deterministic evidence in current repo state |
| `partial` | partly available with user-visible gaps |
| `scaffold-only` | structural artifacts exist but runtime path is incomplete |
| `unverified` | no current evidence |

## Current Snapshot (2026-02-10)

Repository is intentionally in docs-only standby baseline.

## Baseline Verification

| Check | Status | Evidence |
|---|---|---|
| Source artifacts removed | `verified` | repository root contains docs-first baseline only |
| Workspace/build manifests removed | `verified` | no `Cargo.toml`, `Cargo.lock`, `src/` |
| CI/release workflow artifacts removed | `verified` | `.github/workflows/` reset for future regeneration |
| TODO standby state | `verified` | all implementation-phase items remain unchecked |
| Doc integrity rules | `verified` | no broken links, no `../` links, full TODO doc coverage |

## Domain Summary

| Domain | Status | Note |
|---|---|---|
| Runtime behavior domains | `unverified` | no source implementation present in standby baseline |
| Spec authority | `verified` | `/docs/spec/` is canonical target for reimplementation |
| Reconstruction controls | `verified` | `/docs/todo/` governs rebuild sequencing and gates |

## Claim Rules

Any runtime conformance claim MUST remain absent until reimplementation produces:

1. reachable behavior from real input path
2. deterministic verification evidence
3. synchronized updates to `LIMITATIONS` and `DRIFT_MATRIX`

## Related

- Open gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Mismatch matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/current/README.md](/docs/todo/current/README.md)
