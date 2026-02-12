# Automation Domain

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Rule Model

| Field | Requirement |
|---|---|
| `trigger` | event source that starts evaluation |
| `condition_json` | deterministic predicate configuration |
| `action_json` | deterministic action definition |
| `enabled` | controls active evaluation |

## Run Lifecycle

| State | Meaning |
|---|---|
| Queued | rule qualified and awaits execution |
| Running | action is executing |
| Succeeded | action completed and effects committed |
| Failed | action failed and error is recorded |

## Determinism Rules

- Rule evaluation MUST be deterministic for identical input events.
- Runs MUST be idempotent per `(rule_id, triggering_event_id)`.
- Failed runs MUST preserve error detail for audit and debugging.
- Automation side-effects on notes MUST use the same optimistic concurrency rules
 as user writes.

## Related

- Events: [events.md](events.md)
- Permissions: [permissions.md](permissions.md)
- Operations: [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md)
