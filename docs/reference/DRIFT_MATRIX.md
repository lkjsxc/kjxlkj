# Drift Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Requirement-level mismatch tracking for reconstruction.

## Mismatch Classes

| Class | Meaning |
|---|---|
| `M1 correctness` | runtime behavior violates spec |
| `M2 missing feature` | specified capability absent |
| `M3 undocumented behavior` | implementation exists without spec |
| `M4 verification gap` | behavior exists but evidence insufficient |
| `M5 stale docs` | docs contradict stronger evidence |

## Matrix

| Req ID | Canonical Document | Requirement | Observed Status | Mismatch Class | Action |
|---|---|---|---|---|---|
| `R-DOC-01` | [/docs/README.md](/docs/README.md) | docs are canonical contract | aligned | closed | maintain |
| `R-TODO-01` | [/docs/todo/README.md](/docs/todo/README.md) | TODO drives deterministic rebuild with direct links | aligned | closed | maintain |
| `R-STRUCT-01` | [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md) | docs-only and runtime target structures are explicit | aligned | closed | maintain |
| `R-SEARCH-NEW-01` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | hybrid lexical+semantic search | spec-only | `M2` | implement in `kjxlkj-search` |
| `R-NOTE-TITLE-01` | [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md) | untitled note defaults to datetime title | spec-only | `M2` | implement create-path defaulting |
| `R-EDITOR-OBS-01` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | Obsidian-like markdown editor | spec-only | `M2` | rebuild frontend editor |
| `R-MENU-THRESH-01` | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | compact menu threshold at `<=1280px` | spec-only | `M2` | rebuild responsive layout |
| `R-AGENT-JSON-01` | [/docs/spec/technical/agent-prompt-json.md](/docs/spec/technical/agent-prompt-json.md) | full prompt defined via JSON | spec-only | `M2` | implement prompt loader |
| `R-AGENT-KV-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | KV memory carry-over, no full convo logs | spec-only | `M2` | implement loop memory policy |
| `R-AGENT-YOLO-01` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | YOLO mode direct note mutation | spec-only | `M2` | implement guarded YOLO execution |
| `R-RUNTIME-ABSENT-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime reachable from source tree | absent by intent | `M2` | rebuild source from TODO waves |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 8 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 0 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [CONFORMANCE.md](CONFORMANCE.md)
- Limitations: [LIMITATIONS.md](LIMITATIONS.md)
