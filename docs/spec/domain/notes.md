# Notes Domain

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Canonical Content Model

- Each note MUST have a stable unique `note_id`.
- Each note MUST have a mutable `title` field independent from `note_id`.
- Note body MUST be Markdown UTF-8 text.
- Wiki-link syntax `[[target]]` MUST be parsed for backlinks.
- Tags are explicit normalized labels, independent of inline hashtags.
- Each note stream MUST include `note_kind`, `workspace_id`, and `access_scope`.
- Notes MAY include `project_id`.

## Default Title Rule

When creating a note without an explicit title:

- the server MUST assign the current timestamp as title at creation time
- the format MUST be `YYYY-MM-DD HH:mm:ss` in server local timezone
- the assigned title MUST be returned in create response and projection reads

This rule applies to user-created notes and agent-created notes.

## Stream Lifecycle

| State | Meaning |
|---|---|
| Active | readable and writable |
| SoftDeleted | hidden from default list/search, recoverable |

Deletion rules:

- Delete MUST be soft-delete at stream level by default.
- Deleted notes MUST be excluded from default list/search.
- Include-deleted mode MAY expose tombstoned streams for recovery.

## Write Rules

- Every write MUST include `base_version`.
- If `base_version != current_version`, mutation MUST be rejected.
- Accepted writes increment version by exactly 1.
- Title-only updates follow identical optimistic version rules.

## Conflict Contract

| Surface | Required behavior |
|---|---|
| HTTP `PATCH /notes/{id}` | return `409` with latest version metadata |
| WS `apply_patch` | return `patch_rejected` with expected/current versions |

## Agent Mutation Rules

- `kjxlkj-agent` MAY create and edit notes in YOLO mode.
- Agent writes MUST still follow optimistic version and permission checks.
- Agent writes MUST be auditable as `actor_type=agent` with agent name.

## Related

- Events: [events.md](events.md)
- Permissions: [permissions.md](permissions.md)
- UI editor flow: [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- API websocket: [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
