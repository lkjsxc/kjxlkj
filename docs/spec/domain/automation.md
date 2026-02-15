# Automation Domain

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Rule Model

| Field | Requirement |
|---|---|
| `trigger` | event source that starts evaluation |
| `condition_json` | deterministic predicate configuration |
| `action_json` | deterministic action definition |
| `enabled` | controls active evaluation |

## `kjxlkj-agent` Contract

The canonical autonomous agent name MUST be `kjxlkj-agent`.

`action_json.kind = "kjxlkj_agent"` MUST support:

- JSON-defined prompt pack path
- provider config (`openrouter` or `lmstudio`)
- operation mode (`reviewed` or `yolo`)
- memory policy (`kv_store` persisted between loops)
- bounded execution and retry controls

In `yolo` mode, the agent MAY create/update/delete notes directly.

## Memory and Logging Rules

- Agent memory MUST use a mutable key-value store persisted across loops.
- Memory keys MUST be freely add/update/delete capable by the agent.
- Full conversation transcript logging MUST be disabled by default.
- Run audit logs MUST keep operation summaries, not full raw chat history.

## Prompt Rules

- Prompt content MUST be fully loaded from JSON files.
- Prompt configuration MUST be hot-reloadable or reload-on-start.
- Prompt schema validation MUST fail fast on invalid JSON.

## Run Lifecycle

| State | Meaning |
|---|---|
| Queued | rule qualified and awaits execution |
| Running | action is executing |
| Succeeded | action completed and effects committed |
| Failed | action failed and error is recorded |

## Determinism and Safety

- Runs MUST be idempotent per `(rule_id, triggering_event_id)`.
- Agent side effects on notes MUST obey optimistic concurrency.
- Cross-workspace writes MUST be rejected.
- Every applied operation MUST be auditable with stable error codes.

## Related

- Notes: [notes.md](notes.md)
- API protocol: [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- Agent technical contract: [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md)
