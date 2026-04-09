# Indexing and Sitemap Contract

## Indexable Surface

- Only `GET /` and guest-readable current live resource pages are search-indexable.
- `/search` is guest-readable but not search-indexable.
- Setup, login, admin, settings, history, saved snapshots, fragments, health, write/API routes, and file routes are never search-indexable.

## `robots.txt`

- `GET /robots.txt` exists only when `public_base_url` is non-blank and valid.
- `robots.txt` allows crawling of `/` and current public resource routes.
- `robots.txt` disallows `/search`, `/setup`, `/login`, `/admin`, `/resources`, `/_/`, and `/healthz`.

## `sitemap.xml`

- `GET /sitemap.xml` exists only when `public_base_url` is non-blank and valid.
- The sitemap contains the homepage plus current public live resource URLs only.
- The sitemap excludes search, setup, login, admin, settings, history, snapshots, fragments, health, file routes, and write/API routes.
