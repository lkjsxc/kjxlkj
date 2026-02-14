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
| `DashboardWidget` | `id`, `workspace_id`, `type`, `config_json`, `layout` (optional extension) |
| `AutomationRule` | `id`, `workspace_id`, `trigger`, `condition_json`, `action_json`, `enabled` |
| `AutomationRun` | `id`, `rule_id`, `status`, `started_at`, `finished_at`, `result_json` |
| `LibrarianProviderConfig` | `provider_kind`, `model`, `base_url`, `timeout_ms`, `max_tokens`, `temperature`, `fallback_models` |
| `LibrarianStructuringPlan` | `goal`, `scope`, `taxonomy_json`, `style_profile`, `strict_mode`, `max_operations` |
| `LibrarianOperation` | `operation_id`, `kind`, `target_note_id`, `target_path`, `title`, `body_markdown`, `reason`, `confidence` |
| `LibrarianRunReport` | `run_id`, `provider_kind`, `model`, `prompt_hash`, `parsed_operations`, `applied_operations`, `rejected_operations`, `warnings` |
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

`librarian_provider_kind` MUST be one of:

- `openrouter`
- `lmstudio`

`librarian_operation_kind` MUST be one of:

- `create_note`
- `rewrite_note`
- `retitle_note`
- `relink_note`
- `retag_note`
- `defer`

## Patch Type

`PatchOp` MUST be one of:

- `{ "retain": <count> }`
- `{ "insert": <text> }`
- `{ "delete": <count> }`

Patch arrays MUST be applied in order and validated against base document length.

`AutomationRule.action_json` for librarian behavior MUST include:

- `kind = "librarian_structure"`
- `provider` matching `LibrarianProviderConfig`
- `plan` matching `LibrarianStructuringPlan`
- `protocol = "xml_attrless"`

## Timestamp and ID Types

- IDs SHOULD be UUID v7.
- Timestamps MUST be UTC RFC3339 values.

## Related

- HTTP: [http.md](http.md)
- WebSocket: [websocket.md](websocket.md)
