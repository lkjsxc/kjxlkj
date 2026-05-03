# Write Endpoint Contracts

## Browse Query Parameters

- `/{user}/search` accepts `q`, `kind`, `sort`, `cursor`, `limit`, `direction`, and `scope`.
- `/api/users/{user}/resources/search` accepts the same query parameters and returns JSON.
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
  "visibility": "public"
}
```

- `POST /{user}/resources/notes` requires JSON `body`.
- `POST /api/users/{user}/resources/notes` accepts the same JSON shape.
- Browser-created notes seed `body` with a browser-local minute heading.
- Missing `visibility` uses the personal-space default.

## Media Create Payload

- `POST /{user}/resources/media` is `multipart/form-data`.
- `POST /api/users/{user}/resources/media` accepts the same multipart shape.
- Required part: `file`.
- Optional parts: `alias`, `is_favorite`, and `visibility`.
- Accepted direct-upload formats include current image and video formats plus file-family `.heic` and `.heif`.
- The server derives media family, content metadata, and initial Markdown body from the uploaded file.
- The server stores the original file and attempts derivative WebP preparation only for image and video media.

## Note Media Attachment Payload

- `POST /{user}/resources/{id}/media-attachments` is `multipart/form-data`.
- Required parts: one or more `file` values plus `body`, `is_favorite`, `visibility`, `insert_start`, and `insert_end`.
- Optional part: `alias`.
- `body`, `alias`, `is_favorite`, and `visibility` describe the unsaved live-note draft that becomes authoritative if the batch succeeds.
- `body` may be empty or whitespace-only and must be preserved exactly after UTF-8 decoding.
- `insert_start` and `insert_end` are zero-based UTF-8 string indices into that draft body.
- Invalid, reversed, or stale insertion ranges append embeds to the submitted draft and set `selection_fallback`.
- The endpoint is valid only for live notes.
- Each created media stores the triggering note as immutable `owner_note_id`.
- Responses include `cursor_utf8`, the zero-based UTF-8 cursor offset after the inserted block in the saved body.

## Shared Update Rules

- `PUT /{user}/resources/{id}` accepts JSON updates for `body`, `alias`, `is_favorite`, and `visibility`.
- `PUT /api/users/{user}/resources/{ref}` accepts the same JSON update shape.
- Every successful live-resource update creates one new immutable saved snapshot.

## Delete

- `DELETE /{user}/resources/{id}` soft-deletes a live resource.
- `DELETE /api/users/{user}/resources/{ref}` soft-deletes a live resource.
- Successful delete returns `204`.

## File Variant Query

- `GET /{user}/{ref}/file` returns the preserved original current file.
- `GET /{user}/{ref}/file?variant=card` returns a current card WebP when present for image or video media.
- `GET /{user}/{ref}/file?variant=display` returns a current display WebP when present for image media.
- `GET /{user}/{ref}/file?variant=poster` returns a current video poster WebP when present for video media.
- Snapshot file routes accept the same variant names and use saved derivative metadata.
- `variant=display` and `variant=card` may fall back to the raw original only when that original is reasonably browser-renderable inline.
