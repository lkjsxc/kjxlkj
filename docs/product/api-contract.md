# API Contract

## Resource

The system manages `note` resources.

## Note Schema

```json
{
  "id": "Q29udHJhY3RSdW50aW1lMQ",
  "body": "# Title\n\nMarkdown content...",
  "is_private": true,
  "created_at": "2026-03-26T08:34:00Z",
  "updated_at": "2026-03-26T08:35:00Z"
}
```

## Field Rules

- `id`: Primary key. Exact 22-character Base64URL string representing 128 random bits.
- `body`: UTF-8 Markdown content. May be empty. Supported authoring paths include headings, lists, task lists, blockquotes, fenced code, links, and GFM tables.
- `is_private`: Boolean. Default `true`.
- `created_at`: UTC RFC3339 timestamp.
- `updated_at`: UTC RFC3339 timestamp.

## Create Payload

```json
{
  "body": "# 2026-03-27 21:04\n",
  "is_private": true
}
```

- `POST /records` requires `body`.
- Browser-created notes use a browser-local minute timestamp heading as the default title seed.
- The server does not synthesize fallback body text when `body` is omitted.

## Derived Presentation Rules

- Display title is extracted from the first `# ` heading line.
- Missing heading yields `Untitled note`.
- Normal UI does not display raw `id` values.
- Created time is the secondary identity cue in lists and note chrome.
- Admin note pages edit the canonical body through a single rendered Markdown workspace.
- Admin note pages should open with keyboard focus in the visible editor.

## UI Semantics

- The public-facing control is `Public`.
- `Public = checked` maps to `is_private = false`.
- `Public = unchecked` maps to `is_private = true`.
- Search UI is canonical on `/search`.

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

## Navigation Payload

```json
{
  "id": "Q29udHJhY3RSdW50aW1lMQ"
}
```

When no accessible neighbor exists, `id` is `null`.
