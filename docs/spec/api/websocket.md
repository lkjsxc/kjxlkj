# WebSocket Contract

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

Endpoint: `GET /ws`

## Client Messages

| `type` | Required fields | Purpose |
|---|---|---|
| `subscribe_note` | `note_id` | receive note events |
| `unsubscribe_note` | `note_id` | stop note stream |
| `subscribe_workspace` | `workspace_id` | receive workspace/activity stream |
| `ack` | `stream_id`, `event_seq` | confirm cursor |
| `apply_patch` | `note_id`, `base_version`, `patch_ops`, `idempotency_key`, `client_ts` | patch note |

## Server Messages

| `type` | Required fields | Purpose |
|---|---|---|
| `subscribed` | `stream_id`, `current_version`, `replay_cursor` | subscription ack |
| `patch_committed` | `note_id`, `version`, `event_seq`, `idempotency_key` | patch accepted |
| `patch_rejected` | `note_id`, `expected_version`, `current_version`, `reason` | version conflict |
| `note_event` | `note_id`, `event_seq`, `version`, `event_type`, `payload` | note stream event |
| `workspace_event` | `workspace_id`, `event_seq`, `event_type`, `payload` | workspace activity |
| `automation_event` | `workspace_id`, `run_id`, `status`, `event_seq`, `payload` | `kjxlkj-agent`/automation updates |
| `error` | `code`, `message`, `details`, `request_id` | protocol error |

## Ordering and Replay

- `event_seq` MUST be monotonic per stream.
- Reconnect MUST replay from acknowledged cursor.
- Duplicate idempotency key MUST replay existing commit identity.
- Stale cursor regression MUST return `STALE_CURSOR` with structured `details`.

## Related

- Error model: [errors.md](errors.md)
- Editor flow: [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- Automation domain: [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
