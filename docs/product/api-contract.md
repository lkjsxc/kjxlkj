# API Contract

## Resource

The system manages `note` resources.

## Note Schema

```json
{
  "id": "01jvq6z3f4t2p8k7m9n0b1c2d3",
  "alias": "release-notes",
  "body": "# Title\n\nMarkdown content...",
  "is_favorite": true,
  "favorite_position": 2,
  "is_private": true,
  "created_at": "2026-03-26T08:34:00Z",
  "updated_at": "2026-03-26T08:35:00Z"
}
```

## Field Rules

- `id`: Primary key. Exact 26-character lowercase Base32 string representing 128 random bits.
- `alias`: Optional lowercase route alias.
- `body`: UTF-8 Markdown content. May be empty. Supported authoring paths include headings, lists, task lists, blockquotes, fenced code, links, and GFM tables.
- `is_favorite`: Boolean. Default `false`.
- `favorite_position`: Nullable positive integer. Present when the note is favorited.
- `is_private`: Boolean. Default `true`.
- `created_at`: UTC RFC3339 timestamp.
- `updated_at`: UTC RFC3339 timestamp.

## Admin Analytics Schema

```json
{
  "view_count_total": 18,
  "view_count_7d": 4,
  "view_count_30d": 9,
  "view_count_90d": 12,
  "last_viewed_at": "2026-04-01T08:35:00Z"
}
```

- Analytics fields are admin-only presentation data.
- Non-admin HTML and non-admin note payloads do not expose view totals.

## Create Payload

```json
{
  "body": "# 2026-03-27 21:04\n",
  "alias": null,
  "is_favorite": false,
  "is_private": true
}
```

- `POST /records` requires `body`.
- Browser-created notes use a browser-local minute timestamp heading as the default title seed.
- Browser-created notes initialize `is_private` from the current global default-new-note visibility setting.
- The server does not synthesize fallback body text when `body` is omitted.

## Derived Presentation Rules

- Display title is extracted from the first `# ` heading line.
- Missing heading yields `Untitled note`.
- Normal UI does not display raw `id` values.
- Created time is the secondary identity cue in lists and note chrome.
- Admin note pages edit the canonical body through a first-party Markdown textarea with on-demand preview.
- Admin note pages should open with keyboard focus in the visible editor.
- Note pages do not render a duplicate visible title outside the Markdown body.
- The editor does not show a visible `Markdown body` label.
- Alias typing must preserve internal `-`, `_`, and `.` separators until save-time validation.
- Homepage hero content uses only editable intro Markdown.
- Homepage and dashboard popularity default to `30d` and switch windows without mutating the visible URL.
- `/search` popularity supports `popular_window=7d|30d|90d` and defaults to `30d`.
- Public note URLs prefer `alias` when present.
- `/search` with empty `q` is the canonical paginated all-notes card view.

## UI Semantics

- The public-facing control is `Public`.
- `Public = checked` maps to `is_private = false`.
- `Public = unchecked` maps to `is_private = true`.
- Search and browse UI are canonical on `/search`.
- Favorite state is explicit admin-managed note state.
- Favorite ordering is explicit admin-managed note state.

## Preview API

```json
{
  "body": "# Preview me"
}
```

```json
{
  "html": "<h1>Preview me</h1>\n"
}
```

- `POST /admin/markdown-preview` is admin-only.
- The endpoint renders Markdown through the same server path used by note display.
- Preview rendering does not mutate note state.

## Saved Snapshot History

Creating a note creates saved snapshot `1`. Every successful update creates one new immutable saved snapshot:

```json
{
  "id": "aj6m3m3jy6hm74m6rfj7dnu3ga",
  "snapshot_number": 3,
  "alias": "release-notes",
  "title": "Title",
  "summary": "Old content...",
  "body": "# Title\n\nSaved content...",
  "is_private": true,
  "created_at": "2026-03-26T08:35:00Z"
}
```

History listing responses are paginated:

```json
{
  "snapshots": [
    {
      "id": "aj6m3m3jy6hm74m6rfj7dnu3ga",
      "snapshot_number": 3,
      "alias": "release-notes",
      "title": "Title",
      "summary": "Saved content...",
      "body": "# Title\n\nSaved content...",
      "is_private": true,
      "created_at": "2026-03-26T08:35:00Z"
    }
  ],
  "previous_cursor": null,
  "next_cursor": "opaque"
}
```

## Navigation Payload

```json
{
  "id": "01jvq6z3f4t2p8k7m9n0b1c2d3"
}
```

When no accessible neighbor exists, `id` is `null`.
