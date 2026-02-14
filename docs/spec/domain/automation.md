# Automation Domain

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Rule Model

| Field | Requirement |
|---|---|
| `trigger` | event source that starts evaluation |
| `condition_json` | deterministic predicate configuration |
| `action_json` | deterministic action definition |
| `enabled` | controls active evaluation |

## Librarian Action Contract

The autonomous librarian agent is modeled as an automation action.

`action_json.kind = "librarian_structure"` MUST include:

- provider config (`openrouter` or `lmstudio`)
- prompt-pack config (`manifest_path`, `pack_version`, `hash_algorithm`)
- protocol marker `xml_attrless`
- structuring plan (taxonomy, style profile, operation limits)
- guardrails (`allow_delete`, `strict_mode`, `max_operations`)

Librarian runs MUST parse and emit operations using
[/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md).

## Provider Requirements

- `openrouter` mode MUST support remote OpenAI-compatible chat completion APIs.
- `lmstudio` mode MUST support local OpenAI-compatible server APIs.
- Provider timeout and retry policy MUST be explicit and deterministic.
- Provider selection MUST be auditable in each run record.
- Prompt-pack version and hash MUST be auditable in each run record.

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
- Librarian runs MUST reject malformed protocol output with stable error codes.
- Librarian operation application order MUST be deterministic by operation list order.

## Safety Rules

- Librarian rules MUST default to `allow_delete = false`.
- Operations that touch notes outside rule scope MUST be rejected.
- A run MAY emit `defer` operations instead of unsafe or ambiguous writes.
- Raw prompt/response text SHOULD be retained for replay-safe audit.

## Related

- Events: [events.md](events.md)
- Permissions: [permissions.md](permissions.md)
- Librarian protocol: [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- Operations: [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md)
