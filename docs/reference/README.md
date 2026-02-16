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
8. [logs/README.md](logs/README.md)

## Snapshot (2026-02-16)

- Repository is in reconstructed-runtime state.
- Runtime source tree, Cargo manifests, and scripts are present.
- Docker artifacts are intentionally absent per policy.
- Evidence and improvement logs are stored under `docs/reference/logs/`.

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
