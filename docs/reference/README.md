# Reference Documentation

Back: [/docs/README.md](/docs/README.md)

`/docs/reference/` tracks the strongest currently available evidence.

## Authority

Within current-state reporting, use this order:

1. [CONFORMANCE.md](CONFORMANCE.md)
2. [LIMITATIONS.md](LIMITATIONS.md)
3. [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
4. domain sub-ledgers in [conformance/](conformance/README.md)

`/docs/spec/` remains the target behavior contract.

## Evidence Discipline

- strongest evidence wins
- user-reported runtime failure outranks weaker model-level test confidence
- no domain may be marked `verified` when a high-severity blocker remains open

## Documents

| Document | Role |
|---|---|
| [CONFORMANCE.md](CONFORMANCE.md) | current verified and blocked status |
| [LIMITATIONS.md](LIMITATIONS.md) | active user-visible mismatches |
| [DRIFT_MATRIX.md](DRIFT_MATRIX.md) | requirement-by-requirement mismatch tracking |
| [CI.md](CI.md) | verification profile definition |
| [RELEASE.md](RELEASE.md) | release evidence process |
| [conformance/README.md](conformance/README.md) | domain sub-ledgers |
| [COMPARISON.md](COMPARISON.md) | non-authoritative comparison notes |
| [PLUGIN_MAPPING.md](PLUGIN_MAPPING.md) | non-authoritative migration notes |

## Update Discipline

- update conformance, limitations, and drift matrix in the same change
- every `verified` claim needs deterministic evidence
- unresolved behavior must stay `partial`, `blocked`, or `unverified`

## Related

- Target behavior: [/docs/spec/README.md](/docs/spec/README.md)
- Reconstruction plan: [/docs/todo/README.md](/docs/todo/README.md)
