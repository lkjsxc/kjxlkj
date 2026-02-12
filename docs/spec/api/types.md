# External Types

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

## Core Resource Types

| Type | Required fields |
|---|---|
| `User` | `id`, `email`, `display_name`, `role`, `status`, `created_at` |
| `Workspace` | `id`, `slug`, `name`, `owner_user_id`, `created_at` |
| `WorkspaceMember` | `workspace_id`, `user_id`, `role`, `joined_at` |
| `Project` | `id`, `workspace_id`, `name`, `description`, `created_at` |
| `SavedView` | `id`, `workspace_id`, `query_json`, `sort`, `filters`, `owner_user_id` |
| `DashboardWidget` | `id`, `workspace_id`, `type`, `config_json`, `layout` |
| `AutomationRule` | `id`, `workspace_id`, `trigger`, `condition_json`, `action_json`, `enabled` |
| `AutomationRun` | `id`, `rule_id`, `status`, `started_at`, `finished_at`, `result_json` |
| `NoteStream` | `id`, `workspace_id`, `project_id`, `title`, `note_kind`, `access_scope`, `created_at`, `updated_at`, `current_version`, `deleted_at` |
| `NoteProjection` | `note_id`, `workspace_id`, `project_id`, `title`, `note_kind`, `version`, `markdown`, `rendered_html`, `metadata_json`, `search_vector` |
| `NoteEvent` | `event_id`, `note_id`, `seq`, `event_type`, `payload_json`, `actor_id`, `created_at` |
| `Attachment` | `id`, `note_id`, `filename`, `mime`, `size_bytes`, `sha256`, `chunk_count` |
| `AttachmentChunk` | `attachment_id`, `chunk_index`, `bytes` |

## Enum Types

`note_kind` MUST be one of:

- `markdown`
- `settings`
- `media_image`
- `media_video`

`role` MUST be one of:

- `owner`
- `admin`
- `editor`
- `viewer`

`access_scope` MUST be one of:

- `workspace`
- `project`
- `private`

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
