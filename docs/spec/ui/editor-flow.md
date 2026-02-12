# Note Editor Flow

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Editing Rules

- Local edits produce patch operations.
- Patch submissions use WS `apply_patch` when connection is active.
- HTTP fallback MAY be used when WS is unavailable.

## Conflict UX

| Condition | Required UX |
|---|---|
| `patch_rejected` or HTTP `409` | show conflict state and offer refresh/reapply |
| reconnect after disconnect | replay missing events before accepting new patch |

## Multi-Device Behavior

- Changes from another session MUST appear as ordered remote updates.
- Duplicate retransmit MUST NOT duplicate content.

## Related

- WS protocol: [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- Notes domain: [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
