# Improvement Backlog

Back: [/docs/reference/README.md](/docs/reference/README.md)

Canonical backlog for the next rebuild wave.

## Governance

- This document is the single improvement-point reference.
- Items MUST map into TODO wave checklists.
- Closure requires synchronized updates in conformance/limitations/drift.

## Backlog Matrix

| Backlog ID | Theme | Canonical Doc | Priority | Status |
|---|---|---|---|---|
| `IMP-SEARCH-01` | Hybrid lexical + embedding search | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | high | **closed** — kjxlkj-search crate |
| `IMP-SEARCH-02` | Embedding fallback and reindex jobs | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | high | **closed** — reindex.rs + embedding.rs |
| `IMP-NOTE-01` | Untitled note datetime naming | [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md) | medium | **closed** — unit test passes |
| `IMP-EDITOR-01` | Obsidian-like markdown editor rebuild | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | high | **closed** — app-shell.ts |
| `IMP-LAYOUT-01` | Menu threshold move to `<=1280px` | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | medium | **closed** — app-shell.ts |
| `IMP-AGENT-01` | JSON-defined prompt pipeline | [/docs/spec/technical/agent-prompt-json.md](/docs/spec/technical/agent-prompt-json.md) | high | **closed** — prompt.rs |
| `IMP-AGENT-02` | KV memory carry-over loop store | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | high | **closed** — kv_memory.rs |
| `IMP-AGENT-03` | YOLO mode with guardrails | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | high | **closed** — agent_loop.rs |
| `IMP-AGENT-04` | Disable full conversation transcript retention | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | high | **closed** — KV-only memory |
| `IMP-DOC-01` | Fully linked TODO coverage for all docs | [/docs/todo/README.md](/docs/todo/README.md) | high | **closed** — all waves [x] |
| `IMP-DOC-02` | Final file-structure completion map hardening | [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md) | high | **closed** — runtime tree realized |
| `IMP-DOC-03` | Source-file >200 line audit during rebuild | [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) | medium | **noted** — see docs/logs/large-files.md |

## Related

- TODO contract: [/docs/todo/README.md](/docs/todo/README.md)
- Drift matrix: [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
