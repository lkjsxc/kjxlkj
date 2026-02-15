# Search and Backlinks

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Search Model

Search MUST use a hybrid retrieval pipeline:

1. lexical retrieval via PostgreSQL `tsvector` + GIN
2. semantic retrieval via embedding vectors
3. deterministic reranking over merged candidates

The system MUST support rebuilding from lexical-only mode and enabling semantic
mode by configuration.

## Indexing Rules

- Title and markdown body MUST be indexed for lexical retrieval.
- Each searchable note revision MUST have an embedding vector payload.
- Embeddings MUST be regenerated whenever title or body changes.
- Search/index updates MUST occur in the same commit cycle as accepted writes.
- Deleted notes MUST be removed from lexical and vector candidate sets.

## Query Contract

| Endpoint | Behavior |
|---|---|
| `GET /search` | hybrid retrieval across note title/body/metadata |
| `GET /notes/{id}/backlinks` | backlink list sorted by `updated_at desc` |

`GET /search` SHOULD accept:

- `q` (required)
- `workspace_id` (required)
- `project_id` (optional)
- `limit` (optional)
- `mode` (`hybrid`, `lexical`, `semantic`; default `hybrid`)

## Ranking Contract

Hybrid mode MUST:

1. fetch top lexical candidates
2. fetch top semantic candidates
3. deduplicate by note ID
4. compute deterministic combined score
5. return stable order for equal scores by `updated_at desc`, then `note_id`

## Backlinks

- Wiki links `[[target]]` MUST update backlink projections.
- Backlinks MUST converge within one committed write cycle.
- Backlink reads MUST be permission-filtered to caller scope.

## Safety and Scope Rules

- Search results MUST be filtered by workspace and authorization scope.
- Search MUST include `settings` and media-note metadata where relevant.
- Search MUST include librarian/agent-generated note updates.
- Search MUST NOT leak inaccessible note existence through score counts.

## Failure and Fallback Rules

- If embedding service is unavailable, lexical search MUST continue.
- Semantic failure MUST set machine-readable diagnostics in response metadata.
- Reindex jobs MUST be resumable and idempotent.

## Related

- Notes model: [notes.md](notes.md)
- API contract: [/docs/spec/api/http.md](/docs/spec/api/http.md)
- Performance: [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md)
