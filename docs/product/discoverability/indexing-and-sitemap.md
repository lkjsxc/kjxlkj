# Indexing and Sitemap Contract

## Indexable Surface

- Only `GET /` and guest-readable current live note pages are search-indexable.
- `/search` is guest-readable but not search-indexable.
- Setup, login, admin, settings, history, saved snapshots, fragments, health, and write/API routes are never search-indexable.

## `robots.txt`

- `GET /robots.txt` exists only when `public_base_url` is non-blank and valid.
- `robots.txt` allows crawling of `/` and current public note routes.
- `robots.txt` disallows `/search`.
- `robots.txt` disallows `/setup`, `/login`, `/admin`, `/records`, `/_/`, and `/healthz`.
- `robots.txt` advertises the absolute sitemap URL.
- When `public_base_url` is blank or invalid, `GET /robots.txt` returns `404`.

## `sitemap.xml`

- `GET /sitemap.xml` exists only when `public_base_url` is non-blank and valid.
- The sitemap contains the homepage plus current public live note URLs only.
- The sitemap excludes search, setup, login, admin, settings, history, snapshots, fragments, health, and write/API routes.
- Sitemap URLs are absolute and use `public_base_url`.
- Sitemap entries for notes include `lastmod` from note `updated_at`.
- When `public_base_url` is blank or invalid, `GET /sitemap.xml` returns `404`.

## Safe Fallback

- Blank or invalid `public_base_url` must not produce guessed canonical URLs.
- Blank or invalid `public_base_url` must not produce request-derived sitemap URLs.
- Blank or invalid `public_base_url` places all HTML into safe `noindex,nofollow` mode.
- Blank or invalid `public_base_url` disables machine-readable discovery routes rather than emitting stale data.
