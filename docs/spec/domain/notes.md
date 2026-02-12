# Notes Domain

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Canonical Content Model

- Note body MUST be Markdown UTF-8 text.
- Wiki-link syntax `[[target]]` MUST be parsed for backlinks.
- Tags are explicit normalized labels, independent of inline markdown hashtags.

## Stream Lifecycle

| State | Meaning |
|---|---|
| Active | readable and writable |
| SoftDeleted | hidden from default list, recoverable via history |

## Write Rules

- Every write MUST include `base_version`.
- If `base_version != current_version`, server MUST reject with conflict.
- Accepted write increments note version by exactly 1.

## Conflict Contract

| Surface | Required behavior |
|---|---|
| HTTP `PATCH /notes/{id}` | return `409` with latest version |
| WS `apply_patch` | return `patch_rejected` with expected/current versions |

## Related

- Events: [events.md](events.md)
- API websocket: [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
