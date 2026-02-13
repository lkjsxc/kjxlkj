# Reconstruction Improvement Ideas (2026-02-13)

Back: [/docs/log/improvements/README.md](/docs/log/improvements/README.md)

## Scope

Improvements that increase deterministic reconstruction quality from docs-only baseline.

## Ideas

1. [x] Add a generated `docs/reference/EVIDENCE_INDEX.md` that maps each TODO wave to
   required verification commands and latest passing audit document.
2. Add a deterministic `docs-integrity` script that enforces:
   - every TODO file has `## Relevant Documents`
   - every TODO proof link resolves
   - no TODO file exceeds structural policy limits
3. Add a machine-readable wave state file (for example `docs/todo/waves/state.json`)
   as a projection of markdown checkboxes to prevent ledger drift.
4. [x] Add a reconstruction bootstrap guide that translates Stage 01 source-layout
   requirements into a concrete file-by-file scaffold checklist.
5. Add an explicit “baseline mode” marker in TODO files to distinguish
   historical-completed waves from current-runtime-complete waves.

## Source-Length Note

- Current docs-only baseline has no `src/` directory.
- Deterministic check command:
   `if [ -d src ]; then find src -type f ... ; else echo NO_SOURCE_DIR; fi`
- Result: `NO_SOURCE_DIR`.
- Action: record file-length over-200 rows after Stage 01 source scaffold exists.
