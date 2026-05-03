# Machine Route Contracts

## Machine-Facing Routes

- `GET /api/users/{user}/resources/search` is the canonical assistant-facing search route.
- `GET /api/users/{user}/resources/{ref}` returns the resource payload.
- `GET /api/users/{user}/resources/{ref}/history` returns saved-snapshot history JSON.
- `POST /api/users/{user}/resources/notes` creates a note.
- `POST /api/users/{user}/resources/media` creates a media resource.
- `PUT /api/users/{user}/resources/{ref}` updates a resource.
- `DELETE /api/users/{user}/resources/{ref}` soft-deletes a resource.

## Machine Auth

- Machine routes accept `Authorization: Bearer <token>`.
- Tokens belong to one service account and one personal space.
- Token scopes include `resource:read` and `resource:write`.
- Service tokens never grant member-management or settings-management access.
- Cookie sessions may use machine routes only when CSRF validation passes.

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

- `POST /{user}/settings/site-icon` requires `ManageSettings` and `multipart/form-data`.
- `POST /{user}/settings/site-icon/reset` requires `ManageSettings`.
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

- `POST /{user}/markdown-preview` requires `WriteResource`.
- The endpoint uses the same sanitized Markdown renderer as guest display.
