# Interleaved Test Schedule

The schedule is deterministic. Execute gates in ascending order.

| Gate | Trigger Point | Tests | Expected Evidence |
| --- | --- | --- | --- |
| G00 | After Phase 00 | `T00-topology-baseline`, `T01-readme-per-dir` | Baseline file inventory and README presence snapshot |
| G01 | After Phase 01 | `T01-readme-per-dir`, `T08-entrypoint-links` | New restructuring TOCs resolve to child docs |
| G02 | After Phase 02 | `T02-phase-order`, `T03-section-shape` | Ten ordered phase files with normalized sections |
| G03 | After Phase 03 | `T04-intent-catalog`, `T05-phase-intent-links` | Intent IDs exist and are linked from phases |
| G04 | After Phase 04 | `T06-schedule-order`, `T07-gate-evidence` | Interleaved schedule is complete and ordered |
| G05 | After Phase 05 | `T08-coverage-completeness`, `T09-link-target-exists` | Matrix covers all docs markdown links |
| G06 | After Phase 06 | `T10-root-link`, `T11-docs-link` | Root/docs readmes link restructuring TOCs |
| G07 | After Phase 07 | `T12-topology-check`, `T13-line-limit-check`, `T14-link-sanity` | Validation suite passes with no blockers |
| G08 | After Phase 08 | `T15-layout-contract` | Layout governance file includes restructuring directory |
| G09 | After Phase 09 | `T16-final-audit`, `T17-status-update` | Final summary and SQL status update completed |

## Test ID Definitions

- `T00-topology-baseline`: list `docs/**/*.md` in lexical order.
- `T01-readme-per-dir`: ensure each docs directory contains one `README.md`.
- `T02-phase-order`: verify phases exist from `00` to `09` with no gaps.
- `T03-section-shape`: verify required six section headings exist in each phase file.
- `T04-intent-catalog`: verify `FI-00`..`FI-09` are present in catalog.
- `T05-phase-intent-links`: verify each phase links to at least one intent anchor.
- `T06-schedule-order`: verify gate order is `G00`..`G09`.
- `T07-gate-evidence`: verify every gate defines expected evidence text.
- `T08-coverage-completeness`: verify matrix row set equals markdown file set under `docs/`.
- `T09-link-target-exists`: verify matrix links resolve to existing files.
- `T10-root-link`: verify root `README.md` links to restructuring docs.
- `T11-docs-link`: verify `docs/README.md` links to restructuring docs.
- `T12-topology-check`: validate recursive README coverage in docs tree.
- `T13-line-limit-check`: validate every docs markdown file has `<300` lines.
- `T14-link-sanity`: validate local markdown links used by readmes/restructuring docs.
- `T15-layout-contract`: verify docs layout contract includes `docs/restructuring/`.
- `T16-final-audit`: verify final file list and coverage explanation are present.
- `T17-status-update`: verify SQL todo update executed with terminal status.
