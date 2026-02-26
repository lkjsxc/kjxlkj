# WebSocket Contract

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

Endpoint: `GET /ws`

## Handshake Contract

- Upgrade MUST validate active authenticated session.
- Server MUST assign a connection-level `request_id` base for correlation.
- Unauthorized upgrade MUST fail with HTTP `401` error envelope.

## Client Messages

| `type` | Required fields | Purpose |
|---|---|---|
| `subscribe_note` | `request_id`, `note_id`, `cursor` | receive note events from cursor |
| `unsubscribe_note` | `request_id`, `stream_id` | stop note stream |
| `subscribe_workspace` | `request_id`, `workspace_id`, `cursor` | receive workspace/activity stream |
| `ack` | `request_id`, `stream_id`, `event_seq` | confirm cursor |
| `apply_patch` | `request_id`, `note_id`, `base_version`, `patch_ops`, `idempotency_key`, `client_ts` | patch note |
| `ping` | `request_id`, `client_ts` | connectivity check |

## Server Messages

| `type` | Required fields | Purpose |
|---|---|---|
| `subscribed` | `request_id`, `stream_id`, `current_version`, `replay_cursor` | subscription ack |
| `patch_committed` | `request_id`, `note_id`, `version`, `event_seq`, `idempotency_key` | patch accepted |
| `patch_rejected` | `request_id`, `note_id`, `expected_version`, `current_version`, `reason` | version conflict |
| `note_event` | `stream_id`, `note_id`, `event_seq`, `version`, `event_type`, `payload` | note stream event |
| `workspace_event` | `stream_id`, `workspace_id`, `event_seq`, `event_type`, `payload` | workspace activity |
| `automation_event` | `stream_id`, `workspace_id`, `run_id`, `status`, `event_seq`, `payload` | agent updates |
| `pong` | `request_id`, `server_ts` | ping response |
| `error` | `request_id`, `code`, `message`, `details` | protocol error |

## Ordering and Replay

- `event_seq` MUST be monotonic per stream.
- Reconnect MUST replay from acknowledged cursor.
- Duplicate idempotency key MUST replay existing commit identity.
- Stale cursor regression MUST return `STALE_CURSOR` with structured `details`.

## `STALE_CURSOR` Details Schema

```json
{
	"stream_id": "string",
	"requested_cursor": 123,
	"min_available_cursor": 456,
	"recovery": "resubscribe_full"
}
```

## Idempotency Contract

- `apply_patch.idempotency_key` scope is `(workspace_id, note_id, key)`.
- First accepted patch stores commit identity `(note_id, version, event_seq)`.
- Duplicate key MUST return original identity and MUST NOT reapply patch operations.
- Key TTL MUST be finite and documented in runtime config.

## Failure Semantics

- Unknown message type returns `error.code = WS_UNKNOWN_MESSAGE`.
- Malformed payload returns `error.code = WS_BAD_PAYLOAD`.
- Unauthorized stream access returns `error.code = WS_FORBIDDEN`.
- Server-side replay gap returns `STALE_CURSOR` and never fabricates missing events.

## Related

- Error model: [errors.md](errors.md)
- Editor flow: [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- Automation domain: [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- Events: [/docs/spec/domain/events.md](/docs/spec/domain/events.md)
