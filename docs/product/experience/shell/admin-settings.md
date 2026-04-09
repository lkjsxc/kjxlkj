# Admin Settings Contract

## Page Intent

- `GET /admin/settings` is the dedicated settings workspace.
- The page owns the canonical global settings form.
- The page is reachable from the dashboard and the admin rail.

## Form Groups

- `Site identity` contains `Site name`, `Site description`, and `Public base URL`.
- `Home hero` contains the editable intro Markdown field.
- `Home sections` contains visibility, draggable order, and item-count controls for mixed-resource sections.
- `Sessions` contains the future-login timeout field in minutes.
- `New resources` contains the default visibility control used when opening a fresh note or media resource.
- `Search` contains the default result-count control for `/search`.

## Behavior

- Saving redirects back to `/admin/settings`.
- Successful saves immediately affect `/`, `/search`, `/admin`, newly opened resource pages, and discovery surfaces.
- Fresh installs and untouched settings default new resources to public.
