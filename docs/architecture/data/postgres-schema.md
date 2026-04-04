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

- `id`: `CHAR(26)` primary key.
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
- `home_intro_markdown`: optional homepage hero Markdown.
- `home_recent_limit`: homepage recent-note count.
- `home_favorite_limit`: homepage favorite count.
- `home_popular_limit`: homepage popular-note count.
- `home_recent_visible`: homepage recent-section visibility.
- `home_favorite_visible`: homepage favorite-section visibility.
- `home_popular_visible`: homepage popular-section visibility.
- `home_recent_position`: homepage recent-section order slot.
- `home_favorite_position`: homepage favorite-section order slot.
- `home_popular_position`: homepage popular-section order slot.
- `search_results_per_page`: default HTML search page size.
- `default_new_note_is_private`: default visibility flag for freshly opened note pages; the product default is `FALSE` so new notes start public unless the setting is turned on.
- `session_timeout_minutes`: future-login session lifetime; the product default is `1440`.
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
