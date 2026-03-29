# Settings and Preferences Behavior

## Global Settings

- Global app settings are stored in PostgreSQL.
- Admin dashboard owns the canonical settings form.
- Global settings include homepage recent-note count, homepage favorite count, and default search page size.
- Global settings affect HTML routes immediately after a successful save.

## Local Preferences

- Editor preferences remain browser-local.
- Vim mode is disabled by default.
- Admin dashboard exposes the Vim-mode toggle even though the stored value remains local to the browser.
- Reloading the page preserves the local Vim-mode preference in that browser.

## Defaults

- Homepage recent-note count defaults to `6`.
- Homepage favorite count defaults to `6`.
- Default search page size defaults to `20`.
