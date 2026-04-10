# Settings and Preferences Behavior

## Global Settings

- `site_name`, `site_description`, and `public_base_url` remain global.
- `default_new_resource_is_private` controls both new note and new media defaults.
- `search_results_per_page` still controls the default `/search` page size.
- `media_webp_quality` controls future image WebP and video poster generation quality.
- Uploaded site icon metadata controls favicon and shell icon delivery.
- Home section visibility, ordering, and limits apply to mixed-resource sections.

## Immediate Effects

- Saving settings immediately affects `/`, `/search`, `/admin`, new note pages, new media pages, and discovery surfaces.
- Changing `default_new_resource_is_private` affects future creations only.
- Changing `media_webp_quality` affects future uploads only.
- Uploading or resetting the site icon affects subsequent HTML head and shell icon rendering immediately.
- Changing `public_base_url` affects later canonical URLs, `robots.txt`, and `sitemap.xml` without restart.
