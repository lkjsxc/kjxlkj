# First Login and Live Use

## Create the Admin Account

1. Open `http://127.0.0.1:${APP_PORT:-8080}/setup`.
2. Read the one-time setup code from the app server console.
3. Create the first admin account with the setup code.
4. Confirm redirect to `/login`.
5. Sign in and confirm redirect to `/admin`.

## Review Settings Before Publishing Real Resources

1. Open `/admin/settings`.
2. Set `Site name`, `Site description`, and `Public base URL`.
3. Set homepage intro, section visibility, section order, and item counts.
4. Set `Search page size`.
5. Set `New resources start private` based on deployment policy.
6. Set `Media WebP quality`.
7. Set `Session timeout (minutes)`.
8. Upload or reset the site icon.

## Create the First Real Resources

1. Click `New note` and create one public note.
2. Use `Upload media` on that note and upload one public image or video.
3. Confirm the current note receives a direct embed for the new media.
4. Open the new media resource and confirm it has its own media page.

## Verify Published Surface

1. Confirm `/` shows mixed-resource sections.
2. Confirm `/search?kind=all|note|media` behaves as expected.
3. Confirm the embedded media renders inside the note page.
4. Confirm media history exposes stable file versions through snapshot URLs.
