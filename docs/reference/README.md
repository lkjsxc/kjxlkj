# Reference Documentation

Back: [/docs/README.md](/docs/README.md)

`/docs/reference/` is the canonical truth for current verified state.

## Authority

For current-state claims, use this precedence:

1. [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
2. [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
3. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
4. [/docs/reference/CI.md](/docs/reference/CI.md)
5. [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)

`/docs/spec/` remains the target behavior contract.

## Snapshot (2026-02-12)

The repository is in pivot reconstruction mode.

- Canonical docs target a web-notes server.
- Legacy terminal-editor behavior is out of scope.
- Runtime and deployment artifacts are intentionally absent in this docs-only reconstruction baseline.

## Synchronization Rule

Whenever status changes, synchronize:

- `CONFORMANCE.md`
- `LIMITATIONS.md`
- `DRIFT_MATRIX.md`
- `/docs/todo/README.md`

in one logical change.

## Related

- Target behavior: [/docs/spec/README.md](/docs/spec/README.md)
- Execution contract: [/docs/todo/README.md](/docs/todo/README.md)
