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

## Current Snapshot (2026-02-16)

High-confidence statement:

- Docs governance and precedence are active.
- Runtime source tree is reconstructed and reachable.
- Docker artifacts remain intentionally absent.
- Workspace build/test and frontend type/build gates pass.
- TODO waves are marked complete and synchronized with reference ledgers.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Docs governance and precedence | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | policy tree present and linked |
| Final file structure contract | [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md) | `verified` | root tree matches State B runtime target |
| TODO execution workflow | [/docs/todo/README.md](/docs/todo/README.md) | `verified` | staged and wave TODO checklists marked `[x]` |
| Runtime crates and services | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | `verified` | `src/crates/**` + `cargo build --workspace` pass |
| HTTP API behavior | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `partial` | `kjxlkj-http` routes compile; acceptance IDs pending |
| WebSocket behavior | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `partial` | `kjxlkj-ws` compiles; replay acceptance IDs pending |
| Hybrid search behavior | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | `verified` | `kjxlkj-search` crate + unit tests pass |
| Editor and responsive UX | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | `partial` | frontend `check`/`build` pass; browser E2E pending |
| `kjxlkj-agent` runtime loop | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | `verified` | `kjxlkj-automation` crate + parser tests pass |
| Auth/session/CSRF | [/docs/spec/security/README.md](/docs/spec/security/README.md) | `verified` | `kjxlkj-auth` crate present + unit tests pass |
| Frontend type safety | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | `verified` | `npm --prefix src/frontend/app run check` pass |

## Closure Rule

No row may move to `verified` without:

1. deterministic test evidence
2. runtime reachability from documented paths
3. synchronized reference and TODO updates

## Related

- Limitations: [LIMITATIONS.md](LIMITATIONS.md)
- Drift matrix: [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
- TODO contract: [/docs/todo/README.md](/docs/todo/README.md)
