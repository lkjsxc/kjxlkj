# Reconstruction Improvement Ideas (2026-02-13)

Back: [/docs/log/improvements/README.md](/docs/log/improvements/README.md)

## Scope

High-leverage improvements for deterministic reconstruction quality.

## Carried Into Current Rewrite

1. [x] Preserve a stage-to-proof map in `docs/reference/EVIDENCE_INDEX.md`.
2. [x] Keep a deterministic reconstruction bootstrap guide and update it to typed runtime scaffolding.
3. [x] Add explicit baseline mode doctrine clarification (`All in Docs` vs `docs-only`).

## Still Open (Next High-Value)

1. [ ] Add a deterministic `docs-integrity` script to enforce:
   - TODO `## Relevant Documents` presence
   - internal link validity
   - structure policy constraints
2. [ ] Add machine-readable wave state projection (for example `docs/todo/waves/state.json`).
3. [ ] Add typed-contract lint gate (`TYPE-03` scanner + TS strict config validator).

## Notes

- Source/runtime artifacts are treated as disposable projections.
- Canonical value remains in docs even when runtime snapshots are absent.

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- TODO baseline: [/docs/todo/README.md](/docs/todo/README.md)
