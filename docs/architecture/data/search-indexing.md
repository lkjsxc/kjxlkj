# Search Indexing Contract

## Derived Fields

- Current notes persist derived `title` and `summary` values.
- `title` comes from the first `# ` heading line.
- `summary` comes from the first non-heading non-empty line.
- Missing heading yields `Untitled note`.

## Search Document

- Current notes persist a full-text search document built from current alias, title, and body.
- Search indexing applies only to the current note state, not revisions.
- Search indexes must support public/admin list queries at thousands-note scale.
- Search may supplement full-text ranking with trigram-assisted fallback matching.
- Search queries may also request non-relevance ordering, favorite scope, and popularity ordering without changing the indexed fields.

## UI Visibility

- Opaque `id` values are not part of normal note cards, headers, or rails.
- Created time replaces visible ID chips as the secondary identity cue.
