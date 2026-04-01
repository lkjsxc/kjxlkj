# Settings and Preferences Behavior

## Global Settings

- Global app settings are stored in PostgreSQL.
- `GET /settings` owns the canonical settings form.
- The admin dashboard links to settings but does not own the full form.
- Global settings include homepage intro Markdown, homepage section visibility, homepage section order, homepage recent-note count, homepage favorite count, homepage popular-note count, default search page size, default new-note visibility, and favorite ordering controls.
- Global settings affect HTML routes immediately after a successful save.

## Homepage Section Settings

- Homepage section settings apply only to `/`.
- `Popular notes`, `Recently updated`, and `Favorites` may each be shown or hidden.
- Those three sections have an explicit saved order.
- `Quick search` remains visible and stays ahead of the configurable note sections.
- Section counts are stored independently per note section.

## Defaults

- Homepage recent-note count defaults to `5`.
- Homepage favorite count defaults to `5`.
- Homepage popular-note count defaults to `5`.
- Homepage intro Markdown defaults to empty.
- Default search page size defaults to `20`.
- Default new-note visibility defaults to private.
