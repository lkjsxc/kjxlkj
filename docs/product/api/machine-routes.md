# Machine Route Contracts

## Machine-Facing Routes

- `GET /api/resources/search` is the canonical assistant-facing search route.
- `GET /api/resources/{id}` returns the same resource payload used by browser create and update responses.
- `GET /api/resources/{id}/history` returns the same JSON saved-snapshot history shape as `/resources/{id}/history`.
- `POST /api/resources/notes` mirrors `POST /resources/notes`.
- `POST /api/resources/media` mirrors `POST /resources/media`.

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
