# Search Page Contract

This contract defines the dedicated search surface.

## Route

- `GET /search` renders the dedicated search page.
- `GET /search` is available to both non-admin users and admins after setup completion.

## Page Contract

- Root container: `<main id="search-page">`.
- Search form ID: `#search-form`.
- Query input ID: `#search-query`.
- Results container ID: `#search-results`.

## Query Behavior

- Empty query returns a deterministic empty-state result block.
- Non-empty query returns ranked matches.
- Ranking source is full-text search index contract from architecture data docs.

## Result Shape

Each result row MUST include:

- canonical article link `/article/{slug}`
- slug
- optional title
- text snippet around matches

## Privacy and Role Filtering

- Non-admin users MUST receive only public matches.
- Admin users MAY receive public and private matches.
- Private-match labeling MAY be shown to admins.

## Menu Integration

- Navigation shell MUST include a stable entry to `/search`.
- Search page MUST render within shared navigation shell contract.

## Cross-References

- Shared shell behavior: [navigation-shell.md](navigation-shell.md)
- Public/article visibility rules: [public-site.md](public-site.md)
- Privacy policy: [../policies/privacy.md](../policies/privacy.md)
