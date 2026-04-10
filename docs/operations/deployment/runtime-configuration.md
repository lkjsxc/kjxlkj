# Runtime Configuration Ownership

## Compose Environment

- `.env` owns host port exposure, bind address, PostgreSQL credentials, MinIO credentials, S3 endpoint settings, and optional `SETUP_CODE`.
- Compose assembles `DATABASE_URL` and S3-compatible environment for the runtime container.
- `docker-compose.yml` treats `.env` as authoritative rather than relying on inline fallback defaults.
- CI must create `.env` from `.env.example` before running compose-backed verification.
- Compose environment does not own site identity, search defaults, session timeout, home section ordering, or discovery public-origin state.

## Persisted Operator Settings

- `/admin/settings` owns `site_name`, `site_description`, and `public_base_url`.
- `/admin/settings` owns homepage intro, section visibility and order, section limits, search page size, default new-resource visibility, media WebP quality, site icon, and session timeout.

## Bootstrap Sequence

1. Start the compose stack with database, MinIO, and bind settings only.
2. Complete `/setup` and `/login`.
3. Open `/admin/settings`.
4. Save site identity, `public_base_url`, and default new-resource visibility.
5. Set media WebP quality and upload a site icon when desired.
6. Create at least one note and one media resource before handoff.

## Discovery Rule

- Blank `public_base_url` is the safe pre-launch state.
- Non-blank `public_base_url` enables canonical URLs, `robots.txt`, and `sitemap.xml`.
- The runtime must not guess public URLs from request headers when `public_base_url` is blank.
