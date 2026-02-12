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

`/docs/spec/` remains the target behavior contract.

## Snapshot (2026-02-12)

The repository is in hard-pivot reconstruction mode.

- Canonical docs target a multi-user workspace platform on `/api` and `/ws`.
- Canonical docs include librarian-agent contracts for autonomous documentation
  structuring with OpenRouter/LM Studio compatible providers.
- Runtime and deployment artifacts remain intentionally absent in this baseline.

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
