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

- `Site identity` contains `Site name`, `Site description`, and `Public base URL`.
- `Public base URL` may be blank to keep discovery disabled.
- `Home hero` contains only the editable intro Markdown field.
- `Home sections` contains visibility, draggable order, and item-count controls for `Popular`, `Recently updated`, and `Favorites`.
- The drag row order is the only visible order affordance; numeric order badges are omitted.
- `Sessions` contains the future-login timeout field in minutes.
- `New notes` contains the default visibility control used when opening a fresh note.
- `Search` contains the default result-count control for `/search`.

## Behavior

- Saving redirects back to `/admin/settings`.
- Successful saves immediately affect `/`, `/search`, `/admin`, newly opened note pages, and discovery surfaces.
- Successful saves affect future login session length without mutating active sessions.
- Fresh installs and untouched settings default new notes to public.
- The settings page contains no Vim-mode controls.
- The settings page uses factual validation copy and does not hide invalid values behind silent coercion.
- The settings page should describe blank `Public base URL` as an intentional discovery-off state.
