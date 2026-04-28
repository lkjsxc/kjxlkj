# Admin Settings Contract

## Page Intent

- `GET /admin/settings` is the dedicated settings workspace.
- The page owns the canonical global settings form.
- The page also owns the canonical favorite-order management surface.
- The page is reachable from the dashboard and the admin rail.

## Flat Settings List

- Settings render as one continuous list under the page title.
- Visible category headings and nested setting panels are omitted.
- Every setting row uses the same visual weight and feels parallel to neighboring rows.
- Ordinary scalar settings are not grouped into multi-field rows.
- Row labels use slash-path names such as `Site_identity/Site_name`.
- Home-section ordering, favorite ordering, Nostr JSON settings, live defaults, site icon controls, password change, and save actions are rows in the same list.
- Large controls such as Markdown textarea, favorite ordering, and password change may span the full list width without becoming separate groups.

## Behavior

- The page exposes one client-side settings search input that filters labels, helper text, and row content in place.
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
