# Findings Traceability

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

Canonical mapping from findings to normative requirements.

## Matrix

| Finding | Category | Normative Requirement | Canonical Spec Coverage | Acceptance IDs |
|---|---|---|---|---|
| `IMP-001` | editor correctness | synced/draft state split is mandatory | [editor-flow.md](editor-flow.md) | `E2E-06`, `E2E-17` |
| `IMP-002` | replay safety | duplicate idempotency keys replay same commit identity | [editor-flow.md](editor-flow.md), [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `WS-04` |
| `IMP-004` | reconnect semantics | ack-cursor replay is deterministic | [editor-flow.md](editor-flow.md), [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `WS-05` |
| `USR-003` | editing UX | markdown-first autosave editor confidence path | [editor-flow.md](editor-flow.md) | `E2E-06`, `E2E-17` |
| `USR-005` | responsive UX | compact menu threshold and close-on-select behavior | [layout-and-interaction.md](layout-and-interaction.md) | `E2E-12`, `E2E-19`, `E2E-25` |
| `USR-007` | consistency | title rename propagates same cycle | [web-app.md](web-app.md), [editor-flow.md](editor-flow.md) | `API-NOTE-02`, `E2E-23` |
| `USR-008` | focus | default editor chrome remains minimal | [web-app.md](web-app.md), [editor-flow.md](editor-flow.md) | `E2E-24` |
| `USR-009` | note creation | untitled note defaults to datetime title | [web-app.md](web-app.md), [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md) | `API-NOTE-01`, `E2E-23` |
| `USR-010` | search quality | hybrid lexical+semantic retrieval replaces broken search-only path | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | `API-SEARCH-01`, `API-SEARCH-02` |
| `USR-011` | agent control | prompt fully defined in JSON for `kjxlkj-agent` with KV memory carry-over | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | `API-AUTO-03`, `API-AUTO-04`, `AGENT-01`, `AGENT-02` |

## Closure Rule

A finding is closed only when spec, tests, and ledgers all align.
