# Settings and Preferences Behavior

## Global Settings

- Global app settings are stored in PostgreSQL.
- `GET /admin/settings` owns the canonical settings form.
- The dashboard links to the settings page, but does not replace it.
- Global settings include `home_intro_markdown`.
- Global settings include homepage recent-note, favorite, and popular limits.
- Global settings include homepage recent, favorite, and popular visibility toggles.
- Global settings include homepage recent, favorite, and popular order positions.
- Global settings include default HTML search page size.
- Global settings include default new-note visibility.
- Global settings include session timeout in minutes.
- Global settings affect HTML routes immediately after a successful save.
- Session timeout affects future logins only.

## Local UI State

- Preview open or closed state is ephemeral per page load.
- Drawer open or closed state is ephemeral per viewport session.
- No Vim-mode preference or browser-local Vim override remains anywhere in the product.

## Defaults

- Homepage recent-note count defaults to `5`.
- Homepage favorite count defaults to `5`.
- Homepage popular-note count defaults to `5`.
- Homepage section visibility defaults to all three sections enabled.
- Homepage section order defaults to `Popular notes`, `Recently updated`, then `Favorites`.
- Homepage intro Markdown defaults to empty.
- Default search page size defaults to `20`.
- Default new-note visibility defaults to public.
- Session timeout defaults to `1440` minutes.
