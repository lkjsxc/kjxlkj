# First Session

## Create the First Admin

1. Open `http://127.0.0.1:${APP_PORT:-8080}/setup`.
2. Create the first admin account.
3. Confirm redirect to `/login`.
4. Sign in and confirm redirect to `/admin`.

## Review Settings Before Real Use

1. Open `/admin/settings`.
2. Set `Site name` and `Site description`.
3. Set `Public base URL` if the deployment already has its final public origin.
4. Set homepage intro Markdown if needed.
5. Set homepage section visibility, order, and item counts.
6. Set `Search page size`.
7. Set `New notes start private`.
8. Set `Session timeout (minutes)`.
9. Save and confirm the page returns to `/admin/settings`.

## Session Timeout Rules

- Allowed range: `5` through `10080` minutes
- Untouched default: `1440` minutes
- New value affects future logins only

## Public Origin Rule

- Blank `Public base URL` keeps canonical URLs, `robots.txt`, and `sitemap.xml` disabled.
- Set it once the deployment has the final public origin that crawlers should see.

## Create the First Real Note

1. Click `New note`.
2. Confirm the note starts with the configured default visibility.
3. Add a heading and real Markdown body.
4. Set an alias if the note needs a stable human-readable URL.
5. Save through the normal editor flow.

## Verify Live Surfaces

1. Open `/` and confirm the homepage reflects saved settings.
2. Open `/search` and confirm browse results and default page size.
3. Open the note by alias if present, otherwise by its opaque ID.
4. If the note has an alias, confirm the raw current-note ID redirects to the alias URL.
5. Edit the note once, open `/{note_ref}/history`, and confirm saved snapshots render newest first.
6. If `Public base URL` is set, confirm `/robots.txt` and `/sitemap.xml` respond.

## Next Step

- Use [verification.md](verification.md) for full acceptance checks.
- Use [../operations/deployment/first-login-and-live-use.md](../operations/deployment/first-login-and-live-use.md) for the deeper operator runbook.
