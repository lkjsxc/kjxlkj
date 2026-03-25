# Note Page Contract

## Page Scope

- `GET /{slug}` renders the current note state.
- Guests see only public notes.
- Admins can view and edit public or private notes.

## Header

- Title extracted from the first `# ` heading, else slug.
- Top-right menu button on narrow screens.
- Metadata row includes slug, created timestamp, and updated timestamp.

## Sidebar Requirements

- Show the current note title and slug.
- Show previous note based on the nearest older accessible note by `created_at`.
- Show next note based on the nearest newer accessible note by `created_at`.
- Show a history section with a link to the current version and visible past revisions.

## Content Pane

- Guests get rendered Markdown only.
- Admins get the editor plus rendered context chrome.
- Empty notes still render a stable shell.

## Visibility Control

- Admin note pages expose a checkbox labeled `Public`.
- Checked means the note is guest-readable.
- Unchecked means admin-only.
- Visibility changes save immediately.

## Footer

- Save status for admin sessions.
- Links back to note history and neighboring notes remain duplicated at the bottom on long pages.
