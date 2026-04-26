# Resource API Schema

## Shared Resource

```json
{
  "id": "01jvq6z3f4t2p8k7m9n0b1c2d3",
  "kind": "note",
  "alias": "release-notes",
  "body": "# Title\n\nMarkdown content...",
  "is_favorite": true,
  "favorite_position": 2,
  "is_private": false,
  "created_at": "2026-03-26T08:34:00Z",
  "updated_at": "2026-03-26T08:35:00Z"
}
```

## Media Extension

```json
{
  "kind": "media",
  "media_family": "video",
  "file_href": "/launch-video/file",
  "content_type": "video/mp4",
  "byte_size": 18342012,
  "original_filename": "launch-video.mp4",
  "sha256_hex": "c0ffee...",
  "owner_note_id": null,
  "width": 1920,
  "height": 1080,
  "duration_ms": 93210,
  "media_variants": {
    "card": { "href": "/launch-video/file?variant=card", "content_type": "image/webp" },
    "poster": { "href": "/launch-video/file?variant=poster", "content_type": "image/webp" }
  }
}
```

## Field Rules

- `kind` is `note` or `media`.
- `id` is an exact 26-character lowercase Base32 string.
- `alias` is nullable and unique across live resources.
- `body` stores UTF-8 Markdown and may be empty.
- `is_favorite` defaults to `false`.
- `favorite_position` is nullable and positive when present.
- `is_private` defaults from `default_new_resource_is_private`.
- Timestamps are UTC RFC3339 strings.
- Media-only fields are absent for notes.
- `media_variants` is nullable derivative metadata for media resources.
- `owner_note_id` is set only for media created from note attachment.
