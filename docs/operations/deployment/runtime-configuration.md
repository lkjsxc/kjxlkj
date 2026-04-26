# Runtime Configuration Ownership

## Compose Defaults

- Compose owns local development and verification defaults.
- Compose does not require `.env`.
- Local app URL is `http://localhost:8080`.
- Local PostgreSQL credentials are literal compose-only values.
- Local SeaweedFS S3 settings are literal compose-only values.
- Upload limits are literal compose defaults.
- Verification uses deterministic `SETUP_CODE=visual-setup-code`.

## Direct App Environment

- Direct non-compose runs may set `BIND_HOST` and `BIND_PORT`.
- Direct non-compose runs must set `DATABASE_URL`.
- Direct non-compose runs must set required SeaweedFS S3 variables.
- Direct non-compose runs may set upload byte limits and `SETUP_CODE`.
- Direct app environment does not configure public URLs or live ICE servers.

## Persisted Operator Settings

- `/admin/settings` owns `site_name`, `site_description`, and `public_base_url`.
- `/admin/settings` owns homepage intro, section visibility and order, section limits, search page size, default new-resource visibility, media WebP quality, favorite ordering, site icon, and session timeout.
- `/admin/settings` owns `Live/ICE_servers_JSON`, which controls browser WebRTC ICE server configuration.
- Persisted settings must not require process restart.

## Bootstrap Sequence

1. Start the compose stack with database, object storage, upload limits, bind address, and setup code.
2. Complete `/setup` and `/login`.
3. Open `/admin/settings`.
4. Save site identity, `public_base_url`, and default new-resource visibility.
5. Set media WebP quality, favorite ordering, site icon, and live ICE servers when desired.
6. Create at least one note and one media resource before handoff.

## Discovery Rule

- Blank `public_base_url` is the safe pre-launch state.
- Non-blank `public_base_url` enables canonical URLs, `robots.txt`, and `sitemap.xml`.
- The runtime must not guess public URLs from request headers when `public_base_url` is blank.
