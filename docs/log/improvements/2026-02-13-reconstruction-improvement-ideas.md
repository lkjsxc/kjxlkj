# Reconstruction Improvement Ideas (2026-02-13)

Back: [/docs/log/improvements/README.md](/docs/log/improvements/README.md)

## Scope

Improvements that increase deterministic reconstruction quality from docs-only baseline.

## Ideas

1. Add a generated `docs/reference/EVIDENCE_INDEX.md` that maps each TODO wave to
   required verification commands and latest passing audit document.
2. Add a deterministic `docs-integrity` script that enforces:
   - every TODO file has `## Relevant Documents`
   - every TODO proof link resolves
   - no TODO file exceeds structural policy limits
3. Add a machine-readable wave state file (for example `docs/todo/waves/state.json`)
   as a projection of markdown checkboxes to prevent ledger drift.
4. Add a reconstruction bootstrap guide that translates Stage 01 source-layout
   requirements into a concrete file-by-file scaffold checklist.
5. Add an explicit “baseline mode” marker in TODO files to distinguish
   historical-completed waves from current-runtime-complete waves.

## Source-Length Note

- No runtime source files are currently present in this repository baseline.
- Therefore, there are no runtime source files exceeding 200 lines at this time.
