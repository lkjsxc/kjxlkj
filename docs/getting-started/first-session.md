# First Session

## Create the First Admin

1. Open `http://127.0.0.1:${APP_PORT:-8080}/setup`.
2. Copy the one-time setup code from the app server console.
3. Create the first admin account with the setup code.
4. Confirm redirect to `/login`.
5. Sign in and confirm redirect to `/admin`.

## Review Settings Before Real Use

1. Open `/admin/settings`.
2. Set `Site name`, `Site description`, and `Public base URL` when ready.
3. Set homepage intro, section visibility, section order, and item counts.
4. Set `Search page size`.
5. Set `Media WebP quality`.
6. Set `New resources start private` if the install should default new notes and media to private.
7. Set `Session timeout (minutes)`.

## Create the First Real Resources

1. Click `New note`.
2. Add a heading and real Markdown body.
3. Set an alias if the note needs a stable human-readable URL.
4. Click `Upload media`.
5. Select one image or video and confirm the current note receives a direct media embed.
6. Open the new media resource and confirm it has its own media page.

## Verify Live Surfaces

1. Open `/` and confirm the homepage reflects saved settings.
2. Open `/search?kind=all`, `/search?kind=note`, and `/search?kind=media`.
3. Open the note body and confirm inline media renders when referenced through `/{ref}/file`.
4. As admin, edit a note or media body, open `/{ref}/history`, and confirm snapshots render newest first.
