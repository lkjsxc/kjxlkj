# WebSocket Contract

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

Endpoint: `GET /ws/v1/notes`

## Client Messages

| `type` | Required fields | Purpose |
|---|---|---|
| `subscribe_note` | `note_id` | start receiving note events |
| `unsubscribe_note` | `note_id` | stop receiving note events |
| `apply_patch` | `note_id`, `base_version`, `patch_ops`, `idempotency_key`, `client_ts` | submit mutation patch |
| `ack` | `note_id`, `event_seq` | confirm receipt cursor |

## Server Messages

| `type` | Required fields | Purpose |
|---|---|---|
| `subscribed` | `note_id`, `current_version`, `replay_cursor` | subscription acknowledgement |
| `patch_committed` | `note_id`, `version`, `event_seq` | caller patch accepted |
| `patch_rejected` | `note_id`, `expected_version`, `current_version`, `reason` | optimistic conflict response |
| `note_event` | `note_id`, `event_seq`, `version`, `event_type`, `payload` | committed stream event |
| `heartbeat` | `server_ts` | keepalive |
| `error` | `code`, `message` | protocol or authorization error |

## Ordering Rules

- Each note stream has a monotonically increasing `event_seq`.
- Server broadcasts MUST follow commit order.
- Duplicate `idempotency_key` for same note MUST be idempotent.
- Conflicting `base_version` MUST return `patch_rejected`.
- Under healthy network and server conditions, patch propagation SHOULD feel near-real-time for active subscribers.
- Reconnect flows MUST support replay from acknowledged cursor without requiring full note reload.

## Related

- Domain events: [/docs/spec/domain/events.md](/docs/spec/domain/events.md)
- Conflict rules: [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
