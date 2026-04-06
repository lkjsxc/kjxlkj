# First Login and Live Use

## Create the Admin Account

1. Open `http://127.0.0.1:${APP_PORT:-8080}/setup`.
2. Create the first admin account.
3. Confirm the browser redirects to `/login`.
4. Sign in and confirm the browser redirects to `/admin`.

## Review Settings Before Publishing Real Notes

1. Open `/admin/settings`.
2. Set `Site name` and `Site description`.
3. Set `Public base URL` to the final public origin once the deployment hostname is ready.
4. Set the homepage intro Markdown if the deployment needs a landing message.
5. Set homepage section visibility, order, and item counts.
6. Set `Search page size`.
7. Set `New notes start private` based on the deployment policy.
8. Set `Session timeout (minutes)` to the desired login lifetime.
9. Save and confirm the page redirects back to `/admin/settings`.

## Session Timeout Rule

- The saved timeout affects future logins only.
- Already-issued sessions keep their existing expiry.
- Allowed range is `5` through `10080` minutes.
- The untouched default is `1440` minutes.

## Public Origin Rule

- Blank `Public base URL` keeps the deployment out of search-indexable mode.
- A saved public origin should be the exact absolute origin that canonical URLs and the sitemap must advertise.

## Create the First Real Note

1. Click `New note`.
2. Confirm the note starts with the configured default visibility.
3. Add a heading and real Markdown body.
4. Set an alias if the note needs a stable human-readable URL.
5. Save the note through the normal editor flow.

## Verify the Published Surface

1. Open `/`.
2. Confirm the homepage hero and sections reflect the saved settings.
3. Open `/search` and confirm browse results and default page size match expectations.
4. Open the note by alias if present, otherwise by its opaque ID.
5. If the note has an alias, confirm visiting the raw current-note ID redirects to the alias URL.
6. If `Public base URL` is set, confirm `/robots.txt` and `/sitemap.xml` advertise the same origin.

## Verify Saved-Snapshot Behavior

1. Edit the note at least once.
2. Open `/{note_ref}/history`.
3. Confirm the history list is newest-first by `snapshot_number`.
4. Confirm the first pinned card is `Live note` and the first saved card is `Latest saved snapshot`.
5. Open one saved-snapshot card.
6. Confirm the saved-snapshot page uses its own opaque root-path URL like `/{id}`.

## Start Real Usage

- Use `/` for the public landing surface.
- Use `/search` for large-scale browse and query work.
- Use `/admin` for analytics, favorites, recent activity, and the settings entry point.
- Use `/admin/settings` for singleton app configuration.
