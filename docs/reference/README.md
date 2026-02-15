# Reference Documentation

Back: [/docs/README.md](/docs/README.md)

`/docs/reference/` is the canonical truth for verified repository state.

## Authority

For state claims, use this order:

1. [CONFORMANCE.md](CONFORMANCE.md)
2. [LIMITATIONS.md](LIMITATIONS.md)
3. [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
4. [CI.md](CI.md)
5. [RELEASE.md](RELEASE.md)
6. [EVIDENCE_INDEX.md](EVIDENCE_INDEX.md)
7. [IMPROVEMENT_BACKLOG.md](IMPROVEMENT_BACKLOG.md)

## Snapshot (2026-02-15)

- Repository is intentionally in docs-only baseline.
- Runtime source tree is removed and must be rebuilt from docs.
- `tmp/` intake material is transitional and must be deleted after capture.
- Canonical improvement points are tracked in `IMPROVEMENT_BACKLOG.md`.

## Synchronization Rule

Whenever status changes, update in one logical change:

- `CONFORMANCE.md`
- `LIMITATIONS.md`
- `DRIFT_MATRIX.md`
- `RELEASE.md`
- `/docs/todo/README.md`

## Related

- Target behavior: [/docs/spec/README.md](/docs/spec/README.md)
- Execution plan: [/docs/todo/README.md](/docs/todo/README.md)
