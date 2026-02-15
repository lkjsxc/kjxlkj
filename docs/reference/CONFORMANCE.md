# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger reports currently verified behavior only.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | deterministic evidence exists and no high-severity contradiction is open |
| `partial` | behavior exists but verification is incomplete |
| `blocked` | known user-visible failure or contradiction is open |
| `unverified` | no trustworthy runtime evidence exists |
| `spec-only` | behavior is defined in spec only |

## Current Snapshot (2026-02-15)

High-confidence statement:

- All in Docs governance is active and canonical.
- Stage 00 governance baseline complete.
- Stage 01 crate skeleton complete: 10 crates compile, zero warnings.
- Auth/session/setup-lock, RBAC guards, and workspace membership implemented.
- HTTP routes for health, auth, users, workspaces, projects, notes (CRUD+history).
- WebSocket session actor with heartbeat and connection upgrade.
- All source files respect 200-line layout constraint.
- Runtime not yet tested against live database (no integration test evidence).
- Stage 02 notes lifecycle: create/list/get/patch/delete/rollback handlers compiled.
- Stage 02 metadata & tags: upsert/delete/replace handlers compiled.
- Stage 02 search & backlinks: FTS search and backlink extraction compiled.
- Stage 02 attachments: chunked upload/download/delete with SHA-256 compiled.
- Stage 02 WS realtime: subscribe/unsubscribe/ack/apply-patch with cursor-based replay compiled.
- Stage 02 idempotency: key store/find/cleanup for WS patches compiled.
- Stage 02 snapshots: store/find for event-sourced rollback compiled.
- Stage 03 frontend: React/Vite SPA compiled with auth/editor/layout.
- Stage 04 migration 008: librarian_run_reports, librarian_operations, status alignment.
- Stage 04 automation rules: CRUD with provider validation compiled.
- Stage 04 automation runs: launch/list/detail/review with idempotency compiled.
- Stage 04 export/backup: markdown export and SQL backup job lifecycle compiled.
- Stage 04 librarian reports and operations: audit log store and decision pipeline compiled.
- Stage 05 CSRF middleware: CsrfEnforcer validates x-csrf-token header on mutating requests; returns 403 CSRF_INVALID.
- Stage 05 security headers middleware: SecurityHeaders adds nosniff, DENY, no-store, XSS-protection, referrer-policy.
- Stage 05 regression test stubs: 14 structural stubs for REG-IMP-*, REG-USR-*, REG-UX-003 compiled and passing.
- Stage 05 acceptance test stubs: 24 structural stubs for API/WS/OPS/PERF acceptance pack compiled and passing.
- Stage 06 REST surface parity: views, dashboards, projects CRUD, media notes routes compiled.
- Stage 06 librarian provider adapters: openrouter/lmstudio chat completion with fallback chain compiled.
- Stage 06 JSON prompt pack loader: manifest, stage definitions, template rendering, pack hash compiled.
- Stage 06 xml_attrless parser: tag extraction, attribute rejection, operation validation, confidence bounds compiled.
- Stage 06 safety policy: delete prevention, cross-workspace rejection, scope constraints compiled.
- Stage 06 run pipeline: ingest/plan/propose/validate stages with bounded repair retries compiled.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Policy and governance model | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | policy set defines reconstruction boundaries and documentation precedence |
| All in Docs doctrine | [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md) | `verified` | doctrine defines docs as canonical product value |
| TODO restructure-step workflow | [/docs/todo/waves/README.md](/docs/todo/waves/README.md) | `verified` | stage/wave program reset and linked for deterministic reconstruction |
| Final completion file map | [/docs/spec/architecture/completion-file-map.md](/docs/spec/architecture/completion-file-map.md) | `verified` | canonical completion tree and per-path purpose are explicit |
| Runtime configuration split (`data/config.json` + `.env`) | [/docs/spec/architecture/configuration.md](/docs/spec/architecture/configuration.md) | `verified` | non-secret vs secret boundary is explicit and documented |
| Runtime implementation | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | `partial` | 10 crates compile; startup sequence coded; live DB test pending |
| HTTP/API reachability | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `partial` | all route handlers compiled; acceptance tests pending |
| WS protocol reachability | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `partial` | session actor, subscribe/ack replay, apply-patch compiled; live test pending |
| Notes lifecycle (CRUD + rollback) | [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md) | `partial` | create/list/get/patch/delete/rollback handlers compiled; live test pending |
| Metadata & tags | [/docs/spec/domain/metadata.md](/docs/spec/domain/metadata.md) | `partial` | upsert/delete/replace compiled; validation tests in domain crate |
| Search & backlinks | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | `partial` | FTS and backlink extraction compiled; live test pending |
| Attachments (chunked) | [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) | `partial` | upload/download/delete with SHA-256 compiled; live test pending |
| Idempotency & snapshots | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `partial` | key store and snapshot store compiled; live test pending |
| Typed frontend runtime | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | `partial` | React/Vite SPA compiled; shell/auth/editor/layout implemented; live test pending |
| Editor flow | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | `partial` | synced/draft split, autosave, conflict handling compiled; live test pending |
| Responsive layout | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | `partial` | split-pane at 1024px, compact menu toggle compiled; visual test pending |
| Automation rules and runs | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | `partial` | rule CRUD, run state machine, idempotency, review compiled; live test pending |
| Export and backup jobs | [/docs/spec/domain/export.md](/docs/spec/domain/export.md) | `partial` | markdown/SQL job lifecycle compiled; live test pending |
| Librarian operations audit | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | `partial` | report/operation store and decision pipeline compiled; live test pending |
| CSRF enforcement | [/docs/spec/security/csrf.md](/docs/spec/security/csrf.md) | `partial` | CsrfEnforcer middleware compiled; live request validation pending |
| Transport security headers | [/docs/spec/security/transport.md](/docs/spec/security/transport.md) | `partial` | SecurityHeaders middleware compiled; response header verification pending |
| Auth/session boundaries | [/docs/spec/security/auth.md](/docs/spec/security/auth.md) | `partial` | extract_session guard and role checks compiled; live session lifecycle pending |
| Regression closure | [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) | `partial` | 14 regression stubs compiled and passing; runtime verification pending |
| Acceptance test baseline | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `partial` | 24 acceptance stubs compiled and passing; runtime verification pending |
| Saved views CRUD | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `partial` | list/create/update/delete views compiled; live test pending |
| Dashboard widgets | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `partial` | list/upsert widgets compiled; live test pending |
| Media note creation | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `partial` | POST /notes/media compiled; live test pending |
| Provider adapter contract | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | `partial` | openrouter/lmstudio adapters compiled; live test pending |
| Prompt pack loading | [/docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md) | `partial` | manifest loader and stage validation compiled; live test pending |
| xml_attrless parser | [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md) | `partial` | parser with tag/attribute/confidence validation compiled; live test pending |
| Librarian pipeline | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | `partial` | full pipeline with bounded retry compiled; live test pending |
| Mobile menu top-right and close-on-select behavior | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | `spec-only` | requirement is documented; runtime implementation pending rebuild |
| Create New Note regression test | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `spec-only` | requirement is documented as `E2E-23`; runtime test implementation pending rebuild |
| Release gate | [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md) | `blocked` | runtime and acceptance evidence must be reconstructed |

## Conformance Closure Rule

No row may move to `verified` without all of:

1. deterministic test evidence
2. runtime reachability from documented paths
3. synchronized reference and TODO updates

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)
