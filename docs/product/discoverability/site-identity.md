# Site Identity Contract

## Canonical Terms

- `site_name` is the operator-configurable visible product name for HTML pages.
- `site_description` is the operator-configurable shared product description for metadata and discovery copy.

## Defaults

- Default `site_name`: `kjxlkj`.
- Default `site_description`: `Markdown-first resource system for LLM-operated workflows.`

## Visible Branding

- Guest and admin HTML shells render `site_name` in the visible brand lockup.
- Visible branding does not require renaming the repository or binary.
- Guest and admin HTML shells use the uploaded site icon when one is configured.
- The default authored icon remains the fallback when no uploaded icon is configured.

## Site Icon

- The site icon is operator-configurable from `/admin/settings`.
- Uploaded site icons must be image files.
- Uploaded site icons are stored in SeaweedFS-backed object storage.
- `/admin/settings` exposes an `Upload icon` trigger that opens the local file picker.
- `/admin/settings` also exposes `Reset icon` when an uploaded icon is configured.
- `GET /assets/site-icon` serves the configured uploaded icon or the fallback icon.
- `GET /favicon.ico` may redirect to or proxy the same effective icon.
- Resetting the icon removes the configured icon metadata and restores the fallback icon.

## Browser Titles

- HTML pages use the `Page | site` title format.
- Live resource browser title is `{resource title} | {site_name}`.
- History browser title is `History: {resource title} | {site_name}`.
