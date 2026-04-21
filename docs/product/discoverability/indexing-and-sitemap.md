# Indexing and Sitemap Contract

## Indexable Surface

- Only `GET /` and guest-readable current live resource pages are search-indexable.
- `/search` is guest-readable but not search-indexable.
- `/live` is guest-readable but not search-indexable.
- Setup, login, admin, settings, history, saved snapshots, fragments, health, write/API routes, discovery JSON routes, and file routes are never search-indexable.

## `robots.txt`

- `GET /robots.txt` exists only when `public_base_url` is non-blank and valid.
- `robots.txt` allows crawling of `/` and current public resource routes.
- `robots.txt` disallows `/search`, `/live`, `/setup`, `/login`, `/admin`, `/resources`, `/_/`, `/.well-known/`, and `/healthz`.

## `sitemap.xml`

- `GET /sitemap.xml` exists only when `public_base_url` is non-blank and valid.
- The sitemap contains `/` plus every current public live note and media page.
- The sitemap excludes search, setup, login, admin, settings, history, snapshots, fragments, health, file routes, and write/API routes.
