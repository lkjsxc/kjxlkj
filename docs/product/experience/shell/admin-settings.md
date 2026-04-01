# Admin Settings Contract

## Page Intent

- `GET /admin/settings` is the dedicated settings workspace.
- The page owns the canonical global settings form.
- The page is reachable from the dashboard and the admin rail.

## Layout

- The persistent shell rail remains visible.
- The page header stays concise and includes a clear return path to `/admin`.
- Settings groups render as short stacked sections rather than one giant form wall.
- Form density stays compact, but field groups use enough spacing to remain scannable.

## Form Groups

- `Home hero` contains the editable home title and intro Markdown.
- `Home sections` contains visibility, order, and item-count controls for `Popular notes`, `Recently updated`, and `Favorites`.
- `New notes` contains the default visibility control used when opening a fresh note.
- `Search` contains the default result-count control for `/search`.

## Behavior

- Saving redirects back to `/admin/settings`.
- Successful saves immediately affect `/`, `/search`, `/admin`, and newly opened note pages.
- The settings page contains no Vim-mode controls.
- The settings page uses factual validation copy and does not hide invalid values behind silent coercion.
