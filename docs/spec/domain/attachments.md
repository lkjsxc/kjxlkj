# Attachments Contract

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Storage Model

Current baseline stores attachment payloads as typed records in application
state.

| Parameter | Value |
|---|---|
| Per-file max | 500 MiB |
| Payload format | UTF-8 string field (`content`) |
| Record shape | `id`, `note_id`, `filename`, `mime`, `size_bytes`, `content` |

## Upload Rules

- Upload MUST validate note existence and caller authorization.
- Server MUST reject files larger than max with `413`.
- Completed upload MUST return `201` with attachment record payload.
- Upload and download access MUST enforce session + role checks.

## Download Rules

- Download (`GET /attachments/{id}`) MUST return deterministic attachment record payload.

## Deletion Rules

- Deleting attachment MUST remove the attachment record and return `204`.

## Related

- API endpoints: [/docs/spec/api/http.md](/docs/spec/api/http.md)
- Performance constraints: [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md)
