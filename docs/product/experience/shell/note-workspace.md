# Resource Workspace Contract

## Overall Feel

- Resource pages remain dark, dense, flat, and document-first.
- Header chrome is compact and informative.
- The resource header does not show a duplicated visible title outside the main body or media surface.

## Shared Header Content

- Created and updated time.
- No visible raw ID chips in normal UI.
- Browser title, rail title, and other chrome may derive from the first `# ` heading in `body`.

## Admin Editing Surface

- Notes use one Markdown-first workspace.
- Media uses a file-first workspace with the same Markdown body editor and preview companion.
- Desktop uses a body-editor-first workspace with preview closed by default.
- Media preview surfaces do not show `Current file` copy above the primary image or video.
- Live note pages place `Upload media` beside `Show preview`.
- Public checkbox, alias, and favorite controls stay inside the editing surface.
- Opening the resource should leave the caret ready for direct typing.
- Media pages do not expose binary replacement controls.

## Note Page Layout

- Live note pages move `Prev`, `History`, and `Next` into one horizontal compact-card row near the top of the main pane.
- Live note pages place current note metadata in the main pane rather than a `Live note` rail card.
- Live note pages do not show `LIVE NOTE` copy in the rail or main pane.
- Live note pages do not show alias inside the rail.
- Live note pages do not place history below the note body.

## Guest View

- Guests see rendered Markdown only for notes.
- Guests see the primary image or video plus rendered Markdown body for media.
- Image media pages prefer a derived display image when the original binary is not browser-friendly.
- Markdown typography may use a dedicated content stack separate from UI chrome.
