# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open gaps between target spec and current repository behavior.

## Baseline (2026-02-17)

- Repository contains reconstructed runtime (10 Rust crates + TS frontend).
- All 41 unit tests pass. No DB-backed integration tests yet.
- HTTP and WebSocket handlers are stub implementations pending DB wiring.

## Open Limitations

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-RUNTIME-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime stubs not wired to DB | `M2 missing feature` | high | wire PostgreSQL repositories |
| `LIM-HTTP-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | HTTP handlers are stub responses | `M2 missing feature` | high | wire DB in handlers |
| `LIM-WS-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS replay/cursor not yet implemented | `M2 missing feature` | high | implement replay engine |
| `LIM-SEARCH-01` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | search not wired to DB/embedding | `M2 missing feature` | high | wire search pipeline |
| `LIM-UI-01` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | editor is minimal HTML (no rich editor) | `M2 missing feature` | high | integrate CodeMirror/ProseMirror |
| `LIM-AGENT-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | agent not wired to LLM provider | `M2 missing feature` | high | wire OpenRouter/LMStudio adapter |
| `LIM-TEST-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | no DB-backed integration tests | `M4 verification gap` | medium | add ephemeral DB harness |

## Closure Rules

A limitation closes only when:

1. behavior is runtime-reachable
2. deterministic tests pass
3. drift and TODO ledgers are synchronized

## Related

- Drift matrix: [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
- TODO program: [/docs/todo/README.md](/docs/todo/README.md)
