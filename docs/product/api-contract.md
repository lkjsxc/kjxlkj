# API Contract

## Resource Schema

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

## Media Resource Extension

```json
{
  "id": "01jvq6z3f4t2p8k7m9n0b1c2d3",
  "kind": "media",
  "alias": "launch-video",
  "body": "# Launch Video\n\nPrimary product walkthrough.",
  "media_family": "video",
  "file_href": "/launch-video/file",
  "content_type": "video/mp4",
  "byte_size": 18342012,
  "original_filename": "launch-video.mp4",
  "sha256_hex": "c0ffee...",
  "width": 1920,
  "height": 1080,
  "duration_ms": 93210,
  "is_favorite": false,
  "favorite_position": null,
  "is_private": false,
  "created_at": "2026-03-26T08:34:00Z",
  "updated_at": "2026-03-26T08:35:00Z"
}
```

## Field Rules

- `kind`: `note` or `media`.
- `id`: exact 26-character lowercase Base32 string.
- `alias`: optional lowercase route alias unique across all live resources.
- `body`: UTF-8 Markdown content. May be empty.
- `is_favorite`: boolean. Default `false`.
- `favorite_position`: nullable positive integer.
- `is_private`: boolean. Default comes from `default_new_resource_is_private`.
- `created_at` and `updated_at`: UTC RFC3339 timestamps.
- Media-only fields are absent for `note`.

## Browse Query Parameters

- `/search` accepts `q`, `kind`, `sort`, `cursor`, `limit`, `direction`, `scope`, and `popular_window`.
- `kind=all` is the default.
- `kind=note` narrows to notes only.
- `kind=media` narrows to media only.
- `popular_window` accepts `7d`, `30d`, `90d`, and `all`.

## Settings Schema

```json
{
  "site_name": "kjxlkj",
  "site_description": "Markdown-first resource system for LLM-operated workflows.",
  "public_base_url": "https://notes.example.com",
  "default_new_resource_is_private": false
}
```

## Note Create Payload

```json
{
  "body": "# 2026-03-27 21:04\n",
  "alias": null,
  "is_favorite": false,
  "is_private": false
}
```

- `POST /resources/notes` requires JSON `body`.
- Browser-created notes seed `body` with a browser-local minute heading.

## Media Create Payload

- `POST /resources/media` is `multipart/form-data`.
- Required part: `file`.
- Optional parts: `alias`, `is_favorite`, `is_private`.
- The server derives `media_family`, content metadata, and the initial Markdown body from the uploaded file.

## Note Media Attachment Payload

- `POST /resources/{id}/media-attachments` is `multipart/form-data`.
- Required parts: one or more `file` values plus `body`, `is_favorite`, `is_private`, `insert_start`, and `insert_end`.
- Optional part: `alias`.
- `body`, `alias`, `is_favorite`, and `is_private` describe the current unsaved live-note draft that should become authoritative if the batch succeeds.
- The `body` part may be empty or whitespace-only and must be preserved exactly after UTF-8 decoding.
- `insert_start` and `insert_end` are zero-based UTF-8 string indices into that draft body.
- Valid insertion ranges replace the selected draft slice with the inserted embeds.
- Invalid, reversed, or stale insertion ranges append the embeds to the end of the submitted draft and set `selection_fallback` in the response.
- The endpoint is valid only for live notes.

## Shared Update Rules

- `PUT /resources/{id}` accepts JSON updates for `body`, `alias`, `is_favorite`, and `is_private`.
- Every successful live-resource update creates one new immutable saved snapshot.

## Note Media Attachment Result

```json
{
  "current_note": {
    "id": "01jvq6z3f4t2p8k7m9n0b1c2d3",
    "kind": "note",
    "alias": "release-notes",
    "body": "# Title\n\n![](/ag6m3m3jy6hm74m6rfj7dnu3ga/file)\n",
    "is_favorite": true,
    "favorite_position": 2,
    "is_private": false,
    "created_at": "2026-03-26T08:34:00Z",
    "updated_at": "2026-03-26T08:40:00Z"
  },
  "inserted_markdown": "![](/ag6m3m3jy6hm74m6rfj7dnu3ga/file)\n",
  "created_media": [
    {
      "id": "ag6m3m3jy6hm74m6rfj7dnu3ga",
      "kind": "media",
      "alias": null,
      "file_href": "/ag6m3m3jy6hm74m6rfj7dnu3ga/file"
    }
  ],
  "selection_fallback": false
}
```

- The endpoint creates one media resource for each uploaded file.
- The endpoint does not create generated notes that only link to or embed media.
- The current note is updated only when the entire batch succeeds.
- `selection_fallback = true` means the embeds were appended because the submitted selection was not valid for the submitted draft body.

## Preview API

```json
{
  "body": "# Preview me\n\n![](/demo/file)\n\n<video controls src=\"/clip/file\"></video>"
}
```

```json
{
  "html": "<h1>Preview me</h1>\n<p><img src=\"/demo/file\" alt=\"\"></p>\n<video controls=\"\" src=\"/clip/file\"></video>\n"
}
```

- `POST /admin/markdown-preview` is admin-only.
- The endpoint uses the same sanitized Markdown renderer as guest display.

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
  "file_href": "/aj6m3m3jy6hm74m6rfj7dnu3ga/file",
  "content_type": "video/mp4",
  "byte_size": 18342012,
  "created_at": "2026-03-26T08:35:00Z"
}
```

- Media snapshots also preserve immutable object references and file metadata.
- Note snapshots omit media-only fields.
