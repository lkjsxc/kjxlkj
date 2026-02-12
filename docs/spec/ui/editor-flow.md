# Note Editor Flow

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Editing Rules

- Local edits produce patch operations.
- Editor MUST autosave local note changes with a bounded debounce window.
- Editor implementation MUST be built from scratch for Markdown authoring behavior.
- Patch submissions use WS `apply_patch` when connection is active.
- HTTP fallback MAY be used when WS is unavailable.
- Manual save MAY be exposed as an explicit user action.
- Client idempotency keys MUST remain available even when `crypto.randomUUID`
 is unavailable.
- Markdown interactions SHOULD include syntax-aware affordances without breaking
 plain-text flow.

## Presence and Collaboration Rules

- Active editors in the same note SHOULD appear in a presence strip.
- Presence updates MUST NOT alter note version state.
- Remote note changes MUST appear as ordered updates.

## Conflict UX

| Condition | Required UX |
|---|---|
| `patch_rejected` or HTTP `409` | show conflict state and offer refresh/reapply |
| reconnect after disconnect | replay missing events before accepting new patch |

## Related

- WS protocol: [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- Notes domain: [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
