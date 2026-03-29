# Postgres Schema Contract

## `records`

- `id`: `CHAR(26)` primary key.
- `alias`: nullable unique route alias.
- `body`: current Markdown body.
- `title`: current derived title.
- `summary`: current derived summary.
- `is_favorite`: favorite flag.
- `is_private`: privacy flag.
- `created_at`: immutable UTC timestamp.
- `updated_at`: mutable UTC timestamp.
- `deleted_at`: nullable soft-delete timestamp.
- `search_document`: current full-text search column.

## `record_revisions`

- `id`: surrogate primary key.
- `record_id`: note reference.
- `body`: historical snapshot body.
- `is_private`: historical snapshot privacy.
- `revision_number`: immutable per-note sequence.
- `created_at`: snapshot UTC timestamp.

## `app_settings`

- `id`: singleton key fixed to `1`.
- `home_recent_limit`: homepage recent-note count.
- `home_favorite_limit`: homepage favorite count.
- `search_results_per_page`: default HTML search page size.
- `updated_at`: mutable UTC timestamp.

## Required Indexes

- active-record index on non-deleted rows
- unique partial index on `alias`
- updated-order index for public/admin indexes
- created-order index for `Prev` / `Next`
- GIN index for `search_document`
- trigram index support for alias/title/body fallback matching
- revision lookup index on `(record_id, revision_number DESC)`
