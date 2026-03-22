# Direct Edit Mode Contract

## Mode Model

- Direct editing is inline on `/article/{slug}` for authenticated admin.
- No dedicated editor page exists.
- Canonical source remains Markdown; no HTML-first storage.

## Interaction Rules

- Inline editor fields include `title`, `private`, `body`, `last_known_revision`.
- Private toggle is positioned above body in edit form.
- Save and preview buttons are removed.
- Autosave runs after 2 seconds of inactivity and on blur.
- Before unload, dirty state triggers save attempt and warning.

## Visibility Rules

- Non-admin users never receive editing controls.
- Private articles are hidden from non-admin users.

## History Rules

- Every article exposes history at `/article/{slug}/history` for admin.
- Restore endpoint exists at `/article/{slug}/history/restore`.
- History backend is Git with at most one commit per article per 60 seconds.
