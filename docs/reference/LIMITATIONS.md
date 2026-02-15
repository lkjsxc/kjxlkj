# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open gaps between target spec and current repository behavior.

## Baseline (2026-02-15)

- Repository is intentionally docs-only.
- Source/runtime artifacts are removed and must be rebuilt.
- New search/editor/agent contracts are spec-defined but not implemented.

## Open Limitations

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-RUNTIME-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | source tree absent | `M2 missing feature` | high | execute TODO rebuild stages |
| `LIM-SEARCH-01` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | hybrid vector+lexical search not implemented | `M2 missing feature` | high | implement search indexing/query pipeline |
| `LIM-NOTE-TITLE-01` | [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md) | untitled note datetime default missing | `M2 missing feature` | medium | implement create-note default title |
| `LIM-EDITOR-01` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | Obsidian-like markdown editor not implemented | `M2 missing feature` | high | rebuild frontend editor flow |
| `LIM-LAYOUT-01` | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | compact menu threshold change not implemented | `M2 missing feature` | medium | implement responsive breakpoint update |
| `LIM-AGENT-JSON-01` | [/docs/spec/technical/agent-prompt-json.md](/docs/spec/technical/agent-prompt-json.md) | prompt JSON loader not implemented | `M2 missing feature` | high | implement prompt schema loader |
| `LIM-AGENT-KV-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | KV memory carry-over loop not implemented | `M2 missing feature` | high | implement persistent RAM store |
| `LIM-AGENT-YOLO-01` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | YOLO note mutation path not implemented | `M2 missing feature` | high | implement guarded YOLO apply path |

## Closure Rules

A limitation closes only when:

1. behavior is runtime-reachable
2. deterministic tests pass
3. drift and TODO ledgers are synchronized

## Related

- Drift matrix: [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
- TODO program: [/docs/todo/README.md](/docs/todo/README.md)
