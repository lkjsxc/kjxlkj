# Error Contract

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

## Response Envelope

Error responses MUST return JSON:

| Field | Type | Meaning |
|---|---|---|
| `code` | string | stable machine error code |
| `message` | string | human-readable summary |
| `details` | object/null | optional structured context |
| `request_id` | string | correlation identifier |

## HTTP Status Mapping

| Status | Meaning | Example codes |
|---|---|---|
| `400` | input validation failure | `BAD_REQUEST`, `INVALID_PATCH` |
| `401` | unauthenticated | `AUTH_REQUIRED`, `INVALID_CREDENTIALS` |
| `403` | forbidden | `CSRF_INVALID` |
| `404` | resource not found | `NOTE_NOT_FOUND`, `ATTACHMENT_NOT_FOUND` |
| `409` | optimistic conflict | `VERSION_CONFLICT` |
| `413` | payload too large | `ATTACHMENT_TOO_LARGE` |
| `429` | rate limited | `RATE_LIMITED` |
| `500` | internal error | `INTERNAL_ERROR` |

## WebSocket Error Rule

WS `error` messages MUST include `code` and `message` and SHOULD include `request_id`.

## Related

- HTTP contract: [http.md](http.md)
- Security model: [/docs/spec/security/README.md](/docs/spec/security/README.md)
