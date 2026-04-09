# First Login and Live Use

## Create the Admin Account

1. Open `http://127.0.0.1:${APP_PORT:-8080}/setup`.
2. Create the first admin account.
3. Confirm redirect to `/login`.
4. Sign in and confirm redirect to `/admin`.

## Review Settings Before Publishing Real Resources

1. Open `/admin/settings`.
2. Set `Site name`, `Site description`, and `Public base URL`.
3. Set homepage intro, section visibility, section order, and item counts.
4. Set `Search page size`.
5. Set `New resources start private` based on deployment policy.
6. Set `Session timeout (minutes)`.

## Create the First Real Resources

1. Click `New note` and create one public note.
2. Click `New media` and upload one public image or video.
3. Embed the media file in the note through `/{ref}/file`.
4. Edit the note once and replace the media once.

## Verify Published Surface

1. Confirm `/` shows mixed-resource sections.
2. Confirm `/search?kind=all|note|media` behaves as expected.
3. Confirm the embedded media renders inside the note page.
4. Confirm media history exposes old file versions through snapshot URLs.
