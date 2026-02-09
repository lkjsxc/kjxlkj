# Reference Documentation

Back: [/docs/README.md](/docs/README.md)

`/docs/reference/` tracks current verified behavior and explicit gaps.

## Authority

Within current-state reporting, use this order:

1. [CONFORMANCE.md](CONFORMANCE.md)
2. [LIMITATIONS.md](LIMITATIONS.md)
3. Evidence artifacts linked from those files
4. Supporting sub-ledgers in [conformance/](conformance/README.md)

`/docs/spec/` remains the target behavior contract.

## Purpose of This Directory

- prevent stale "implemented" claims
- make current gaps explicit and actionable
- provide evidence links for every current-state assertion

## Documents

| Document | Role |
|---|---|
| [CONFORMANCE.md](CONFORMANCE.md) | Current verified behavior ledger |
| [LIMITATIONS.md](LIMITATIONS.md) | Open user-visible mismatches |
| [CI.md](CI.md) | Verification gate definition |
| [RELEASE.md](RELEASE.md) | Release evidence process |
| [conformance/README.md](conformance/README.md) | Domain-specific conformance sub-ledgers |
| [COMPARISON.md](COMPARISON.md) | Non-authoritative comparison notes |
| [PLUGIN_MAPPING.md](PLUGIN_MAPPING.md) | Non-authoritative migration notes |

## Update Discipline

- Update conformance and limitations together.
- Every claim needs evidence links.
- Unknown or unverified behavior must be marked explicitly as unverified.

## Related

- Target behavior: [/docs/spec/README.md](/docs/spec/README.md)
- Reconstruction plan: [/docs/todo/README.md](/docs/todo/README.md)
