# Note Workspace Contract

## Overall Feel

- Note pages remain dark, dense, flat, and document-first.
- Header chrome is compact and informative.
- History access belongs in the rail, not below the note body.
- The rail history affordance is one `All history` card, not an inline revision list.

## Header Content

- Mode eyebrow or context marker.
- Title extracted from the note body.
- Visibility state.
- Created and updated time.
- No visible raw ID chips in normal UI.

## Admin Editing Surface

- Admin editing uses one rendered Markdown workspace.
- The workspace is powered by a vendored Toast UI Editor build.
- There is no mode-switch choice in normal UI.
- The editing engine stays close to official Toast UI WYSIWYG behavior.
- Desktop uses the upstream default toolbar.
- Narrow screens reduce the toolbar to a fixed supported subset before page overflow is allowed.
- Public checkbox stays inside the editing surface.
- Editor theming may skin the container, but must not flatten or suppress editor content semantics.
- The note page never requires horizontal page scrolling to reach editor controls.

## Live Chrome Sync

- Heading edits update page title, browser title, and current-note rail title immediately.
- Public checkbox toggles update visibility chips and related chrome immediately.
- The removed helper text (`Guest-readable`, `Admin-only`) does not return.

## Guest View

- Guests see rendered Markdown only.
- Markdown typography may use a dedicated content stack separate from UI chrome.
