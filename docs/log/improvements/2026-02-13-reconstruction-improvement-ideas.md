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

- Runtime source files exceeding 200 lines in current state:
   - `src/crates/app/kjxlkj-server/src/handlers/automation.rs` (2159)
   - `src/crates/app/kjxlkj-server/tests/automation_provider_adapter.rs` (1274)
   - `src/crates/app/kjxlkj-server/static/index.html` (1239)
   - `src/crates/db/kjxlkj-db/src/repos/notes.rs` (804)
   - `src/crates/app/kjxlkj-server/src/handlers/notes.rs` (658)
   - `src/crates/app/kjxlkj-server/src/handlers/ws.rs` (538)
   - `src/crates/app/kjxlkj-server/tests/ws_flow.rs` (477)
   - `src/crates/db/kjxlkj-db/src/repos/automation.rs` (465)
   - `src/crates/app/kjxlkj-server/tests/performance_smoke.rs` (426)
   - `src/crates/app/kjxlkj-server/src/handlers/admin.rs` (331)
   - `src/crates/app/kjxlkj-server/tests/automation_run_flow.rs` (325)
   - `src/crates/app/kjxlkj-server/tests/security_hardening.rs` (319)
   - `src/crates/app/kjxlkj-server/src/handlers/auth.rs` (277)
   - `src/crates/app/kjxlkj-server/tests/automation_rules_api.rs` (273)
   - `src/crates/app/kjxlkj-server/src/handlers/views.rs` (215)
