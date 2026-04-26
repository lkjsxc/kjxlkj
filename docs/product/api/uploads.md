# Upload API

## Upload Limits

- Media upload limit defaults to `536870912` bytes.
- Site icon upload limit defaults to `2097152` bytes.
- Application-detected oversized multipart payloads return `413` with `payload_too_large` JSON.
- Middleware or gateway limit errors may be plain text.
- Browser upload clients must handle both JSON and plain-text limit errors.
- Media upload file parts spill to temporary files while the multipart stream is read.
- SeaweedFS uploads read original media bodies from temporary files.
- Image derivative generation may read decoded image bytes into memory.
- Video poster generation may invoke server-side FFmpeg against the temporary upload file.

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
- Accepted direct-upload formats include common image and video formats plus file-family `.heic` and `.heif`.
- The server derives `media_family`, content metadata, and initial Markdown body from the uploaded file.
- The server stores the original file and attempts derivative WebP preparation only for image and video media.
