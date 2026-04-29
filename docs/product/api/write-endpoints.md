# Write Endpoint Contracts

## Browse Query Parameters

- `/search` accepts `q`, `kind`, `sort`, `cursor`, `limit`, `direction`, and `scope`.
- `/api/resources/search` accepts the same query parameters and returns JSON.
- `kind=all` is the default.
- `kind=note` narrows to notes only.
- `kind=media` narrows to media only.
- Popularity sort values are `popular_1d_desc`, `popular_7d_desc`, `popular_30d_desc`, `popular_90d_desc`, and `popular_all_desc`.

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
- Optional parts: `alias`, `is_favorite`, and `is_private`.
- Accepted direct-upload formats include current image and video formats plus file-family `.heic` and `.heif`.
- The server derives media family, content metadata, and initial Markdown body from the uploaded file.
- The server stores the original file and attempts derivative WebP preparation only for image and video media.

## Note Media Attachment Payload

- `POST /resources/{id}/media-attachments` is `multipart/form-data`.
- Required parts: one or more `file` values plus `body`, `is_favorite`, `is_private`, `insert_start`, and `insert_end`.
- Optional part: `alias`.
- `body`, `alias`, `is_favorite`, and `is_private` describe the unsaved live-note draft that becomes authoritative if the batch succeeds.
- `body` may be empty or whitespace-only and must be preserved exactly after UTF-8 decoding.
- `insert_start` and `insert_end` are zero-based UTF-8 string indices into that draft body.
- Invalid, reversed, or stale insertion ranges append embeds to the submitted draft and set `selection_fallback`.
- The endpoint is valid only for live notes.
- Each created media stores the triggering note as immutable `owner_note_id`.
- Responses include `cursor_utf8`, the zero-based UTF-8 cursor offset after the inserted block in the saved body.

## Shared Update Rules

- `PUT /resources/{id}` accepts JSON updates for `body`, `alias`, `is_favorite`, and `is_private`.
- `PUT /api/resources/{id}` accepts the same JSON update shape.
- Every successful live-resource update creates one new immutable saved snapshot.

## File Variant Query

- `GET /{ref}/file` returns the preserved original current file.
- `GET /{ref}/file?variant=card` returns a current card WebP when present for image or video media.
- `GET /{ref}/file?variant=display` returns a current display WebP when present for image media.
- `GET /{ref}/file?variant=poster` returns a current video poster WebP when present for video media.
- Snapshot file routes accept the same variant names and use saved derivative metadata.
- `variant=display` and `variant=card` may fall back to the raw original only when that original is reasonably browser-renderable inline.
