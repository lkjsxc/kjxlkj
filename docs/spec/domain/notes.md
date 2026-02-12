# Notes Domain

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Canonical Content Model

- Note body MUST be Markdown UTF-8 text.
- Wiki-link syntax `[[target]]` MUST be parsed for backlinks.
- Tags are explicit normalized labels, independent of inline markdown hashtags.
- Note title MUST be mutable and independently editable from body.
- Each note stream MUST include `note_kind` from [note-types.md](note-types.md).
- Each note stream MUST include `workspace_id`.
- Notes MAY include `project_id` and MUST set `access_scope`.

## Stream Lifecycle

| State | Meaning |
|---|---|
| Active | readable and writable |
| SoftDeleted | hidden from default list, recoverable via history |

Deletion rules:

- Users with delete permission MUST be able to delete notes.
- Deletion MUST be soft-delete at stream level by default.
- Deleted notes MUST be excluded from default search/list queries unless
 explicit include-deleted mode is requested.

## Write Rules

- Every write MUST include `base_version`.
- If `base_version != current_version`, server MUST reject with conflict.
- Accepted write increments note version by exactly 1.
- Title-only updates MUST follow the same optimistic version rules.

## Conflict Contract

| Surface | Required behavior |
|---|---|
| HTTP `PATCH /notes/{id}` | return `409` with latest version |
| WS `apply_patch` | return `patch_rejected` with expected/current versions |

## Related

- Events: [events.md](events.md)
- Permissions: [permissions.md](permissions.md)
- API websocket: [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
