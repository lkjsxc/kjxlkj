# Note Media Attachment API

## Request

- `POST /resources/{id}/media-attachments` is `multipart/form-data`.
- Required parts: one or more `file` values plus `body`, `is_favorite`, `is_private`, `insert_start`, and `insert_end`.
- Optional part: `alias`.
- The endpoint is valid only for live notes.
- `body`, `alias`, `is_favorite`, and `is_private` describe the unsaved live-note draft that should become authoritative if the batch succeeds.
- The `body` part may be empty or whitespace-only and must be preserved exactly after UTF-8 decoding.
- `insert_start` and `insert_end` are zero-based UTF-8 string indices into the draft body.
- Valid insertion ranges replace the selected draft slice with inserted embeds.
- Invalid, reversed, or stale insertion ranges append embeds to the submitted draft and set `selection_fallback`.
- Each created media stores the triggering note as immutable `owner_note_id`.

## Response

```json
{
  "current_resource": {
    "id": "01jvq6z3f4t2p8k7m9n0b1c2d3",
    "kind": "note",
    "alias": "release-notes",
    "body": "# Title\n\n![](/ag6m3m3jy6hm74m6rfj7dnu3ga/file)\n",
    "is_favorite": true,
    "favorite_position": 2,
    "is_private": false
  },
  "inserted_markdown": "![](/ag6m3m3jy6hm74m6rfj7dnu3ga/file)\n",
  "created_media": [
    {
      "id": "ag6m3m3jy6hm74m6rfj7dnu3ga",
      "kind": "media",
      "alias": null,
      "owner_note_id": "01jvq6z3f4t2p8k7m9n0b1c2d3",
      "file_href": "/ag6m3m3jy6hm74m6rfj7dnu3ga/file"
    }
  ],
  "selection_fallback": false,
  "cursor_utf8": 55
}
```

## Rules

- The endpoint creates one media resource for each uploaded file.
- Attachment insertion is kind-aware: image Markdown image, video safe HTML video, file-family page link.
- The endpoint does not create generated notes that only link to or embed media.
- The target live note is updated only when the entire batch succeeds.
- `cursor_utf8` is authoritative for restoring the browser caret after upload.
