# Indexing and Sitemap Contract

## Indexable Surface

- Only `GET /` and guest-readable current live note pages are search-indexable.
- `/search` is guest-readable but not search-indexable.
- Setup, login, admin, settings, history, saved snapshots, fragments, health, and write/API routes are never search-indexable.

## `robots.txt`

- `GET /robots.txt` exists only when `PUBLIC_BASE_URL` is configured and valid.
- `robots.txt` allows crawling of `/` and current public note routes.
- `robots.txt` disallows `/search`.
- `robots.txt` disallows `/setup`, `/login`, `/admin`, `/records`, `/_/`, and `/healthz`.
- `robots.txt` advertises the absolute sitemap URL.
- When `PUBLIC_BASE_URL` is missing or invalid, `GET /robots.txt` returns `404`.

## `sitemap.xml`

- `GET /sitemap.xml` exists only when `PUBLIC_BASE_URL` is configured and valid.
- The sitemap contains the homepage plus current public live note URLs only.
- The sitemap excludes search, setup, login, admin, settings, history, snapshots, fragments, health, and write/API routes.
- Sitemap URLs are absolute and use `PUBLIC_BASE_URL`.
- Sitemap entries for notes include `lastmod` from note `updated_at`.
- When `PUBLIC_BASE_URL` is missing or invalid, `GET /sitemap.xml` returns `404`.

## Safe Fallback

- Missing or invalid `PUBLIC_BASE_URL` must not produce guessed canonical URLs.
- Missing or invalid `PUBLIC_BASE_URL` must not produce request-derived sitemap URLs.
- Missing or invalid `PUBLIC_BASE_URL` places all HTML into safe `noindex,nofollow` mode.
- Missing or invalid `PUBLIC_BASE_URL` disables machine-readable discovery routes rather than emitting stale data.
