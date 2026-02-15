# Reference Documentation

Back: [/docs/README.md](/docs/README.md)

`/docs/reference/` is the canonical truth for verified current state.

## Authority

For state claims, use this order:

1. [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
2. [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
3. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
4. [/docs/reference/CI.md](/docs/reference/CI.md)
5. [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
6. [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)
7. [/docs/reference/IMPROVEMENT_BACKLOG.md](/docs/reference/IMPROVEMENT_BACKLOG.md)

`/docs/spec/` remains the target behavior contract.

## Snapshot (2026-02-15)

- All in Docs governance is active.
- Runtime source code has been intentionally removed for clean reconstruction.
- Runtime manifests and container artifacts are intentionally absent.
- Runtime configuration contract is split between `data/config.json` and `.env`.
- TODO program is reset to unchecked reconstruction baseline.
- Improvement backlog is canonicalized in `IMPROVEMENT_BACKLOG.md`.
- Release gate remains blocked until full acceptance evidence is complete.

## Synchronization Rule

Whenever state changes, update these in one logical change:

- `CONFORMANCE.md`
- `LIMITATIONS.md`
- `DRIFT_MATRIX.md`
- `RELEASE.md`
- `/docs/todo/README.md`

## Related

- Target behavior: [/docs/spec/README.md](/docs/spec/README.md)
- Execution contract: [/docs/todo/README.md](/docs/todo/README.md)
- Stage-proof map: [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)
- Improvement backlog: [/docs/reference/IMPROVEMENT_BACKLOG.md](/docs/reference/IMPROVEMENT_BACKLOG.md)
