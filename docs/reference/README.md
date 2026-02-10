# Reference Documentation

Back: [/docs/README.md](/docs/README.md)

`/docs/reference/` tracks current verified behavior and explicit gaps.

## Authority

Within current-state reporting, use this order:

1. [CONFORMANCE.md](CONFORMANCE.md)
2. [LIMITATIONS.md](LIMITATIONS.md)
3. [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
4. domain sub-ledgers in [conformance/](conformance/README.md)

`/docs/spec/` remains the target behavior contract.

## Documents

| Document | Role |
|---|---|
| [CONFORMANCE.md](CONFORMANCE.md) | current verified behavior summary |
| [LIMITATIONS.md](LIMITATIONS.md) | open user-visible mismatches |
| [DRIFT_MATRIX.md](DRIFT_MATRIX.md) | requirement-by-requirement mismatch matrix |
| [CI.md](CI.md) | verification gate definition |
| [RELEASE.md](RELEASE.md) | release evidence process |
| [conformance/README.md](conformance/README.md) | domain sub-ledgers |
| [COMPARISON.md](COMPARISON.md) | non-authoritative comparison notes |
| [PLUGIN_MAPPING.md](PLUGIN_MAPPING.md) | non-authoritative migration notes |

## Update Discipline

- update conformance, limitations, and drift matrix together
- every `verified` claim needs deterministic evidence
- unknown behavior must be marked as `unverified`

## Related

- Target behavior: [/docs/spec/README.md](/docs/spec/README.md)
- Reconstruction plan: [/docs/todo/README.md](/docs/todo/README.md)
