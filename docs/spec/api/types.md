# External Types

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

## Core Resource Types

| Type | Required fields |
|---|---|
| `NoteStream` | `id`, `title`, `note_kind`, `created_at`, `updated_at`, `current_version`, `deleted_at` |
| `NoteProjection` | `note_id`, `title`, `note_kind`, `version`, `markdown`, `rendered_html`, `metadata_json`, `search_vector` |
| `NoteEvent` | `event_id`, `note_id`, `seq`, `event_type`, `payload_json`, `actor_id`, `created_at` |
| `Attachment` | `id`, `note_id`, `filename`, `mime`, `size_bytes`, `sha256`, `chunk_count` |
| `AttachmentChunk` | `attachment_id`, `chunk_index`, `bytes` |

## Note Kind Type

`note_kind` MUST be one of:

- `markdown`
- `settings`
- `media_image`
- `media_video`

## Patch Type

`PatchOp` MUST be one of:

- `{ "retain": <count> }`
- `{ "insert": <text> }`
- `{ "delete": <count> }`

Patch arrays MUST be applied in order and validated against base document length.

## Timestamp and ID Types

- IDs SHOULD be UUID v7.
- Timestamps MUST be UTC RFC3339 values.

## Related

- HTTP: [http.md](http.md)
- WebSocket: [websocket.md](websocket.md)
