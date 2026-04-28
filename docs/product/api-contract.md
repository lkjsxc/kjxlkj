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
  "owner_note_id": null,
  "width": 1920,
  "height": 1080,
  "duration_ms": 93210,
  "media_variants": {
    "card": { "href": "/launch-video/file?variant=card", "content_type": "image/webp" },
    "poster": { "href": "/launch-video/file?variant=poster", "content_type": "image/webp" }
  },
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
- `media_variants` is nullable derivative metadata for media resources.
- `owner_note_id` is nullable and set only for media created from note attachment.

## Upload Limits

- Media upload limit defaults to `536870912` bytes.
- Site icon upload limit defaults to `2097152` bytes.
- Application-detected oversized multipart payloads return `413` with `payload_too_large` JSON.
- Browser upload clients must tolerate non-JSON limit errors from HTTP middleware or gateways and surface them as plain text instead of trying to parse them as JSON.
- Media upload file parts spill to temporary files while the multipart stream is read.
- SeaweedFS uploads read original media bodies from those temporary files rather than cloned in-memory buffers.
- Image derivative generation may read the source image into memory because the derivative encoder operates on decoded bytes.
- Video poster generation may invoke server-side FFmpeg against the temporary upload file.

## Browse Query Parameters

- `/search` accepts `q`, `kind`, `sort`, `cursor`, `limit`, `direction`, and `scope`.
- `/api/resources/search` accepts the same query parameters and returns JSON.
- `kind=all` is the default.
- `kind=note` narrows to notes only.
- `kind=media` narrows to media only.
- Popularity sort values are `popular_1d_desc`, `popular_7d_desc`, `popular_30d_desc`, `popular_90d_desc`, and `popular_all_desc`.

## Settings Schema

```json
{
  "site_name": "kjxlkj",
  "site_description": "Markdown-first resource system for LLM-operated workflows.",
  "public_base_url": "https://notes.example.com",
  "nostr_names": { "_": "7e7e9c42a91bfef19fa734ae08b1a69f3c4b5f0a74e7a9573c5d4be1f8f7f001" },
  "nostr_relays": ["wss://relay.example.com"],
  "live_default_source": "screen",
  "live_default_height": 1080,
  "live_default_fps": 60,
  "live_default_microphone_enabled": false,
  "google_maps_embed_api_key": "",
  "media_webp_quality": 82,
  "default_new_resource_is_private": false
}
```

- `media_webp_quality` is an integer from `1` through `100`.
- `nostr_names` accepts 64-character hex public keys or `npub...` input and stores lowercase hex.
- `nostr_relays` accepts `wss://` relay URLs.
- `live_default_source` is `screen` or `camera`.
- `live_default_height` is one of `360`, `480`, `720`, `1080`, `1440`, or `2160`.
- `live_default_fps` is one of `15`, `30`, `45`, `60`, or `120`.
- `live_default_microphone_enabled` controls whether new broadcasts request audio by default.
- `google_maps_embed_api_key` is optional and enables generated Google Maps iframe embeds.
- Blank `google_maps_embed_api_key` disables generated Google Maps embeds.
- Site icon upload requests use `multipart/form-data` rather than JSON.

## Nostr Discovery Response

```json
{
  "names": {
    "_": "7e7e9c42a91bfef19fa734ae08b1a69f3c4b5f0a74e7a9573c5d4be1f8f7f001"
  },
  "relays": {
    "7e7e9c42a91bfef19fa734ae08b1a69f3c4b5f0a74e7a9573c5d4be1f8f7f001": [
      "wss://relay.example.com"
    ]
  }
}
```

- `GET /.well-known/nostr.json` returns all configured names when `name` is omitted.
- `GET /.well-known/nostr.json?name=alice` returns only `alice` when configured.
- Unknown names return `200` with an empty `names` object.

## Site Icon Response

```json
{
  "configured": true,
  "href": "/assets/site-icon",
  "content_type": "image/png"
}
```

- `POST /admin/site-icon` is admin-only `multipart/form-data` with required part `icon`.
- `POST /admin/site-icon/reset` is admin-only and clears the uploaded icon state.
- Both routes return the same icon-state JSON shape.
- `configured=false` means the bundled fallback icon is active.

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
- Accepted direct-upload formats include current image and video formats plus file-family `.heic` and `.heif`.
- The server derives `media_family`, content metadata, and the initial Markdown body from the uploaded file.
- The server stores the original file and attempts derivative WebP preparation only for image and video media.

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
- Each created media stores the triggering note as immutable `owner_note_id`.
- Responses include `cursor_utf8`, the zero-based UTF-8 cursor offset after the inserted block in the saved body.

## Shared Update Rules

- `PUT /resources/{id}` accepts JSON updates for `body`, `alias`, `is_favorite`, and `is_private`.
- `PUT /api/resources/{id}` accepts the same JSON update shape.
- Every successful live-resource update creates one new immutable saved snapshot.

## Machine-Facing Routes

- `GET /api/resources/search` is the canonical assistant-facing search route.
- `GET /api/resources/{id}` returns the same resource payload used by browser
  create and update responses.
- `GET /api/resources/{id}/history` returns the same JSON snapshot history shape
  as `/resources/{id}/history`.
- `POST /api/resources/notes` mirrors `POST /resources/notes`.
- `POST /api/resources/media` mirrors `POST /resources/media`.

## Note Media Attachment Result

```json
{
  "current_resource": {
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
      "owner_note_id": "01jvq6z3f4t2p8k7m9n0b1c2d3",
      "file_href": "/ag6m3m3jy6hm74m6rfj7dnu3ga/file"
    }
  ],
  "selection_fallback": false,
  "cursor_utf8": 55
}
```

- The endpoint creates one media resource for each uploaded file.
- Attachment insertion is kind-aware: image Markdown image, video safe HTML video, file-family page link.
- The endpoint does not create generated notes that only link to or embed media.
- The target live note is updated only when the entire batch succeeds.
- `selection_fallback = true` means the embeds were appended because the submitted selection was not valid for the submitted draft body.
- `cursor_utf8` is authoritative for restoring the browser caret after upload.

## File Variant Query

- `GET /{ref}/file` returns the preserved original current file.
- `GET /{ref}/file?variant=card` returns a current card WebP when present for image or video media.
- `GET /{ref}/file?variant=display` returns a current display WebP when present for image media.
- `GET /{ref}/file?variant=poster` returns a current video poster WebP when present for video media.
- Snapshot file routes accept the same variant names and use saved derivative metadata.
- `variant=display` and `variant=card` may fall back to the raw original only when that original is reasonably browser-renderable inline.

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

- Media snapshots also preserve immutable object references and file metadata.
- Media snapshots also preserve immutable derivative metadata.
- Note snapshots omit media-only fields.
