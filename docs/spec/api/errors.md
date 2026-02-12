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
| `403` | forbidden | `CSRF_INVALID`, `ROLE_FORBIDDEN` |
| `404` | resource not found | `NOTE_NOT_FOUND`, `WORKSPACE_NOT_FOUND` |
| `409` | optimistic conflict | `VERSION_CONFLICT`, `MEMBERSHIP_CONFLICT` |
| `413` | payload too large | `ATTACHMENT_TOO_LARGE` |
| `422` | semantically invalid state | `RULE_INVALID`, `PROJECT_SCOPE_INVALID` |
| `429` | rate limited | `RATE_LIMITED` |
| `502` | upstream provider failure | `LLM_UPSTREAM_ERROR`, `LLM_PROVIDER_UNREACHABLE` |
| `500` | internal error | `INTERNAL_ERROR` |

## WebSocket Error Rule

WS `error` messages MUST include `code`, `message`, and `request_id`.

Librarian-specific machine codes SHOULD include:

- `LIBRARIAN_PROTOCOL_INVALID`
- `LIBRARIAN_PARSE_FAILED`
- `LIBRARIAN_OPERATION_REJECTED`

## Related

- HTTP contract: [http.md](http.md)
- Security model: [/docs/spec/security/README.md](/docs/spec/security/README.md)
