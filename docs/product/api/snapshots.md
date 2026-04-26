# Saved Snapshot API

## File Variant Query

- `GET /{ref}/file` returns the preserved original current file.
- `GET /{ref}/file?variant=card` returns a current card WebP when present for image or video media.
- `GET /{ref}/file?variant=display` returns a current display WebP when present for image media.
- `GET /{ref}/file?variant=poster` returns a current video poster WebP when present for video media.
- Snapshot file routes accept the same variant names and use saved derivative metadata.
- `variant=display` and `variant=card` may fall back to the raw original only when that original is reasonably browser-renderable inline.

## Saved Snapshot Schema

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
  "media_variants": {
    "card": { "href": "/aj6m3m3jy6hm74m6rfj7dnu3ga/file?variant=card", "content_type": "image/webp" },
    "poster": { "href": "/aj6m3m3jy6hm74m6rfj7dnu3ga/file?variant=poster", "content_type": "image/webp" }
  },
  "created_at": "2026-03-26T08:35:00Z"
}
```

## Rules

- Media snapshots preserve immutable object references and file metadata.
- Media snapshots preserve immutable derivative metadata.
- Note snapshots omit media-only fields.
