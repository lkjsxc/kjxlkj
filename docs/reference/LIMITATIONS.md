# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open gaps between target spec and current repository behavior.

## Baseline (2026-02-16)

- Repository intentionally contains no runtime source code.
- Runtime verification is pending until TODO waves are executed.

## Open Limitations

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-RUNTIME-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime topology is not reconstructed | `M2 missing feature` | high | execute waves `S01` through `S10` |
| `LIM-HTTP-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | HTTP API behavior is not runtime-reachable | `M2 missing feature` | high | execute `S06` wave set |
| `LIM-WS-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS sync/replay contract not implemented | `M2 missing feature` | high | execute `S07` wave set |
| `LIM-SEARCH-01` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | hybrid search pipeline not reconstructed | `M2 missing feature` | high | execute `S02/W022` |
| `LIM-UI-01` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | editor UX not reconstructed | `M2 missing feature` | high | execute `S08` wave set |
| `LIM-AGENT-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | agent runtime loop absent | `M2 missing feature` | high | execute `S04` wave set |
| `LIM-TEST-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | acceptance suite cannot run before runtime rebuild | `M4 verification gap` | medium | run per-wave build/test gates |

## Closure Rules

A limitation closes only when:

1. behavior is runtime-reachable
2. deterministic tests pass
3. drift and TODO ledgers are synchronized

## Related

- Drift matrix: [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
- TODO program: [/docs/todo/README.md](/docs/todo/README.md)
