# API Routes

## Browse Query Parameters

- `/search` accepts `q`, `kind`, `sort`, `cursor`, `limit`, `direction`, and `scope`.
- `/api/resources/search` accepts the same query parameters and returns JSON.
- `kind=all` is the default.
- `kind=note` narrows to notes only.
- `kind=media` narrows to media only.
- Popularity sorts are `popular_1d_desc`, `popular_7d_desc`, `popular_30d_desc`, `popular_90d_desc`, and `popular_all_desc`.

## Machine-Facing Routes

- `GET /api/resources/search` is the canonical assistant-facing search route.
- `GET /api/resources/{id}` returns the resource payload used by browser create and update responses.
- `GET /api/resources/{id}/history` returns the JSON saved-snapshot history shape.
- `POST /api/resources/notes` mirrors `POST /resources/notes`.
- `POST /api/resources/media` mirrors `POST /resources/media`.
- `PUT /api/resources/{id}` mirrors `PUT /resources/{id}`.

## Shared Update Rules

- `PUT /resources/{id}` accepts JSON updates for `body`, `alias`, `is_favorite`, and `is_private`.
- `PUT /api/resources/{id}` accepts the same JSON update shape.
- Every successful live-resource update creates one new immutable saved snapshot.

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
- `POST /api/resources/preview-markdown` uses the same renderer.
- Preview uses the same sanitized Markdown renderer as guest display.
