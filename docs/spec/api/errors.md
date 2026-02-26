# Error Contract

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

## Response Envelope

| Field | Type | Meaning |
|---|---|---|
| `code` | string | stable machine code |
| `message` | string | human-readable summary |
| `details` | object/null | structured context |
| `request_id` | string | correlation id |

Envelope is mandatory for HTTP and WebSocket errors.

## HTTP Status Mapping

| Status | Meaning | Canonical codes |
|---|---|---|
| `400` | syntactic input invalid | `BAD_REQUEST`, `INVALID_PATCH`, `WS_BAD_PAYLOAD` |
| `401` | unauthenticated | `AUTH_REQUIRED`, `INVALID_CREDENTIALS`, `SESSION_EXPIRED` |
| `403` | authenticated but forbidden | `ROLE_FORBIDDEN`, `WORKSPACE_FORBIDDEN`, `AGENT_YOLO_POLICY_VIOLATION`, `CSRF_INVALID` |
| `404` | resource missing | `NOTE_NOT_FOUND`, `WORKSPACE_NOT_FOUND`, `RUN_NOT_FOUND` |
| `409` | optimistic or setup conflict | `VERSION_CONFLICT`, `SETUP_ALREADY_COMPLETED` |
| `422` | semantic validation failure | `RULE_INVALID`, `PROMPT_JSON_INVALID`, `PROMPT_SCHEMA_INVALID`, `SEARCH_MODE_INVALID` |
| `429` | rate limited | `RATE_LIMITED` |
| `502` | upstream/provider failure | `LLM_UPSTREAM_ERROR`, `EMBEDDING_PROVIDER_ERROR` |
| `500` | unexpected server fault | `INTERNAL_ERROR`, `AGENT_MEMORY_STORE_ERROR` |

## WebSocket Error Surface

| Code | Meaning | Retryable |
|---|---|---|
| `WS_UNKNOWN_MESSAGE` | message type unsupported | no |
| `WS_BAD_PAYLOAD` | payload schema invalid | no |
| `WS_FORBIDDEN` | stream access denied | no |
| `STALE_CURSOR` | replay cursor no longer available | yes (resubscribe) |
| `WS_INTERNAL_ERROR` | unexpected runtime fault | yes |

## Details Schemas

### `VERSION_CONFLICT`

```json
{
	"expected_version": 7,
	"current_version": 8,
	"resource_id": "note-uuid"
}
```

### `RATE_LIMITED`

```json
{
	"limit": 10,
	"window_seconds": 1,
	"retry_after_seconds": 1
}
```

### `SEARCH_EMBEDDING_DEGRADED`

```json
{
	"mode_requested": "hybrid",
	"mode_effective": "lexical",
	"provider": "ollama",
	"reason": "timeout"
}
```

## Request ID Rules

- `request_id` MUST be present in all error responses.
- Request IDs MUST be unique per request/message handling scope.
- The same request MAY produce multiple logs but MUST retain one request ID.

## Agent and Search-Specific Codes

- `PROMPT_JSON_INVALID`
- `PROMPT_SCHEMA_INVALID`
- `AGENT_MEMORY_STORE_ERROR`
- `AGENT_YOLO_POLICY_VIOLATION`
- `SEARCH_EMBEDDING_DEGRADED`

## Related

- HTTP contract: [http.md](http.md)
- WebSocket contract: [websocket.md](websocket.md)
- Technical agent contract: [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md)

## Related

- HTTP contract: [http.md](http.md)
- Technical agent contract: [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md)
