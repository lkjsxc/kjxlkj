# Postgres Schema Contract

## `records`

- `id`: `CHAR(22)` primary key.
- `body`: current Markdown body.
- `title`: current derived title.
- `summary`: current derived summary.
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

## Required Indexes

- active-record index on non-deleted rows
- updated-order index for public/admin indexes
- created-order index for `Prev` / `Next`
- GIN index for `search_document`
- revision lookup index on `(record_id, revision_number DESC)`
