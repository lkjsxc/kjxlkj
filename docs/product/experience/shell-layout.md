# Shell Layout Contract

## Design Direction

- The shell uses a warm light workspace, dark ink text, and a single earthy accent.
- Typography avoids generic system-default presentation.
- Headings use an editorial sans stack.
- body copy uses a readable serif stack.
- Code and slugs use a monospace stack.

## Shared Page Structure

- All note-centric pages use a two-region shell: navigation rail plus content pane.
- Desktop keeps the navigation rail visible.
- Narrow screens hide the rail by default and expose it from a top-right menu button.
- The rail always identifies the current mode: guest or admin.

## Navigation Rail Sections

- Brand/home link.
- Context block for the current note when a note is open.
- Previous and next note links when an adjacent accessible note exists.
- Revision history links for the current note.
- Mode actions.

## Guest Mode Actions

- Home link.
- Login link.
- No write controls.

## Admin Mode Actions

- Dashboard link.
- New note action.
- Public visibility control.
- Delete action on note pages.
- Logout action.

## Accessibility Baseline

- All interactive controls have visible labels.
- Focus states remain visible on light and dark surfaces.
- The menu drawer is keyboard dismissible.
- Page chrome remains usable without client-side JavaScript.
