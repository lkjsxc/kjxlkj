# Agent XML Instruction Protocol

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

Normative instruction protocol for `kjxlkj-agent` execution.

## Protocol Goals

- deterministic parsing on small models
- attribute-free XML only
- explicit instruction tags for state, memory, and note operations

## Wire Rules

- Payload MUST be UTF-8 text.
- XML attributes are forbidden.
- Output MUST contain only instruction elements, no prose.
- Unknown tags MUST be ignored or rejected by strict-mode policy.

## Allowed Top-Level Tags

- `<state_add>`
- `<state_delete>`
- `<ram_add>`
- `<ram_delete>`
- `<record_add>`
- `<record_issue>`
- `<record_update>`
- `<record_search>`

## Required Child Elements

| Tag | Required children |
|---|---|
| `state_add` | `state` |
| `state_delete` | `state` |
| `ram_add` | `key`, `value` |
| `ram_delete` | `key` |
| `record_add` | `keywords`, `value` |
| `record_issue` | `key`, `value`, `metadata` |
| `record_update` | `key`, `value` |
| `record_search` | `query` or `ids` |

## Execution Rules

- Instructions execute sequentially in document order.
- RAM writes become visible in the next loop iteration.
- Record operations mutate persistent note storage.
- Agent MUST use RAM KV memory for loop-to-loop continuity.

## Related

- Agent technical contract: [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md)
- Automation domain: [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
