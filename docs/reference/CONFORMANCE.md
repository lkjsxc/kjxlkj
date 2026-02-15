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
- Runtime source is intentionally absent (docs-only baseline).
- Search redesign, editor redesign, and agent redesign are now specified.
- TODO program has been reset for full rebuild execution.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Docs governance and precedence | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | policy files present and linked |
| Final file structure contract | [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md) | `verified` | docs-only and runtime-target trees defined |
| TODO rebuild execution contract | [/docs/todo/README.md](/docs/todo/README.md) | `verified` | ordered stage checklists with links present |
| Runtime HTTP implementation | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `spec-only` | source tree intentionally removed |
| Runtime WS implementation | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `spec-only` | source tree intentionally removed |
| Hybrid lexical+semantic search | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | `spec-only` | redesign defined; runtime pending |
| Note default datetime title | [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md) | `spec-only` | behavior defined; runtime pending |
| Obsidian-like markdown editor | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | `spec-only` | UX contract defined; runtime pending |
| Menu threshold at `<=1280px` | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | `spec-only` | UX contract defined; runtime pending |
| `kjxlkj-agent` JSON prompt + KV memory | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | `spec-only` | technical contract defined; runtime pending |

## Closure Rule

No row may move to `verified` without:

1. deterministic test evidence
2. runtime reachability from documented paths
3. synchronized reference and TODO updates

## Related

- Limitations: [LIMITATIONS.md](LIMITATIONS.md)
- Drift matrix: [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
- TODO contract: [/docs/todo/README.md](/docs/todo/README.md)
