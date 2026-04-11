# Settings and Preferences Behavior

## Global Settings

- `site_name`, `site_description`, and `public_base_url` remain global.
- `default_new_resource_is_private` controls both new note and new media defaults.
- `search_results_per_page` still controls the default `/search` page size.
- `media_webp_quality` controls future image WebP and video poster generation quality.
- Uploaded site icon metadata controls favicon and shell icon delivery.
- Home section visibility, ordering, and limits apply to mixed-resource sections.
- Fresh-install home section order is `Recently updated`, `Favorites`, then `Popular`.

## Workspace State

- `/admin/settings` is the canonical admin workspace for global settings plus favorite ordering.
- The page exposes one browser-local settings search that filters visible settings items without persisting anything.
- Reordering home sections changes only the in-form pending state until `Save settings`.
- Home-section ordering uses drag-and-drop only; explicit `Up` and `Down` controls are not part of the contract.
- Favorite ordering and site icon upload/reset persist immediately through their own admin-only requests.
- Dirty settings block same-origin shell navigation, browser back/forward, and full-page unload until the user explicitly discards the pending edits.
- Choosing to stay keeps both the visible page and the browser URL on `/admin/settings`.
- Async favorite reordering and async site icon upload/reset must not discard unsaved main-form edits.

## Immediate Effects

- Saving settings immediately affects `/`, `/search`, `/admin`, new note pages, new media pages, and discovery surfaces.
- Changing `default_new_resource_is_private` affects future creations only.
- Changing `media_webp_quality` affects future uploads only.
- Uploading or resetting the site icon affects subsequent HTML head and shell icon rendering immediately.
- Reordering favorites affects `/`, `/admin`, and `/search?scope=favorites` immediately.
- Changing `public_base_url` affects later canonical URLs, `robots.txt`, and `sitemap.xml` without restart.
