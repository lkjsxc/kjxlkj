# Public Origin Contract

## Canonical Term

- `public_base_url` is the persisted absolute public origin used for canonical URLs, sitemap entries, and `robots.txt` sitemap advertisement.
- The older deploy-time `PUBLIC_BASE_URL` concept is no longer the canonical runtime source of truth.

## Ownership

- `public_base_url` is stored in `app_settings`.
- `GET /admin/settings` owns the canonical edit surface for `public_base_url`.
- Saving `public_base_url` updates subsequent HTML metadata, `robots.txt`, and `sitemap.xml` responses immediately.

## Allowed States

- Blank `public_base_url` is valid and means discovery is intentionally disabled.
- Non-blank `public_base_url` must be an absolute `http` or `https` origin.
- Non-blank `public_base_url` must not contain a path, query string, fragment, username, or password.
- Stored values should be normalized to an origin without a trailing slash.

## Effective Discovery Mode

- A non-blank valid `public_base_url` enables canonical URLs, machine-readable discovery routes, and indexable HTML mode for allowed pages.
- A blank or invalid `public_base_url` places the app in safe discovery-off mode.
- Safe discovery-off mode uses `noindex,nofollow`, omits canonical links, and returns `404` for `robots.txt` and `sitemap.xml`.

## Operator Guidance

- Operators may leave `public_base_url` blank during private setup, staging, or hostname cutover work.
- Operators should set `public_base_url` before publishing public notes to search engines.
- The settings page should explain that blank disables indexing rather than silently guessing public URLs.
