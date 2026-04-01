# Settings and Preferences Behavior

## Global Settings

- Global app settings are stored in PostgreSQL.
- Admin dashboard owns the canonical settings form.
- Global settings include homepage recent-note count, homepage favorite count, homepage popular-note count, homepage intro Markdown, default search page size, and default Vim mode.
- Global settings affect HTML routes immediately after a successful save.

## Local Preferences

- Editor preference override remains browser-local.
- The browser-local Vim override has three states:
  - `default`
  - `on`
  - `off`
- Effective Vim mode uses the local override when set.
- Effective Vim mode falls back to the global default when the local override is `default`.
- Reloading the page preserves the local Vim override in that browser.

## Defaults

- Homepage recent-note count defaults to `6`.
- Homepage favorite count defaults to `6`.
- Homepage popular-note count defaults to `6`.
- Homepage intro Markdown defaults to empty.
- Default search page size defaults to `20`.
- Default Vim mode defaults to `false`.
