# Reference Documentation

Back: [/docs/README.md](/docs/README.md)

`/docs/reference/` records the strongest current-state evidence.

## Authority

For current-state claims, use this precedence:

1. [CONFORMANCE.md](CONFORMANCE.md)
2. [LIMITATIONS.md](LIMITATIONS.md)
3. [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
4. [CI.md](CI.md)
5. [RELEASE.md](RELEASE.md)

`/docs/spec/` remains the target behavior contract.

## Evidence Rules

- strongest evidence wins
- user-reported runtime failure outranks weaker automated confidence
- no domain may be `verified` without deterministic evidence
- in docs-only baseline, implementation claims are `unverified` or `spec-only`

## Current Snapshot (2026-02-11)

The repository is intentionally in docs-only baseline state for reconstruction.
Implementation artifacts were removed to prepare clean regeneration from docs.

## Documents

| Document | Role |
|---|---|
| [CONFORMANCE.md](CONFORMANCE.md) | current verified and blocked status |
| [LIMITATIONS.md](LIMITATIONS.md) | active user-visible and baseline mismatches |
| [DRIFT_MATRIX.md](DRIFT_MATRIX.md) | requirement-level mismatch tracking |
| [CI.md](CI.md) | verification profile definitions |
| [RELEASE.md](RELEASE.md) | release evidence and gate rules |
| [conformance/README.md](conformance/README.md) | consolidation policy |
| [COMPARISON.md](COMPARISON.md) | non-authoritative comparison notes |
| [PLUGIN_MAPPING.md](PLUGIN_MAPPING.md) | non-authoritative migration notes |

## Update Discipline

- update `CONFORMANCE`, `LIMITATIONS`, and `DRIFT_MATRIX` together
- when implementation state changes, refresh snapshot date and baseline statement
- never keep stale implementation status after major repo-state transitions

## Related

- Target behavior: [/docs/spec/README.md](/docs/spec/README.md)
- Reconstruction checklist: [/docs/todo/README.md](/docs/todo/README.md)
