# API Contract

## Resource

The system manages `note` resources (internally called `record`).

## Note Schema

```json
{
  "slug": "2026-03-25-0134",
  "body": "# Title\n\nMarkdown content...",
  "is_private": true,
  "created_at": "2026-03-25T01:34:00Z",
  "updated_at": "2026-03-25T01:34:00Z"
}
```

## Field Rules

- `slug`: Primary key. Auto-generated from datetime on creation. Format: `YYYY-MM-DD-HHmm` (e.g., `2026-03-25-0134`). Lowercase with dashes. Minimum 3 characters, maximum 64 characters.
- `body`: UTF-8 Markdown content. May be empty. First `# heading` line is extracted as the display title.
- `is_private`: Boolean. Default `true`. When `true`, only authenticated admin can view. When `false`, publicly accessible.
- `created_at`: UTC RFC3339 timestamp. Set on creation, never modified.
- `updated_at`: UTC RFC3339 timestamp. Updated on every save.

## UI Semantics

- The public-facing control is `Public`.
- `Public = checked` maps to `is_private = false`.
- `Public = unchecked` maps to `is_private = true`.

## Title Extraction

The display title is extracted from the body:

1. Find the first line matching `^# (.+)$`.
2. If found, use the captured text as the title.
3. If not found, use the slug as the title.

## Revision History

Every update creates a new revision:

```json
{
  "revision_number": 3,
  "body": "# Title\n\nOld content...",
  "is_private": true,
  "created_at": "2026-03-25T01:30:00Z"
}
```

Revisions are immutable snapshots of past states.

## Navigation Payload

The previous and next JSON endpoints return:

```json
{
  "slug": "2026-03-25-0134"
}
```

When no accessible neighbor exists, `slug` is `null`.
