# Resource API Schema

## Base Resource

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

## Field Rules

- `kind` is `note` or `media`.
- `id` is an exact 26-character lowercase Base32 string.
- `alias` is an optional lowercase route alias unique across all live resources.
- `body` is UTF-8 Markdown content and may be empty.
- `is_favorite` defaults to `false`.
- `favorite_position` is nullable and positive when present.
- `is_private` defaults from `default_new_resource_is_private`.
- `created_at` and `updated_at` are UTC RFC3339 timestamps.
- Media-only fields are absent for `note`.

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

## Media Rules

- `media_family` is `image`, `video`, or `file`.
- `media_variants` is nullable derivative metadata.
- `owner_note_id` is set only for media created from note attachment.
- Image and video media may include dimensions.
- Video media may include `duration_ms`.

## Saved Snapshot

```json
{
  "id": "aj6m3m3jy6hm74m6rfj7dnu3ga",
  "resource_id": "01jvq6z3f4t2p8k7m9n0b1c2d3",
  "kind": "media",
  "snapshot_number": 3,
  "alias": "launch-video",
  "title": "Launch Video",
  "summary": "Primary product walkthrough.",
  "body": "# Launch Video\n\nPrimary product walkthrough.",
  "is_private": false,
  "owner_note_id": null,
  "file_href": "/aj6m3m3jy6hm74m6rfj7dnu3ga/file",
  "content_type": "video/mp4",
  "byte_size": 18342012,
  "created_at": "2026-03-26T08:35:00Z"
}
```

## Saved Snapshot Rules

- Media snapshots preserve immutable object references and file metadata.
- Media snapshots preserve immutable derivative metadata.
- Note snapshots omit media-only fields.
