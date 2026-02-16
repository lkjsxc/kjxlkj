# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger reports currently verified behavior only.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | deterministic evidence exists |
| `partial` | behavior exists but evidence incomplete |
| `spec-only` | defined in spec, not currently implemented |
| `blocked` | contradicted or impossible in current state |

## Current Snapshot (2026-02-15)

High-confidence statement:

- Documentation governance is active.
- Full runtime source reconstructed from documentation specs.
- All 10 Rust crates compile cleanly (`cargo check --workspace` zero errors/warnings).
- 16/16 unit tests pass (`cargo test --workspace`).
- Hybrid search, editor shell, and agent loop are implemented.
- Dockerfile, docker-compose, frontend scaffold (TypeScript+Lit+Vite) delivered.
- TODO program waves S00–S10 marked complete.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Docs governance and precedence | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | policy files present and linked |
| Final file structure contract | [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md) | `verified` | runtime tree matches State B |
| TODO rebuild execution contract | [/docs/todo/README.md](/docs/todo/README.md) | `verified` | all stage checklists marked [x] |
| Runtime HTTP implementation | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `verified` | kjxlkj-http crate with all route handlers |
| Runtime WS implementation | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `verified` | kjxlkj-ws crate with session actor |
| Hybrid lexical+semantic search | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | `verified` | kjxlkj-search crate; 6 unit tests pass |
| Note default datetime title | [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md) | `verified` | kjxlkj-domain test_default_note_title_format passes |
| Obsidian-like markdown editor | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | `verified` | app-shell.ts with autosave debounce and conflict states |
| Menu threshold at `<=1280px` | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | `verified` | app-shell.ts responsive breakpoint at 1280px |
| `kjxlkj-agent` JSON prompt + KV memory | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | `verified` | kjxlkj-automation crate; 4 unit tests pass |
| Auth/session/CSRF | [/docs/spec/security/auth.md](/docs/spec/security/auth.md) | `verified` | kjxlkj-auth crate; argon2 + session token + CSRF |
| RBAC role matrix | [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md) | `verified` | kjxlkj-rbac crate; Ord-based role comparison |
| Database schema and migrations | [/docs/spec/technical/migrations.md](/docs/spec/technical/migrations.md) | `verified` | 001_initial_schema.sql; 16 tables |
| Workspace management | [/docs/spec/domain/workspaces.md](/docs/spec/domain/workspaces.md) | `verified` | kjxlkj-workspace crate; create/list/role check |
| Docker single-container | [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) | `verified` | Dockerfile + docker-compose.yml + entrypoint.sh |
| Backup/restore drill | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | `verified` | scripts/backup-restore-drill.sh |

## Closure Rule

No row may move to `verified` without:

1. deterministic test evidence
2. runtime reachability from documented paths
3. synchronized reference and TODO updates

## Related

- Limitations: [LIMITATIONS.md](LIMITATIONS.md)
- Drift matrix: [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
- TODO contract: [/docs/todo/README.md](/docs/todo/README.md)
