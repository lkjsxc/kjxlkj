# Note Workspace Contract

## Overall Feel

- Note pages remain dark, dense, flat, and document-first.
- Header chrome is compact and informative.
- History access belongs in the rail, not below the note body.
- The rail history affordance is one `All history` card, not an inline revision list.
- The note header does not show an eyebrow label such as `Admin note`.

## Header Content

- Title extracted from the note body.
- Created and updated time.
- No visible raw ID chips in normal UI.
- The header does not place a `Public` or `Private` pill between the title and editor.

## Admin Editing Surface

- Admin editing uses one Markdown-first workspace.
- The workspace uses a first-party textarea plus a rendered preview companion.
- There is no mode-switch choice in normal UI.
- Desktop uses a Markdown-first workspace with preview closed by default.
- Public checkbox stays inside the editing surface.
- Alias and favorite controls stay inside the editing surface.
- Preview stays closed by default and opens on demand from the editor chrome.
- Preview work may stay inactive until the user opens it.
- The typing surface stays dark, but preview content may switch to a light paper-like surface for readability.
- The note page owns the vertical scroll path for long notes.
- The note page never requires horizontal page scrolling to reach editor controls.
- Opening the note should leave the caret ready for direct typing.

## Live Chrome Sync

- Heading edits update page title, browser title, and current-note rail title immediately.
- Public checkbox toggles update visibility chips and related chrome immediately.
- Alias edits update canonical-link targets after save.
- Favorite toggles update relevant chrome immediately.
- The removed helper text (`Guest-readable`, `Admin-only`) does not return.
- Preview content updates from the current unsaved Markdown body.

## Guest View

- Guests see rendered Markdown only.
- Markdown typography may use a dedicated content stack separate from UI chrome.
