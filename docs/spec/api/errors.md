# Error Contract

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

## Response Envelope

| Field | Type | Meaning |
|---|---|---|
| `code` | string | stable machine code |
| `message` | string | human-readable summary |
| `details` | object/null | structured context |
| `request_id` | string | correlation id |

## HTTP Status Mapping

| Status | Meaning | Example codes |
|---|---|---|
| `400` | input invalid | `BAD_REQUEST`, `INVALID_PATCH` |
| `401` | unauthenticated | `AUTH_REQUIRED`, `INVALID_CREDENTIALS` |
| `403` | forbidden | `ROLE_FORBIDDEN`, `WORKSPACE_FORBIDDEN` |
| `404` | not found | `NOTE_NOT_FOUND`, `WORKSPACE_NOT_FOUND` |
| `409` | optimistic conflict | `VERSION_CONFLICT` |
| `422` | semantically invalid | `RULE_INVALID`, `PROMPT_JSON_INVALID`, `SEARCH_MODE_INVALID` |
| `429` | rate limit | `RATE_LIMITED` |
| `502` | upstream/provider failure | `LLM_UPSTREAM_ERROR`, `EMBEDDING_PROVIDER_ERROR` |
| `500` | internal failure | `INTERNAL_ERROR` |

## Agent and Search-Specific Codes

- `PROMPT_JSON_INVALID`
- `PROMPT_SCHEMA_INVALID`
- `AGENT_MEMORY_STORE_ERROR`
- `AGENT_YOLO_POLICY_VIOLATION`
- `SEARCH_EMBEDDING_DEGRADED`

## Related

- HTTP contract: [http.md](http.md)
- Technical agent contract: [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md)
