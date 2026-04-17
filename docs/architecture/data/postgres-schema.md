# Postgres Schema Contract

## `resources`

- `id`: `CHAR(26)` primary key.
- `kind`: `TEXT` constrained to `note` or `media`.
- `alias`: nullable unique live-resource alias shared across all kinds.
- `body`: current Markdown body.
- `title`: current derived title.
- `summary`: current derived summary.
- `media_family`: nullable `image` or `video`.
- `file_key`: nullable current object-storage key for media.
- `content_type`: nullable current media MIME type.
- `byte_size`: nullable current media byte length.
- `sha256_hex`: nullable current media checksum.
- `original_filename`: nullable current upload filename.
- `width`: nullable current intrinsic width.
- `height`: nullable current intrinsic height.
- `duration_ms`: nullable current video duration.
- `media_variants`: nullable current derivative metadata JSON.
- `owner_note_id`: nullable immutable live-note reference for media created from note attachment.
- `is_favorite`: favorite flag.
- `favorite_position`: nullable persistent favorite ordering slot.
- `is_private`: privacy flag.
- `view_count_total`: lifetime successful page views.
- `last_viewed_at`: last counted page-view timestamp.
- `created_at`: immutable UTC timestamp.
- `updated_at`: mutable UTC timestamp.
- `deleted_at`: nullable soft-delete timestamp.
- `search_document`: current full-text search column.

## `resource_snapshots`

- `id`: `CHAR(26)` primary key.
- `resource_id`: live-resource reference.
- `kind`: copied resource kind.
- `alias`: saved route alias.
- `title`: saved derived title.
- `summary`: saved derived summary.
- `body`: saved Markdown body.
- `media_family`: nullable saved media family.
- `file_key`: nullable immutable object-storage key.
- `content_type`: nullable saved media MIME type.
- `byte_size`: nullable saved media byte length.
- `sha256_hex`: nullable saved media checksum.
- `original_filename`: nullable saved upload filename.
- `width`: nullable saved intrinsic width.
- `height`: nullable saved intrinsic height.
- `duration_ms`: nullable saved video duration.
- `media_variants`: nullable saved derivative metadata JSON.
- `owner_note_id`: nullable copied owner-note reference for snapshot-stable image-link behavior.
- `is_private`: saved visibility.
- `snapshot_number`: immutable per-resource sequence.
- `created_at`: snapshot UTC timestamp.

## `resource_daily_views`

- `resource_id`: live-resource reference.
- `view_date`: UTC date bucket.
- `view_count`: counted resource-page views for that UTC day.
- Primary key: `(resource_id, view_date)`.

## `app_settings`

- `id`: singleton key fixed to `1`.
- `home_intro_markdown`: optional homepage hero Markdown.
- `home_recent_limit`, `home_favorite_limit`, `home_popular_limit`: mixed-resource section counts.
- `home_recent_visible`, `home_favorite_visible`, `home_popular_visible`: section visibility flags.
- `home_recent_position`, `home_favorite_position`, `home_popular_position`: section order slots.
- `search_results_per_page`: default HTML search page size.
- `default_new_resource_is_private`: default visibility for newly created notes and media.
- `session_timeout_minutes`: login session lifetime.
- `site_name`, `site_description`, `public_base_url`: shared site identity and discovery fields.
- `media_webp_quality`: quality value for future derivative WebP generation.
- `site_icon_key`, `site_icon_content_type`, and `site_icon_updated_at`: optional uploaded site icon state.
- `updated_at`: mutable UTC timestamp.

## `password_reset_tokens`

- `id`: UUID primary key.
- `token_hash`: hash of the one-time reset token.
- `expires_at`: token expiry timestamp.
- `used_at`: nullable consumption timestamp.
- `created_at`: token creation timestamp.
