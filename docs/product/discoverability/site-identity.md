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

## Browser Titles

- HTML pages use the `Page | site` title format.
- Live resource browser title is `{resource title} | {site_name}`.
- History browser title is `History: {resource title} | {site_name}`.
