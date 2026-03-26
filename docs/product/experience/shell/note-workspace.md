# Note Workspace Contract

## Overall Feel

- Note pages remain dark, dense, and document-first.
- Header chrome is compact and informative.
- History access belongs in the rail, not below the note body.

## Header Content

- Mode eyebrow or context marker.
- Title extracted from the note body.
- Visibility state.
- Created and updated time.
- No visible raw ID chips in normal UI.

## Admin Editing Surface

- Admin editing uses a plain Markdown editor surface.
- The formatting toolbar is not visible.
- The editor uses the default UI font stack, not a separate branded UI font.
- Public checkbox stays inside the editing surface.

## Live Chrome Sync

- Heading edits update page title, browser title, and current-note rail title immediately.
- Public checkbox toggles update visibility chips and related chrome immediately.
- The removed helper text (`Guest-readable`, `Admin-only`) does not return.

## Guest View

- Guests see rendered Markdown only.
- Markdown typography may use a dedicated content stack separate from UI chrome.
