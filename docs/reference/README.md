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
8. [TEST_MATRIX.md](TEST_MATRIX.md)
9. [TODO_TRACE_MATRIX.md](TODO_TRACE_MATRIX.md)

## Snapshot (2026-02-25)

- Repository is managed as docs-first with runtime rebuilt only through TODO waves.
- Runtime status and readiness are tracked in `CONFORMANCE.md` and `RELEASE.md`.
- Acceptance coverage is tracked in `TEST_MATRIX.md`.
- TODO-to-artifact traceability is tracked in `TODO_TRACE_MATRIX.md`.

## Synchronization Rule

Whenever status changes, update in one logical change:

- `CONFORMANCE.md`
- `LIMITATIONS.md`
- `DRIFT_MATRIX.md`
- `RELEASE.md`
- `TEST_MATRIX.md`
- `TODO_TRACE_MATRIX.md`
- `/docs/todo/README.md`

## Related

- Target behavior: [/docs/spec/README.md](/docs/spec/README.md)
- Execution plan: [/docs/todo/README.md](/docs/todo/README.md)
