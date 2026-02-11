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
- passing trace-only tests do not close user-visible runtime failures
- no domain may be `verified` while a high-severity blocker is open

## Current Snapshot (2026-02-11)

The repository has a reconstructed Rust workspace and passing targeted tests, but
release conformance is blocked by user-reported runtime failures in split view,
file explorer, and `Shift+a` behavior under real usage.

## Documents

| Document | Role |
|---|---|
| [CONFORMANCE.md](CONFORMANCE.md) | current verified and blocked status |
| [LIMITATIONS.md](LIMITATIONS.md) | active user-visible mismatches |
| [DRIFT_MATRIX.md](DRIFT_MATRIX.md) | requirement-level mismatch tracking |
| [CI.md](CI.md) | verification profile definitions |
| [RELEASE.md](RELEASE.md) | release evidence and gate rules |
| [conformance/README.md](conformance/README.md) | consolidation policy |
| [COMPARISON.md](COMPARISON.md) | non-authoritative comparison notes |
| [PLUGIN_MAPPING.md](PLUGIN_MAPPING.md) | non-authoritative migration notes |

## Update Discipline

- update `CONFORMANCE`, `LIMITATIONS`, and `DRIFT_MATRIX` together
- when drift is discovered, downgrade status first, then plan closure
- do not keep stale green status when contradictory evidence exists

## Related

- Target behavior: [/docs/spec/README.md](/docs/spec/README.md)
- Reconstruction checklist: [/docs/todo/README.md](/docs/todo/README.md)
