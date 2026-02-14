# Audits

Back: [/docs/log/README.md](/docs/log/README.md)

Post-change audits and evidence summaries.

## Documents

| Document | Purpose |
|---|---|
| [2026-02-14-api-saved-views-contract.md](2026-02-14-api-saved-views-contract.md) | deterministic evidence for replacing saved-view API stubs with executable CRUD behavior, auth/csrf/role enforcement, and lifecycle test coverage |
| [2026-02-14-automation-librarian-contract-sync.md](2026-02-14-automation-librarian-contract-sync.md) | deterministic evidence for automation/librarian rule/run/review baseline execution, provider/protocol validation, and ledger synchronization |
| [2026-02-14-regression-pack-slice.md](2026-02-14-regression-pack-slice.md) | deterministic evidence for restored frontend/backend regression execution covering idempotency fallback, pre-auth `401` behavior, title propagation, status-rail errors, and backend replay/conflict contracts |
| [2026-02-13-docker-artifact-contract-sync.md](2026-02-13-docker-artifact-contract-sync.md) | deterministic mismatch closure for app-runtime Docker contract, TODO gate hardening, and verified health/readiness startup flow |
| [2026-02-13-all-in-docs-baseline-reset.md](2026-02-13-all-in-docs-baseline-reset.md) | spec/code/todo reset audit for All in Docs baseline, typed-language policy hardening, and ledger synchronization |
| [2026-02-13-docs-launch-and-todo-start-gate.md](2026-02-13-docs-launch-and-todo-start-gate.md) | deterministic reconciliation of docs-launch artifacts and TODO start-gate checks |
| [2026-02-14-frontend-shell-editor-flow.md](2026-02-14-frontend-shell-editor-flow.md) | deterministic evidence for typed note-first shell restoration, synced/draft editor state model, autosave debounce, title propagation, and save-status rail behavior |
| [2026-02-14-http-ws-security-contract.md](2026-02-14-http-ws-security-contract.md) | deterministic evidence for reachable HTTP+WS contract slice, setup/session/csrf/rbac enforcement, note conflict semantics, and ws idempotency replay behavior |
| [2026-02-14-runtime-bootstrap-and-docker-gate.md](2026-02-14-runtime-bootstrap-and-docker-gate.md) | deterministic evidence for runtime bootstrap restoration, root Docker artifact compliance, compose config validation, and health/readiness smoke checks |
| [2026-02-14-structure-contract-slice.md](2026-02-14-structure-contract-slice.md) | deterministic evidence for final-file-structure alignment of backend crate topology, frontend TypeScript module paths, workspace manifest members, and <=200-line runtime source compliance |
| [2026-02-14-todo-start-gate-ledger-sync.md](2026-02-14-todo-start-gate-ledger-sync.md) | deterministic synchronization of TODO start-gate closure, reference ledgers, and recursive documentation logging structure |
| [2026-02-13-reconstruction-reset-sync.md](2026-02-13-reconstruction-reset-sync.md) | historical reset-sync audit captured before the All in Docs hard reset |
| [2026-02-12-implementation-user-findings.md](2026-02-12-implementation-user-findings.md) | consolidated defects and UX gaps discovered during implementation and user feedback |
| [2026-02-12-librarian-doc-sync.md](2026-02-12-librarian-doc-sync.md) | mismatch closure audit for librarian-agent documentation and ledger sync |
| [2026-02-12-stage-00-canonical-reset.md](2026-02-12-stage-00-canonical-reset.md) | deterministic closure evidence for Stage 00 canonical reset waves |
| [2026-02-13-stage-01-workspace-foundation.md](2026-02-13-stage-01-workspace-foundation.md) | deterministic closure evidence for Stage 01 workspace foundation waves |
| [2026-02-13-stage-02-wave-020-notes-core.md](2026-02-13-stage-02-wave-020-notes-core.md) | deterministic closure evidence for Stage 02 Wave 020 notes CRUD/history/versioning |
| [2026-02-13-stage-02-collaborative-notes-core.md](2026-02-13-stage-02-collaborative-notes-core.md) | deterministic closure evidence for Stage 02 Waves 021-022 and overall stage verification |
| [2026-02-13-stage-03-wave-030-saved-views.md](2026-02-13-stage-03-wave-030-saved-views.md) | deterministic closure evidence for Stage 03 Wave 030 saved views and role-denial coverage |
| [2026-02-13-stage-03-wave-031-command-palette.md](2026-02-13-stage-03-wave-031-command-palette.md) | deterministic closure evidence for Stage 03 Wave 031 command palette, command workflows, and setup-lock UI behavior |
| [2026-02-13-stage-03-wave-032-graph-responsive-shell.md](2026-02-13-stage-03-wave-032-graph-responsive-shell.md) | deterministic closure evidence for Stage 03 Wave 032 graph explorer, responsive shell, autosave/title propagation, and minimal editor chrome |
| [2026-02-13-stage-04-wave-040-automation-rules.md](2026-02-13-stage-04-wave-040-automation-rules.md) | deterministic closure evidence for Stage 04 Wave 040 automation rule CRUD, validation, and authorization checks |
| [2026-02-13-stage-04-wave-041-automation-runs.md](2026-02-13-stage-04-wave-041-automation-runs.md) | deterministic closure evidence for Stage 04 Wave 041 automation run idempotency/state-machine and workspace-event replay |
| [2026-02-13-stage-04-wave-042-export-backup-jobs.md](2026-02-13-stage-04-wave-042-export-backup-jobs.md) | deterministic closure evidence for Stage 04 Wave 042 export/backup jobs, artifact status paths, and telemetry signals |
| [2026-02-13-stage-05-wave-050-reliability-guards.md](2026-02-13-stage-05-wave-050-reliability-guards.md) | deterministic closure evidence for Stage 05 Wave 050 finding-mapped reliability regression guards and non-flakiness checks |
| [2026-02-13-stage-05-wave-051-security-hardening.md](2026-02-13-stage-05-wave-051-security-hardening.md) | deterministic closure evidence for Stage 05 Wave 051 CSRF/session/cookie hardening, role boundaries, and rate-limiting checks |
| [2026-02-13-stage-05-wave-052-perf-ops-gate.md](2026-02-13-stage-05-wave-052-perf-ops-gate.md) | deterministic closure evidence for Stage 05 Wave 052 perf/ops baseline gate with restart recovery and defer-recorded PERF-03 prep |
| [2026-02-13-stage-06-wave-060-provider-adapter.md](2026-02-13-stage-06-wave-060-provider-adapter.md) | deterministic closure evidence for Stage 06 Wave 060 provider adapters, timeout/retry/failure classification, and run metadata persistence |
| [2026-02-13-stage-06-wave-061-librarian-payload-contract.md](2026-02-13-stage-06-wave-061-librarian-payload-contract.md) | deterministic closure evidence for Stage 06 Wave 061 librarian action schema validation, operation-report payloads, and scope/safety guards |
| [2026-02-13-stage-06-wave-062-xml-parser-retry.md](2026-02-13-stage-06-wave-062-xml-parser-retry.md) | deterministic closure evidence for Stage 06 Wave 062 xml_attrless parser validation, repair retries, and parse diagnostics retention |
| [2026-02-13-stage-07-websocket-sync.md](2026-02-13-stage-07-websocket-sync.md) | deterministic closure evidence for Stage 07 librarian websocket event typing, replay cursor determinism, and WS-06 acceptance ordering checks |
| [2026-02-13-stage-08-librarian-ux-and-static-delivery.md](2026-02-13-stage-08-librarian-ux-and-static-delivery.md) | deterministic closure evidence for Stage 08 librarian control/run-review UX, operation decision audit linkage, and responsive keyboard-first shell constraints |
| [2026-02-13-stage-09-wave-090-ci-librarian-profiles.md](2026-02-13-stage-09-wave-090-ci-librarian-profiles.md) | deterministic closure evidence for Stage 09 Wave 090 librarian CI profile matrix execution and rerun stability checks |
| [2026-02-13-stage-09-wave-091-perf-ops-archive.md](2026-02-13-stage-09-wave-091-perf-ops-archive.md) | deterministic closure evidence for Stage 09 Wave 091 perf/ops archive execution (`PERF-01`/`PERF-02`/`PERF-03`, `OPS-01`/`OPS-02`) and provider retry/failure diagnostics |
| [2026-02-13-stage-09-wave-092-release-closure.md](2026-02-13-stage-09-wave-092-release-closure.md) | deterministic closure evidence for Stage 09 Wave 092 full release-profile pass, high-severity blocker closure, and reference-ledger synchronization |
