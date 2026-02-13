# Note Editor Flow

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## State Model

- Editor MUST maintain separate state for:
  - synced server snapshot (last accepted version)
  - local draft buffer (user input not yet confirmed)
- Patch generation MUST use synced snapshot + draft diff, never mutable UI text
  as implicit source of truth (`UX-EDIT-01`).

## Editing Rules

- Local edits produce deterministic patch operations.
- Editor MUST autosave within a bounded debounce window and show explicit status
  transitions (`UX-EDIT-02`).
- Markdown-native authoring affordances SHOULD exist without breaking plain text.
- WS `apply_patch` is primary mutation path when connected.
- HTTP fallback MAY be used when WS is unavailable.
- Manual save is optional and SHOULD remain hidden by default (`UX-EDIT-05`).
- Client idempotency keys MUST work even when `crypto.randomUUID` is unavailable
  (`UX-EDIT-03`).
- Title changes MUST propagate to list/navigation surfaces in the same cycle
  (`UX-EDIT-04`).
- Default editor chrome SHOULD omit inline version/save/delete controls
  (`UX-EDIT-05`).

## Replay and Idempotency Rules

- Duplicate `idempotency_key` for same note and base commit MUST replay existing
  commit identity (`UX-EDIT-07`).
- Reconnect MUST replay from acknowledged cursor before accepting new local patch.
- Stale cursor submits MUST produce deterministic conflict/error payloads and
  explicit user actions (`UX-EDIT-06`).

## Librarian Interaction Rules

- If librarian review proposes note rewrites, UI MUST present deterministic diff
  and per-operation accept/reject controls (`UX-LIB-01`).
- Applying librarian changes MUST preserve unresolved local draft state and MUST
  not silently discard user edits (`UX-LIB-02`).

## Conflict UX

| Condition | Required UX |
|---|---|
| `patch_rejected` or HTTP `409` | show conflict state and offer `reload latest`, `reapply draft`, `copy draft` |
| reconnect after disconnect | replay missing events before new patch submit |
| idempotent retransmit | surface stable commit identity rather than duplicate success state |

## Findings Coverage

| Finding IDs | Required Outcome |
|---|---|
| `IMP-001` | explicit synced/draft split |
| `IMP-002`, `IMP-004` | replay-safe idempotency and deterministic reconnect cursor handling |
| `USR-002`, `USR-003` | compatible key generation fallback and autosave-first markdown UX |
| `USR-007`, `USR-008` | same-cycle title propagation and minimal default chrome |

## Related

- UX requirements: [reconstruction-ux-requirements.md](reconstruction-ux-requirements.md)
- WS protocol: [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- Notes domain: [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
- Findings map: [findings-traceability.md](findings-traceability.md)
