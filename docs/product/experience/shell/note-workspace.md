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
- Desktop uses an upstream-style text-plus-table toolbar inside the existing shell.
- Narrow screens reduce the toolbar to a fixed supported subset, but table authoring remains available.
- Toolbar rows wrap inside the editor chrome instead of introducing a detached toolbar scrollbar.
- Public checkbox stays inside the editing surface.
- Editor theming may skin the container, but must not flatten or suppress editor content semantics.
- The note page owns the vertical scroll path for long notes.
- The note page never requires horizontal page scrolling to reach editor controls.
- Opening the note should leave the caret ready for direct typing.

## Live Chrome Sync

- Heading edits update page title, browser title, and current-note rail title immediately.
- Public checkbox toggles update visibility chips and related chrome immediately.
- The removed helper text (`Guest-readable`, `Admin-only`) does not return.
- Markdown shortcut typing at the caret must transform into rendered heading, list, quote, and code-block structure without requiring a reload.
- Table insertion must stay available without leaving the single rendered workspace.

## Guest View

- Guests see rendered Markdown only.
- Markdown typography may use a dedicated content stack separate from UI chrome.
