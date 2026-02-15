# External Types

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

## Core Resource Types

| Type | Required fields |
|---|---|
| `Workspace` | `id`, `slug`, `name`, `owner_user_id`, `created_at` |
| `NoteStream` | `id`, `workspace_id`, `title`, `note_kind`, `current_version`, `created_at`, `updated_at` |
| `NoteProjection` | `note_id`, `title`, `version`, `markdown`, `metadata_json` |
| `SearchResult` | `note_id`, `title`, `snippet`, `score_lexical`, `score_semantic`, `score_final` |
| `AutomationRule` | `id`, `workspace_id`, `trigger`, `condition_json`, `action_json`, `enabled` |
| `AutomationRun` | `id`, `rule_id`, `status`, `started_at`, `finished_at`, `result_json` |
| `AgentPromptConfigRef` | `path`, `hash`, `loaded_at` |

## Agent Action JSON

`AutomationRule.action_json` for `kjxlkj-agent` MUST include:

- `kind = "kjxlkj_agent"`
- `mode` (`reviewed` or `yolo`)
- `prompt_json_path`
- `provider` (`openrouter` or `lmstudio`)
- `memory` (`type = "kv_store"`, `carry_over = true`)

## Note Invariants

- `id` is immutable unique identifier.
- `title` is mutable and independent of `id`.
- Untitled create requests MUST receive datetime title.

## Enum Types

`note_kind` MUST be one of:

- `markdown`
- `settings`
- `media_image`
- `media_video`

`search_mode` MUST be one of:

- `hybrid`
- `lexical`
- `semantic`

## Related

- HTTP: [http.md](http.md)
- Domain notes: [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
- Domain search: [/docs/spec/domain/search.md](/docs/spec/domain/search.md)
