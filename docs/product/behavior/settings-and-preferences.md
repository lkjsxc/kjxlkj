# Settings and Preferences Behavior

## Global Settings

- `site_name`, `site_description`, and `public_base_url` remain global.
- `default_new_resource_is_private` controls both new note and new media defaults.
- `search_results_per_page` still controls the default `/search` page size.
- Home section visibility, ordering, and limits apply to mixed-resource sections.

## Immediate Effects

- Saving settings immediately affects `/`, `/search`, `/admin`, new note pages, new media pages, and discovery surfaces.
- Changing `default_new_resource_is_private` affects future creations only.
- Changing `public_base_url` affects later canonical URLs, `robots.txt`, and `sitemap.xml` without restart.
