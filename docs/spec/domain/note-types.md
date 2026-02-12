# Note Types

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Canonical Note Kind Enum

`note_kind` MUST be one of:

| Kind | Purpose |
|---|---|
| `markdown` | default rich text knowledge note |
| `settings` | settings/configuration note with structured guidance |
| `media_image` | standalone image note |
| `media_video` | standalone video note |

## Behavioral Rules

- `markdown` and `settings` notes MUST support markdown body editing and title editing.
- `settings` notes SHOULD default to metadata/schema-aware rendering in UI.
- `media_image` and `media_video` notes MUST be creatable from upload-first flows.
- Media notes MUST remain first-class note streams (history/search/tags/deletion), not attachment-only side objects.

## Related

- Notes lifecycle: [notes.md](notes.md)
- Attachments/media storage: [attachments.md](attachments.md)
- API types: [/docs/spec/api/types.md](/docs/spec/api/types.md)
