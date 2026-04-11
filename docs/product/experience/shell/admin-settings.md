# Admin Settings Contract

## Page Intent

- `GET /admin/settings` is the dedicated settings workspace.
- The page owns the canonical global settings form.
- The page also owns the canonical favorite-order management surface.
- The page is reachable from the dashboard and the admin rail.

## Form Groups

- `Site identity` contains `Site name`, `Site description`, and `Public base URL`.
- `Site icon` contains current-state, upload, and reset controls for the icon used across the site.
- `Home hero` contains the editable intro Markdown field.
- `Home sections` contains visibility, drag-only order, and item-count controls for mixed-resource sections.
- `Favorites` contains the reorderable mixed-resource favorite list.
- `Sessions` contains the future-login timeout field in minutes.
- `Search` contains the default result-count control for `/search`.
- `Media` contains the WebP quality value for future derivative generation.
- `New resources` contains the default visibility control used when opening a fresh note or media resource.
- `Password` contains the signed-in password change form.

## Behavior

- The page exposes one client-side settings search input that filters headings, labels, and helper text in place.
- Saving redirects back to `/admin/settings`.
- Uploading the site icon opens a local file picker from an `Upload icon` button rather than exposing the file input as the primary control.
- Uploading or resetting the site icon stays on `/admin/settings` and updates the visible icon state without a navigation.
- Reordering favorites stays on `/admin/settings` and persists immediately.
- Successful saves immediately affect `/`, `/search`, `/admin`, newly opened resource pages, and discovery surfaces.
- Fresh installs and untouched settings default new resources to public.
- Fresh installs and untouched settings order Home sections as `Recently updated`, `Favorites`, then `Popular`.
- Leaving the page with unsaved main-form or password fields prompts the user to stay.
- The prompt applies to same-origin shell links, browser back/forward, and full-page unload.
- Canceling the prompt keeps the visible page and browser URL on `/admin/settings`.
- The prompt clears for the submitted form after a successful submit.
