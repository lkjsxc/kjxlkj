# Search and Backlinks

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Full-Text Search

- Search MUST use PostgreSQL `tsvector` with GIN index.
- Projection updates MUST refresh indexed vector deterministically.
- Search MUST include title and markdown body content.
- Search SHOULD provide ranked results for best-match ordering.
- Search MUST include eligible `settings` notes and media-note metadata in query scope.

## Backlinks

- Wiki links parsed from markdown MUST update backlink projection table.
- Backlink reads MUST be eventually consistent within one committed write cycle.

## Query Rules

| Endpoint | Behavior |
|---|---|
| `GET /search` | query notes by text, tags, metadata filters |
| `GET /notes/{id}/backlinks` | return note references sorted by updated timestamp desc |

## Related

- Notes model: [notes.md](notes.md)
- API contract: [/docs/spec/api/http.md](/docs/spec/api/http.md)
