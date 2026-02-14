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

`/docs/spec/` remains the target behavior contract.

## Snapshot (2026-02-14)

- All in Docs governance is active.
- Runtime source scaffold has been partially reconstructed in Rust + TypeScript.
- Top-level reconstruction start-gate read/open rows are checked.
- Top-level HTTP, WebSocket, and security restoration rows are checked with deterministic evidence.
- Top-level automation/librarian restoration row is checked with deterministic evidence.
- Saved-view API lifecycle is executable and deterministic-test verified.
- Frontend regression tests are executable with deterministic `REG-IMP/REG-USR/REG-UX` subset coverage.
- Top-level typed frontend shell/editor-flow rows are checked with deterministic evidence.
- Top-level runtime structure-alignment row is checked with deterministic evidence.
- Deterministic regression pack and completion gate rows remain partially open.
- Docker artifact gate rows are complete with deterministic evidence.
- Every TODO checklist row in `docs/todo/` now links directly to a docs file.
- Release gate is not satisfied until typed runtime reconstruction is complete.

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
