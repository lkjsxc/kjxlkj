# Postgres Schema Contract

## `records`

- `id`: `CHAR(26)` primary key.
- `alias`: nullable unique route alias.
- `body`: current Markdown body.
- `title`: current derived title.
- `summary`: current derived summary.
- `is_favorite`: favorite flag.
- `favorite_position`: nullable persistent favorite ordering slot.
- `is_private`: privacy flag.
- `view_count_total`: lifetime successful note-page views.
- `last_viewed_at`: last counted note view timestamp.
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

## `record_daily_views`

- `record_id`: note reference.
- `view_date`: UTC date bucket.
- `view_count`: counted note views for that UTC day.
- Primary key: `(record_id, view_date)`.

## `app_settings`

- `id`: singleton key fixed to `1`.
- `home_recent_limit`: homepage recent-note count.
- `home_favorite_limit`: homepage favorite count.
- `home_popular_limit`: homepage popular-note count.
- `home_intro_markdown`: optional homepage intro Markdown.
- `search_results_per_page`: default HTML search page size.
- `default_vim_mode`: global default editor Vim flag.
- `updated_at`: mutable UTC timestamp.

## Required Indexes

- active-record index on non-deleted rows
- unique partial index on `alias`
- updated-order index for public/admin indexes
- created-order index for `Prev` / `Next`
- favorite-order index on `(favorite_position, id)` for active favorites
- record-daily-view lookup index covering `(view_date, view_count DESC)`
- GIN index for `search_document`
- trigram index support for alias/title/body fallback matching
- revision lookup index on `(record_id, revision_number DESC)`
