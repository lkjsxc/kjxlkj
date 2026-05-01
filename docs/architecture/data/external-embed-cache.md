# External Embed Cache

## Table

- `external_embed_cache` stores normalized metadata for absolute external URLs.
- `url_hash` is the SHA-256 hex primary key of the normalized source URL.
- `url` is the normalized source URL used for fetching and rendering lookup.
- `canonical_url` is the metadata canonical URL when present.
- `provider` is the normalized host or known provider label.
- `kind` is `bookmark`, `image`, `video`, `audio`, `frame`, or `social`.
- `title`, `description`, `site_name`, and `author_name` are optional display fields.
- `thumbnail_url`, `thumbnail_width`, and `thumbnail_height` are optional direct remote preview-image fields.
- `fetched_at` records the last successful metadata fetch.
- `expires_at` records when metadata should be refreshed by an admin-triggered path.
- `last_error` records the latest fetch or parse failure summary.
- `error_at` records the latest failure time.
- `created_at` and `updated_at` are UTC timestamps.

## Refresh Rules

- Admin preview may insert or refresh rows for URLs in the preview body.
- Resource create and update may insert or refresh rows for URLs in the saved body.
- Public resource rendering reads rows only.
- Successful fetches use a multi-day expiry.
- Failed fetches use a short retry window.
- Deleting or editing a note does not delete cache rows.

## Lookup Rules

- Markdown URL normalization must be identical for refresh and render lookup.
- Query strings stay part of the cache key.
- Fragments are removed from the cache key.
- Unsupported schemes never enter the cache.
- Local root-relative URLs never enter this table.
