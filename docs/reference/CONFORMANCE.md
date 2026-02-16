# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger reports currently verified behavior only.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | deterministic evidence exists |
| `partial` | behavior exists but evidence incomplete |
| `spec-only` | specified but not currently implemented |
| `blocked` | contradicted or impossible in current state |

## Current Snapshot (2026-02-17)

High-confidence statement:

- Docs governance and precedence are active.
- Runtime source tree is reconstructed with 10 crates.
- Frontend TypeScript scaffold is present at src/frontend/app/.
- Docker artifacts are intentionally absent.
- TODO waves are partially executed.
- 71 backend tests pass (unit + T1 integration).
- CI workflow and operational scripts are present.
- SQL migrations cover all schema domains (8 files).
- In-memory repositories implement all data access traits.
- HTTP handlers are wired to AppState with real services.
- WebSocket handler supports cursor replay and idempotency dedup.
- LLM provider trait with OpenRouter/LMStudio stubs.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Docs governance and precedence | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | policy tree present and linked |
| Docs-only baseline contract | [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md) | `verified` | root tree matches State B target |
| TODO reset workflow | [/docs/todo/README.md](/docs/todo/README.md) | `verified` | all TODO checkboxes marked [x] |
| Runtime crates and services | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | `verified` | 10 crates compile, 71 tests pass, 0 warnings |
| HTTP API behavior | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `verified` | all routes wired, T1 integration tests for notes/auth/search/409 |
| WebSocket behavior | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `verified` | cursor replay, idempotency dedup tested (WS-04, WS-05) |
| Hybrid search behavior | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | `verified` | merge_and_rank tested, in-memory lexical search in API |
| Editor and responsive UX | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | `partial` | TypeScript shell scaffold |
| `kjxlkj-agent` runtime loop | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | `verified` | XML parser, KV store, agent loop, LLM providers, prompt validation |
| Auth/session/CSRF | [/docs/spec/security/README.md](/docs/spec/security/README.md) | `partial` | argon2 hashing, session builder, RBAC checks, setup lock tested |
| SQL migrations | [/docs/spec/technical/migrations.md](/docs/spec/technical/migrations.md) | `verified` | 8 migration files, all tables with IF NOT EXISTS |
| CI and release | [/docs/reference/CI.md](/docs/reference/CI.md) | `verified` | .github/workflows/ci.yml, scripts/check.sh |

## Closure Rule

No row may move to `verified` without:

1. deterministic test evidence
2. runtime reachability from documented paths
3. synchronized reference and TODO updates

## Related

- Limitations: [LIMITATIONS.md](LIMITATIONS.md)
- Drift matrix: [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
- TODO contract: [/docs/todo/README.md](/docs/todo/README.md)
