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
| Typed frontend runtime | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | `spec-only` | frontend source intentionally absent |
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
