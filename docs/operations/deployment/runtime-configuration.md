# Runtime Configuration Ownership

## Compose Environment

- `.env` owns host port exposure, bind address, live relay addresses, PostgreSQL credentials, SeaweedFS S3 settings, upload byte limits, and optional `SETUP_CODE`.
- Compose assembles `DATABASE_URL` and SeaweedFS S3 environment for the runtime container.
- `docker-compose.yml` treats `.env` as authoritative rather than relying on inline fallback defaults.
- CI must create `.env` from `.env.example` before running compose-backed verification.
- Compose environment does not own site identity, search defaults, session timeout, home section ordering, or discovery public-origin state.

## Persisted Operator Settings

- `/admin/settings` owns `site_name`, `site_description`, and `public_base_url`.
- `/admin/settings` owns homepage intro, section visibility and order, section limits, search page size, live capture defaults, default new-resource visibility, media WebP quality, favorite ordering, site icon, and session timeout.

## Bootstrap Sequence

1. Start the compose stack with database, SeaweedFS, upload limit, and bind settings only.
2. Complete `/setup` and `/login`.
3. Open `/admin/settings`.
4. Save site identity, `public_base_url`, and default new-resource visibility.
5. Set live capture defaults, media WebP quality, favorite ordering, and the site icon when desired.
6. Create at least one note and one media resource before handoff.

## Discovery Rule

- Blank `public_base_url` is the safe pre-launch state.
- Non-blank `public_base_url` enables canonical URLs, `robots.txt`, and `sitemap.xml`.
- The runtime must not guess public URLs from request headers when `public_base_url` is blank.

## Live Relay Address Rule

- `LIVE_ICE_PUBLIC_IPS` contains externally reachable relay addresses.
- `LIVE_ICE_LAN_IPS` contains LAN-reachable relay addresses.
- `LIVE_TRUSTED_PROXY_IPS` contains proxy peers whose forwarded client IP headers are trusted.
- Public URL discovery and live relay address selection are separate concerns.
