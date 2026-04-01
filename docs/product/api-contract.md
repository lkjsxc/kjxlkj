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
  "view_count_total": 18,
  "view_count_7d": 4,
  "view_count_30d": 9,
  "view_count_90d": 12,
  "last_viewed_at": "2026-04-01T08:35:00Z",
  "created_at": "2026-03-26T08:34:00Z",
  "updated_at": "2026-03-26T08:35:00Z"
}
```

## Field Rules

- `id`: primary key. Exact 26-character lowercase Base32 string representing 128 random bits.
- `alias`: optional lowercase route alias using letters, digits, `.`, `_`, and `-`.
- `body`: UTF-8 Markdown content. May be empty. Supported authoring paths include headings, lists, task lists, blockquotes, fenced code, links, and GFM tables.
- `is_favorite`: boolean. Default `false`.
- `favorite_position`: nullable positive integer. Present when the note is favorited.
- `is_private`: boolean. New-note default is controlled by global settings.
- `view_count_total`: lifetime successful note-page view count.
- `view_count_7d`: rolling 7-day view count.
- `view_count_30d`: rolling 30-day view count.
- `view_count_90d`: rolling 90-day view count.
- `last_viewed_at`: nullable UTC RFC3339 timestamp for the last counted note view.
- `created_at`: UTC RFC3339 timestamp.
- `updated_at`: UTC RFC3339 timestamp.

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
- When `is_private` is omitted, the server uses the configured global new-note default.
- The server does not synthesize fallback body text when `body` is omitted.

## Derived Presentation Rules

- Display title is extracted from the first `# ` heading line.
- Missing heading yields `Untitled note`.
- Normal UI does not display raw `id` values.
- Created time is the secondary identity cue in lists and note chrome.
- Admin note pages edit the canonical body through one in-house textarea-first Markdown workspace with on-demand preview.
- Admin note pages should open with keyboard focus in the visible editor.
- Homepage supports optional admin-authored intro Markdown below the page title and inline admin editing of that block.
- Homepage popularity supports `popular_window=7d|30d|90d` and defaults to `30d`.
- Homepage `Popular notes`, `Recently updated`, and `Favorites` use configurable visibility, order, and per-section counts.
- Homepage section counts default to `5`.
- Public note URLs prefer `alias` when present.
- `/search` with empty `q` is the canonical paginated all-notes card view.
- `/search` also owns favorites and popularity preset browsing through query parameters rather than separate routes.

## UI Semantics

- The public-facing control is `Public`.
- `Public = checked` maps to `is_private = false`.
- `Public = unchecked` maps to `is_private = true`.
- Search and browse UI are canonical on `/search`.
- Favorite state is explicit admin-managed note state.
- Favorite ordering is explicit admin-managed note state.

## Preview Payload

`POST /preview` accepts:

```json
{
  "body": "# Title\n\nDraft Markdown..."
}
```

Response:

```json
{
  "html": "<h1>Title</h1>\n<p>Draft Markdown...</p>\n"
}
```

- `POST /preview` is admin-only.
- Preview rendering uses the same Markdown renderer as guest note pages.

## Revision History

Every update creates a new immutable revision snapshot:

```json
{
  "revision_number": 3,
  "body": "# Title\n\nOld content...",
  "is_private": true,
  "created_at": "2026-03-26T08:35:00Z"
}
```

History listing responses are paginated:

```json
{
  "revisions": [
    {
      "revision_number": 3,
      "body": "# Title\n\nOld content...",
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
