# Resource Workspace Contract

## Overall Feel

- Resource pages remain dark, dense, flat, and document-first.
- Header chrome is compact and informative.
- The resource header does not show a duplicated visible title outside the main body or media surface.

## Shared Header Content

- Created and updated time.
- The top metadata row also carries a compact semantic pill stack immediately after `Updated`.
- The pill stack must make resource kind, favorite state, and visibility obvious without opening editor controls.
- Canonical pill order is: kind first, favorite second when active, visibility last.
- Kind pills use the user-facing resource-family language: `Note`, `Image`, `Video`, or `File`.
- Favorite state uses a dedicated pill such as `Favorite`; it is omitted when inactive.
- Visibility remains the final pill and uses `Public` or `Private`.
- No visible raw ID chips in normal UI.
- Browser title, rail title, and other chrome may derive from the first `# ` heading in `body`.

## Admin Editing Surface

- Notes use one Markdown-first workspace.
- Media uses the same live-resource shell with a media/file surface plus the same Markdown body editor and preview companion.
- Desktop uses a body-editor-first workspace with preview closed by default.
- Media preview surfaces do not show `Current file` copy above the primary image or video.
- Live note pages place `Upload media` beside `Show preview`.
- Public checkbox, alias, and favorite controls stay inside the editing surface.
- Opening the resource should leave the caret ready for direct typing.
- Media pages do not expose binary replacement controls.

## Live Resource Page Layout

- Live note pages and live media pages share the same top-of-main-pane shell.
- Live-resource metadata stays in the main pane rather than a `Live note` or current-resource rail card.
- Live-resource pages do not show `LIVE NOTE` copy in the rail or main pane.
- Live-resource pages do not show a second summary, URL card, or other leftover current-resource card below the metadata row.
- Live-resource pages do not show alias inside the rail.
- Live-resource pages do not place history below the body/editor surface.
- The top metadata row and the live-resource timeline strip align to the same readable content column used by the Markdown surface or editor surface.
- Live-resource chrome must not become wider than the primary note/media body column.
- Admin live-resource pages render `Prev`, `History`, and `Next` in one horizontal compact-card row near the top of the main pane.
- Guest live-resource pages render only `Prev` and `Next` in that row.
- Admin rows use three equal columns; guest rows use two equal columns.
- Visible navigation cards keep matched widths and a matched minimum height so one longer title or summary does not visually enlarge only one card.
- The row remains visibly balanced when one or two targets are unavailable.
- Timeline cards clamp title and summary text instead of letting content change column width.
- `Prev` and `Next` do not show `Created` metadata.
- Live media pages place the current image, video, or file metadata surface below the shared top-row strip and live-resource metadata strip.

## Guest View

- Guests see rendered Markdown only for notes.
- Guests see the primary image, video, or file metadata surface plus rendered Markdown body for media.
- Image media pages prefer a derived display image when the original binary is not browser-friendly.
- Markdown typography may use a dedicated content stack separate from UI chrome.
