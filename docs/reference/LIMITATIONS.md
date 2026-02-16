# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open gaps between target spec and current repository behavior.

## Baseline (2026-02-16)

- Runtime source tree is reconstructed.
- Core workspace and frontend build gates pass.
- Remaining gaps are verification depth (integration + browser E2E).

## Closed Limitations

| ID | Requirement Link | Gap | Resolution |
|---|---|---|---|
| `LIM-RUNTIME-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime topology not reconstructed | **closed** — `src/`, `Cargo.toml`, `Cargo.lock`, and `scripts/` restored |
| `LIM-HTTP-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | HTTP API unreachable | **closed** — `kjxlkj-http` crate restored and workspace build/test pass |
| `LIM-WS-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS sync/replay contract not implemented | **closed** — `kjxlkj-ws` crate restored and compiled |
| `LIM-SEARCH-01` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | hybrid search pipeline missing | **closed** — `kjxlkj-search` crate restored with passing unit tests |
| `LIM-UI-01` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | editor UX runtime missing | **closed** — frontend app restored, strict check/build pass |
| `LIM-AGENT-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | agent runtime loop absent | **closed** — `kjxlkj-automation` restored with passing parser tests |

## Open Limitations

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-TEST-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | full acceptance pack IDs are not yet automated/executed end-to-end | `M4 verification gap` | medium | add acceptance harness and archived run outputs |
| `LIM-INTDB-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | DB-backed integration profile is not yet reproducible in-repo | `M4 verification gap` | medium | add ephemeral PostgreSQL integration harness |
| `LIM-E2E-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | browser E2E IDs are not yet implemented in test suite | `M4 verification gap` | medium | add Playwright/Vitest E2E coverage for `E2E-*` IDs |

## Closure Rules

A limitation closes only when:

1. behavior is runtime-reachable
2. deterministic tests pass
3. drift and TODO ledgers are synchronized

## Related

- Drift matrix: [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
- TODO program: [/docs/todo/README.md](/docs/todo/README.md)
