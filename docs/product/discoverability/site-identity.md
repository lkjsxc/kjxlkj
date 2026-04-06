# Site Identity Contract

## Canonical Terms

- `site_name` is the operator-configurable visible product name for HTML pages.
- `site_description` is the operator-configurable shared product description for metadata and discovery copy.
- `PUBLIC_BASE_URL` is the deploy-time absolute public origin used for canonical URLs, sitemap entries, and `robots.txt` sitemap advertisement.

## Defaults

- Default `site_name`: `kjxlkj`.
- Default `site_description`: `Markdown note system for LLM-operated workflows.`
- `PUBLIC_BASE_URL` has no default value.

## Visible Branding

- Guest and admin HTML shells render `site_name` in the visible brand lockup.
- The repository name, binary name, package name, and internal module names remain `kjxlkj`.
- Visible product branding does not require code or docs path renames.

## Browser Titles

- HTML pages use the `Page | site` title format.
- Homepage browser title is `Home | {site_name}`.
- Search browser title is `Search | {site_name}`.
- Admin dashboard browser title is `Dashboard | {site_name}`.
- Admin settings browser title is `Settings | {site_name}`.
- Live note browser title is `{note title} | {site_name}`.
- History browser title is `History: {note title} | {site_name}`.
- Saved snapshot browser title is `Saved snapshot {n}: {note title} | {site_name}`.
- Setup, login, and not-found pages also append `| {site_name}`.

## Settings Ownership

- `site_name` and `site_description` are stored in `app_settings`.
- `GET /admin/settings` owns the canonical edit surface for both fields.
- Saving either field updates subsequent HTML responses immediately.
