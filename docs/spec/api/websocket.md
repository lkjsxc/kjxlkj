# WebSocket Contract

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

Endpoint: `GET /ws`

## Client Messages

| `type` | Required fields | Purpose |
|---|---|---|
| `subscribe_note` | `note_id` | start receiving note events |
| `unsubscribe_note` | `note_id` | stop receiving note events |
| `subscribe_workspace` | `workspace_id` | receive workspace-level activity and presence |
| `unsubscribe_workspace` | `workspace_id` | stop workspace-level stream |
| `apply_patch` | `note_id`, `base_version`, `patch_ops`, `idempotency_key`, `client_ts` | submit mutation patch |
| `ack` | `stream_id`, `event_seq` | confirm receipt cursor |
| `presence_ping` | `workspace_id`, `note_id`, `cursor` | publish active-presence heartbeat |

## Server Messages

| `type` | Required fields | Purpose |
|---|---|---|
| `subscribed` | `stream_id`, `current_version`, `replay_cursor` | subscription acknowledgement |
| `patch_committed` | `note_id`, `version`, `event_seq`, `idempotency_key` | caller patch accepted |
| `patch_rejected` | `note_id`, `expected_version`, `current_version`, `reason` | optimistic conflict response |
| `note_event` | `note_id`, `event_seq`, `version`, `event_type`, `payload` | committed note stream event |
| `workspace_event` | `workspace_id`, `event_seq`, `event_type`, `payload` | project/view/member/dashboard activity |
| `presence_event` | `workspace_id`, `note_id`, `user_id`, `state`, `server_ts` | collaborator presence state |
| `automation_event` | `workspace_id`, `run_id`, `status`, `payload` | automation run updates |
| `heartbeat` | `server_ts` | keepalive |
| `error` | `code`, `message`, `request_id` | protocol or authorization error |

## Ordering and Replay Rules

- Each stream (`note:{id}` or `workspace:{id}`) has monotonically increasing `event_seq`.
- Server broadcasts MUST follow commit order for each stream.
- Duplicate `idempotency_key` for the same note MUST replay-safe-return existing commit identity.
- Conflicting `base_version` MUST return `patch_rejected`.
- Reconnect flows MUST replay from acknowledged cursor without full-note reload.
- Presence events MAY be lossy, but note and workspace events MUST be lossless.

## Related

- Domain events: [/docs/spec/domain/events.md](/docs/spec/domain/events.md)
- Permission model: [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md)
