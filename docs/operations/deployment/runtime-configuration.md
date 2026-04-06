# Runtime Configuration Ownership

## Compose Environment

- `.env` owns host port exposure, bind address, and PostgreSQL credentials.
- Compose assembles `DATABASE_URL` for the runtime container.
- Compose environment does not own site identity, search defaults, session timeout, or discovery public-origin state.

## Persisted Operator Settings

- `/admin/settings` owns `site_name`.
- `/admin/settings` owns `site_description`.
- `/admin/settings` owns `public_base_url`.
- `/admin/settings` owns homepage intro, homepage section visibility and order, homepage section limits, search page size, default new-note visibility, and session timeout.

## Bootstrap Sequence

1. Start the compose stack with database and bind settings only.
2. Complete `/setup` and `/login`.
3. Open `/admin/settings`.
4. Save site identity and `public_base_url`.
5. Publish public notes only after the saved public origin is correct.

## Discovery Rule

- Blank `public_base_url` is the safe pre-launch state.
- Non-blank `public_base_url` enables canonical URLs, `robots.txt`, and `sitemap.xml`.
- The runtime must not guess public URLs from request headers when `public_base_url` is blank.
