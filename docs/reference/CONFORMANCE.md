# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger reports only currently verified behavior.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | deterministic evidence exists and no high-severity contradiction is open |
| `partial` | behavior exists but verification is incomplete |
| `blocked` | known user-visible failure or contradiction is open |
| `unverified` | no trustworthy runtime evidence exists |
| `spec-only` | behavior is defined in spec only |

## Current Snapshot (2026-02-13)

High-confidence statement:

- Runtime reconstruction has started with Stage 01 foundation implementation.
- Source/runtime artifacts now exist under `/src/crates/`.
- Stage 01 includes deterministic evidence for migration bootstrap, startup health/readiness,
  auth/session baseline, and RBAC membership controls.
- Stage 02 includes deterministic evidence for notes CRUD/history/rollback/versioning,
  workspace/note websocket subscribe+patch replay semantics, and metadata/tags/backlinks/search baseline APIs.
- Stage 03 Wave 030 includes deterministic evidence for saved views persistence APIs
  and role-denial mutation controls.
- Stage 03 Waves 031-032 include deterministic evidence for a root-served
  responsive shell with command palette flows, setup-lock login-only switch,
  backlinks graph/context behavior, and autosave/title propagation paths.
- Stage 04 Wave 040 includes deterministic evidence for automation rule CRUD,
  deterministic validation, and role-guarded mutation controls.
- Stage 04 Wave 041 includes deterministic evidence for automation run
  queue/running/success/failure lifecycle, idempotent run keys by triggering
  event, run status retrieval, and workspace replay of automation run events.
- Stage 04 Wave 042 includes deterministic evidence for admin markdown export
  and SQL backup job lifecycle APIs with artifact paths and telemetry events.
- Stage 05 Wave 050 includes deterministic finding-mapped reliability regressions
  (`IMP-*`/`USR-*`) with replay/idempotency/conflict boundary guard coverage.
- Stage 05 Wave 051 includes CSRF/session/cookie hardening checks, role-boundary
  enforcement tests, and setup/login rate-limit coverage.
- Stage 05 Wave 052 includes PERF-01/PERF-02 smoke baseline and OPS restart
  recovery verification with explicit PERF-03 defer recording for librarian stages.
- Stage 06 Wave 060 includes deterministic evidence for librarian provider adapters
  (`openrouter`, `lmstudio`), bounded timeout/retry/failure classification,
  and provider/model metadata persistence on automation run records.
- Stage 06 Wave 061 includes deterministic evidence for librarian action-schema
  contract validation, run payload operation reports, and scope/safety
  guard-based operation rejection prior to apply stage.
- Stage 06 Wave 062 includes deterministic evidence for `xml_attrless`
  response parsing/required-tag validation, bounded repair retries,
  deterministic parse failure codes, and failed-run diagnostics retention.
- Stage 07 includes deterministic evidence for typed `automation_event`
  websocket emissions, stable automation event payload vocabulary,
  stale-cursor deterministic `ack` rejection, mixed note+librarian ordering,
  unknown-event compatibility, and `WS-06` replay acceptance.
- Stage 08 includes deterministic evidence for librarian rule/run control UX,
  manual run launch/list/review API surfaces, operation review decision
  persistence with audit-linked workspace events, and responsive keyboard-first
  shell flows aligned to Stage 08 constraints.
- Stage 09 Wave 090 includes deterministic librarian profile-matrix execution
  and rerun stability evidence for `Librarian-runtime`/`Librarian-small-model`
  verification paths.
- Stage 09 Wave 091 includes deterministic `PERF-01`/`PERF-02`/`PERF-03`
  and `OPS-01`/`OPS-02` evidence archival, including provider-failure/retry
  diagnostics and restart/export/backup lifecycle proof.
- Stage 09 Wave 092 includes full `Release` profile pass evidence,
  synchronized release ledgers, and closure of high-severity blocker class.
- Canonical UX improvements from implementation/user findings are reflected in spec docs
  (including auth presentation split, compact-screen editor focus toggle,
  optional dashboard/workspace surfaces, title-rename propagation, and minimal editor chrome).
- Canonical librarian-agent contracts are documented, including provider modes
  (`openrouter`, `lmstudio`) and attribute-less XML-like protocol requirements.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Policy and governance model | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | docs-first rules and execution policy are present |
| API contract | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `partial` | Stage 01+02 plus Stage 03/04 baseline routes and Stage 08 librarian UX APIs are runtime-reachable and tested (`setup/auth/users/workspaces-members/health/notes/history/rollback/metadata/tags/search/backlinks/views/automation-rules/automation-runs{list,id,review}/automation-rules{id}/launch/admin-export-markdown/admin-export-{id}/admin-backup-sql`); attachments/media and final release breadth remain pending |
| WS protocol | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `partial` | note/workspace subscribe, patch apply, ack replay, idempotent retransmit, stale-cursor deterministic rejection, conflict responses, typed `automation_event` lifecycle emissions, unknown-event replay compatibility, and `WS-06` mixed-order replay checks are runtime-reachable and integration-tested |
| Domain model | [/docs/spec/domain/README.md](/docs/spec/domain/README.md) | `partial` | users/workspaces/membership/projects, notes/events/metadata/tags/backlinks/search baseline, saved views baseline, and automation rule/run lifecycle baseline are implemented |
| UI/UX contract | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | `partial` | root-served responsive shell, command palette model, setup-lock login-only switch, graph/context pane, autosave/title propagation, librarian control panel, and run-review decision surfaces are implemented and integration-tested |
| Librarian AI contract | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | `partial` | Stage 06 provider/payload/parser-retry baseline, Stage 07 stream/replay baseline, and Stage 08 review/apply decision UI+API baseline are runtime-reachable; full operation-kind apply breadth remains pending |
| Runtime implementation | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | `partial` | Actix+SQLx startup sequence, Stage 01 auth/workspace APIs, Stage 02 notes+WS baseline handlers, Stage 03 root-served shell + views APIs, Stage 04 automation rule/run engine baseline, Stage 04 export/backup job handlers, and Stage 06 Wave 060 provider adapter execution path are implemented |
| Testing/performance evidence | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `verified` | Stage 01-08 deterministic suites plus Stage 09 Wave 090 librarian profile matrix, Wave 091 perf/ops evidence (`PERF-01`/`PERF-02`/`PERF-03`, `OPS-01`/`OPS-02`), and Wave 092 full `Release` profile run are passing with synchronized ledgers |

## Conformance Closure Rule

No `spec-only` row may move to `verified` without:

1. deterministic test evidence
2. runtime reachability from documented APIs
3. synchronized reference and TODO updates

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Findings audit: [/docs/log/audits/2026-02-12-implementation-user-findings.md](/docs/log/audits/2026-02-12-implementation-user-findings.md)
- Librarian doc-sync audit: [/docs/log/audits/2026-02-12-librarian-doc-sync.md](/docs/log/audits/2026-02-12-librarian-doc-sync.md)
- Stage 01 audit: [/docs/log/audits/2026-02-13-stage-01-workspace-foundation.md](/docs/log/audits/2026-02-13-stage-01-workspace-foundation.md)
- Stage 02 audit: [/docs/log/audits/2026-02-13-stage-02-collaborative-notes-core.md](/docs/log/audits/2026-02-13-stage-02-collaborative-notes-core.md)
- Stage 03 Wave 030 audit: [/docs/log/audits/2026-02-13-stage-03-wave-030-saved-views.md](/docs/log/audits/2026-02-13-stage-03-wave-030-saved-views.md)
- Stage 03 Wave 031 audit: [/docs/log/audits/2026-02-13-stage-03-wave-031-command-palette.md](/docs/log/audits/2026-02-13-stage-03-wave-031-command-palette.md)
- Stage 03 Wave 032 audit: [/docs/log/audits/2026-02-13-stage-03-wave-032-graph-responsive-shell.md](/docs/log/audits/2026-02-13-stage-03-wave-032-graph-responsive-shell.md)
- Stage 04 Wave 040 audit: [/docs/log/audits/2026-02-13-stage-04-wave-040-automation-rules.md](/docs/log/audits/2026-02-13-stage-04-wave-040-automation-rules.md)
- Stage 04 Wave 041 audit: [/docs/log/audits/2026-02-13-stage-04-wave-041-automation-runs.md](/docs/log/audits/2026-02-13-stage-04-wave-041-automation-runs.md)
- Stage 04 Wave 042 audit: [/docs/log/audits/2026-02-13-stage-04-wave-042-export-backup-jobs.md](/docs/log/audits/2026-02-13-stage-04-wave-042-export-backup-jobs.md)
- Stage 05 Wave 050 audit: [/docs/log/audits/2026-02-13-stage-05-wave-050-reliability-guards.md](/docs/log/audits/2026-02-13-stage-05-wave-050-reliability-guards.md)
- Stage 05 Wave 051 audit: [/docs/log/audits/2026-02-13-stage-05-wave-051-security-hardening.md](/docs/log/audits/2026-02-13-stage-05-wave-051-security-hardening.md)
- Stage 05 Wave 052 audit: [/docs/log/audits/2026-02-13-stage-05-wave-052-perf-ops-gate.md](/docs/log/audits/2026-02-13-stage-05-wave-052-perf-ops-gate.md)
- Stage 06 Wave 060 audit: [/docs/log/audits/2026-02-13-stage-06-wave-060-provider-adapter.md](/docs/log/audits/2026-02-13-stage-06-wave-060-provider-adapter.md)
- Stage 06 Wave 061 audit: [/docs/log/audits/2026-02-13-stage-06-wave-061-librarian-payload-contract.md](/docs/log/audits/2026-02-13-stage-06-wave-061-librarian-payload-contract.md)
- Stage 06 Wave 062 audit: [/docs/log/audits/2026-02-13-stage-06-wave-062-xml-parser-retry.md](/docs/log/audits/2026-02-13-stage-06-wave-062-xml-parser-retry.md)
- Stage 07 audit: [/docs/log/audits/2026-02-13-stage-07-websocket-sync.md](/docs/log/audits/2026-02-13-stage-07-websocket-sync.md)
- Stage 08 audit: [/docs/log/audits/2026-02-13-stage-08-librarian-ux-and-static-delivery.md](/docs/log/audits/2026-02-13-stage-08-librarian-ux-and-static-delivery.md)
- Stage 09 Wave 090 audit: [/docs/log/audits/2026-02-13-stage-09-wave-090-ci-librarian-profiles.md](/docs/log/audits/2026-02-13-stage-09-wave-090-ci-librarian-profiles.md)
- Stage 09 Wave 091 audit: [/docs/log/audits/2026-02-13-stage-09-wave-091-perf-ops-archive.md](/docs/log/audits/2026-02-13-stage-09-wave-091-perf-ops-archive.md)
- Stage 09 Wave 092 audit: [/docs/log/audits/2026-02-13-stage-09-wave-092-release-closure.md](/docs/log/audits/2026-02-13-stage-09-wave-092-release-closure.md)
