# `kjxlkj-agent` Technical Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

## Canonical Identity

- Agent name MUST be `kjxlkj-agent`.
- Agent configuration MUST be loaded from JSON.
- Prompt text MUST be fully defined in JSON files.

## Source Reference Policy

Agent behavior requirements are derived from the agent sections in
`tmp/lkjspapp-main` and normalized into this repository's canonical docs.

## Loop Model

- Agent runs an iterative loop.
- Each loop carries forward mutable KV memory (`ram`).
- Agent state is explicit and can be added/removed (`state_add`, `state_delete`).
- Records are persistent note storage and require explicit query/update actions.

## Memory Model

- Conversation transcript logging MUST NOT be retained in full by default.
- Agent memory persistence MUST rely on a key-value store carried across loops.
- Agent MUST be able to freely create/update/delete KV entries.
- RAM and record operations MUST remain auditable via operation summaries.

## Prompt JSON Contract

Prompt files MUST support segment-based composition with conditions.

Required top-level fields:

- `agent_name`
- `version`
- `segments[]`

Each segment MUST include:

- `condition`
- `prompt`

Supported `condition` values MUST include:

- `default`
- named states (for example `planning`, `executing`, `evaluating`)

## YOLO Mode Contract

- `mode = "yolo"` allows direct note mutations without review queue.
- Even in YOLO mode, permission scope and optimistic version checks apply.
- Cross-workspace writes MUST be rejected.

## Instruction Contract

Agent output MUST use XML instruction tags defined in:

- [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)

Key tags:

- state: `state_add`, `state_delete`
- kv memory: `ram_add`, `ram_delete`
- note/record IO: `record_add`, `record_issue`, `record_update`, `record_search`

## Determinism and Safety

- Loop retries MUST be bounded and deterministic.
- Parse failures MUST emit stable machine codes.
- Prompt schema errors MUST fail fast at startup.
- Agent run metadata MUST include prompt hash and parser version.

## Related

- Domain automation: [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- XML protocol: [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- Testing: [testing.md](testing.md)
