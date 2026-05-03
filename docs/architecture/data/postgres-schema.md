# Postgres Schema Contract

## Fresh Schema Rule

- The multi-user schema is fresh.
- The implementation does not preserve `admin_user`, `sessions`, or singleton `app_settings`.
- Operators who need old data must export and import manually.
- All tenant-owned rows use `space_id`.

## `users`

- `id`: UUID primary key.
- `email`: unique case-insensitive login email.
- `username`: unique case-insensitive personal-space slug.
- `display_name`: user-facing name.
- `status`: `active`, `invited`, or `disabled`.
- `created_at`, `updated_at`, and `last_login_at`: UTC timestamps.

## `user_local_credentials`

- `user_id`: primary key and user reference.
- `password_hash`: Argon2 password hash.
- `password_updated_at`: UTC timestamp.

## `spaces`

- `id`: UUID primary key.
- `slug`: unique case-insensitive top-level URL slug.
- `name`: user-facing space name.
- `owner_user_id`: canonical owner user.
- `created_at` and `updated_at`: UTC timestamps.

## `space_memberships`

- `space_id`: space reference.
- `user_id`: user reference.
- `role`: `owner`, `admin`, `editor`, or `viewer`.
- Primary key: `(space_id, user_id)`.

## `user_sessions`

- `id`: UUID primary key.
- `user_id`: signed-in user.
- `token_hash`: hash of the opaque session token.
- `csrf_secret_hash`: hash of the CSRF secret.
- `expires_at`, `last_seen_at`, and `revoked_at`: UTC timestamps.

## `service_accounts` and `api_tokens`

- `service_accounts` belong to one space.
- `api_tokens` belong to one service account and one space.
- `api_tokens.token_hash` stores only the bearer-token hash.
- `api_tokens.scopes` stores JSON scope strings such as `resource:read`.
- `api_tokens.expires_at`, `last_used_at`, and `revoked_at` control token lifetime.

## `resources`

- `id`: `CHAR(26)` primary key.
- `space_id`: required space reference.
- `kind`: `note` or `media`.
- `alias`: nullable live-resource alias unique inside one space.
- `body`, `title`, and `summary`: current Markdown and derived text.
- `visibility`: `public`, `space`, or `private`.
- `owner_user_id`: owner for private-resource checks.
- `created_by_user_id` and `updated_by_user_id`: browser actors when present.
- `created_by_service_account_id` and `updated_by_service_account_id`: service actors when present.
- Media fields are populated only when `kind = media`.
- Favorite, analytics, timestamps, soft delete, and search fields remain resource-local.

## `resource_snapshots`

- `id`: `CHAR(26)` primary key.
- `space_id`: required space reference.
- `resource_id`: live-resource reference.
- `kind`, `alias`, `title`, `summary`, `body`, and media fields are immutable copies.
- `visibility`: saved visibility.
- `snapshot_number`: immutable per-resource sequence.
- Actor columns store the user or service account that created the snapshot.
- `created_at`: snapshot UTC timestamp.

## `resource_daily_views`

- `space_id`: required space reference.
- `resource_id`: live-resource reference.
- `view_date`: UTC date bucket.
- `view_count`: counted resource-page views for that UTC day.
- Primary key: `(space_id, resource_id, view_date)`.

## `external_embed_cache`

- `space_id`: required space reference.
- `url_hash`: SHA-256 hex key for the normalized external URL.
- `url`, `canonical_url`, `provider`, and `kind`: normalized source metadata.
- Display fields are optional and never trusted as HTML.
- `fetched_at`, `expires_at`, `last_error`, and `error_at` control refresh.

## `space_settings`

- `space_id`: primary key and space reference.
- Homepage, search, site identity, Nostr, live defaults, WebP quality, and site icon fields are space-scoped.
- `default_new_resource_visibility` defaults to `public`.
- Session timeout, mailer, cookie security, and shared API keys are platform-owned.

## `password_reset_tokens`

- `id`: UUID primary key.
- `user_id`: user reference.
- `token_hash`: hash of the one-time reset token.
- `expires_at`, `used_at`, and `created_at`: UTC timestamps.

## `audit_events`

- `id`: UUID primary key.
- `space_id`: nullable space reference.
- `actor_user_id`: nullable browser actor.
- `actor_service_account_id`: nullable service actor.
- `event_type`: stable event key.
- `entity_type` and `entity_id`: affected object identity.
- `payload`: JSON detail.
- `created_at`: UTC timestamp.
