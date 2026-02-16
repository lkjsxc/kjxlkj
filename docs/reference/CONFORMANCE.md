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
- Runtime source tree is intentionally absent.
- Docker artifacts are intentionally absent.
- TODO waves are reset to unchecked initial state.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Docs governance and precedence | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | policy tree present and linked |
| Docs-only baseline contract | [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md) | `verified` | root tree matches docs-only state |
| TODO reset workflow | [/docs/todo/README.md](/docs/todo/README.md) | `verified` | all TODO checkboxes reset to `[ ]` |
| Runtime crates and services | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | `spec-only` | source tree removed by design |
| HTTP API behavior | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `spec-only` | runtime absent |
| WebSocket behavior | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `spec-only` | runtime absent |
| Hybrid search behavior | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | `spec-only` | runtime absent |
| Editor and responsive UX | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | `spec-only` | runtime absent |
| `kjxlkj-agent` runtime loop | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | `spec-only` | runtime absent |
| Auth/session/CSRF | [/docs/spec/security/README.md](/docs/spec/security/README.md) | `spec-only` | runtime absent |

## Closure Rule

No row may move to `verified` without:

1. deterministic test evidence
2. runtime reachability from documented paths
3. synchronized reference and TODO updates

## Related

- Limitations: [LIMITATIONS.md](LIMITATIONS.md)
- Drift matrix: [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
- TODO contract: [/docs/todo/README.md](/docs/todo/README.md)
