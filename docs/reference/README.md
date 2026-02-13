# Reference Documentation

Back: [/docs/README.md](/docs/README.md)

`/docs/reference/` is the canonical truth for verified repository state.

## Authority

For state claims, use this precedence:

1. [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
2. [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
3. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
4. [/docs/reference/CI.md](/docs/reference/CI.md)
5. [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
6. [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

`/docs/spec/` remains the target behavior contract.

## Snapshot (2026-02-13)

The repository is in docs-only reconstruction reset mode.

- Runtime and source artifacts are intentionally removed.
- Historical implementation/user findings (`IMP-*`, `USR-*`) are preserved and
  mandatory for rebuild regression packs.
- UX requirements are consolidated under
  [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md).
- Docker Compose currently launches the documentation environment only.

## Synchronization Rule

Whenever status changes, synchronize these in one logical change:

- `CONFORMANCE.md`
- `LIMITATIONS.md`
- `DRIFT_MATRIX.md`
- `RELEASE.md`
- `/docs/todo/README.md`

## Related

- Target behavior: [/docs/spec/README.md](/docs/spec/README.md)
- Execution contract: [/docs/todo/README.md](/docs/todo/README.md)
- Stage-proof map: [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)
