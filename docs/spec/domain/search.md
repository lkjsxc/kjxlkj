# Search and Backlinks — Redesigned

**Back:** [Domain Root](/docs/spec/domain/README.md)

---

## Search Model — Next-Generation Hybrid Pipeline

Search MUST use a **multi-stage retrieval pipeline** with modern vectorization:

### Stage 1: Query Understanding
1. **Query normalization** — lowercase, Unicode NFC, strip diacritics
2. **Intent classification** — navigational vs informational vs transactional
3. **Query expansion** — synonym injection via configurable thesaurus

### Stage 2: Parallel Retrieval

#### 2A. Lexical Retrieval (PostgreSQL)
- **tsvector + GIN index** on title + markdown body
- **BM25 scoring** with field weights: title (2.0), headings (1.5), body (1.0)
- **Phrase matching** for quoted queries
- **Prefix matching** for autocomplete scenarios

#### 2B. Semantic Retrieval (Vector Index)
- **Embedding model:** Configurable (default: 768-dim, compatible with `nomic-embed-text-v1.5`)
- **Vector index:** PostgreSQL `pgvector` with HNSW (Hierarchical Navigable Small World)
- **Distance metric:** Cosine similarity (normalized dot product)
- **Late interaction:** Optional ColBERT-style token-level similarity for re-ranking

### Stage 3: Fusion + Re-Ranking
1. **Deduplication** by note ID
2. **Reciprocal Rank Fusion (RRF)** for combining lexical + semantic rankings
3. **Neural re-ranking** (optional) — cross-encoder for top-K candidates
4. **Deterministic tie-breaking** by `updated_at DESC`, then `note_id ASC`

### Stage 4: Post-Processing
1. **Permission filtering** — remove inaccessible notes
2. **Snippet extraction** — context-aware highlights with query terms
3. **Backlink injection** — include incoming link count as boost signal

---

## Indexing Rules

### Lexical Index
- Title and markdown body MUST be indexed for lexical retrieval
- tsvector update MUST occur in same transaction as note write
- Trigger-based: `note_projections_tsv_trigger` on INSERT/UPDATE

### Vector Index
- Each searchable note revision MUST have an embedding vector payload
- Embeddings MUST be regenerated whenever title OR body changes
- **Batch reindexing** MUST be resumable and idempotent
- **Async queue** for embedding generation (non-blocking writes)

### Backlink Index
- Wiki links `[[target]]` MUST update backlink projections atomically
- Backlinks MUST converge within one committed write cycle
- Backlink reads MUST be permission-filtered to caller scope

---

## Query Contract

### Endpoint: `GET /search`

| Parameter | Required | Default | Description |
|-----------|----------|---------|-------------|
| `q` | yes | — | Search query string |
| `workspace_id` | yes | — | Scope to workspace |
| `project_id` | no | — | Further scope to project |
| `limit` | no | 20 | Max results (1-100) |
| `mode` | no | `hybrid` | `hybrid`, `lexical`, `semantic` |
| `offset` | no | 0 | Pagination offset |

### Response Shape

```json
{
  "results": [
    {
      "note_id": "uuid",
      "title": "string",
      "snippet": "string with <mark>highlights</mark>",
      "score_lexical": 0.0,
      "score_semantic": 0.0,
      "score_final": 0.0,
      "backlink_count": 0,
      "updated_at": "ISO8601"
    }
  ],
  "total": 0,
  "mode": "hybrid",
  "degraded": false,
  "degraded_reason": null
}
```

### Endpoint: `GET /notes/{id}/backlinks`

Returns all notes that link TO the specified note, sorted by `updated_at DESC`.

---

## Ranking Contract — Reciprocal Rank Fusion

**Hybrid mode MUST:**

1. Fetch top-N lexical candidates (N = limit × 2)
2. Fetch top-N semantic candidates (N = limit × 2)
3. Deduplicate by note ID
4. Compute RRF score: `RRF = Σ 1 / (k + rank_i)` where k=60 (standard)
5. Return stable order for equal scores by `updated_at DESC`, then `note_id ASC`

**Formula:**
```
lexical_rank = position in lexical results (1-indexed)
semantic_rank = position in semantic results (1-indexed)
RRF_score = 1/(60 + lexical_rank) + 1/(60 + semantic_rank)
final_rank = sort by RRF_score DESC
```

---

## Embedding Provider Contract

### Supported Providers

| Provider | Kind | Base URL | Use Case |
|----------|------|----------|----------|
| **LMStudio** | `lmstudio` | `http://127.0.0.1:1234/v1` | Local models |
| **OpenRouter** | `openrouter` | `https://openrouter.ai/api/v1` | Cloud models |
| **Stub** | `stub` | N/A | Testing (deterministic) |
| **Null** | `null` | N/A | Disabled semantic search |

### API Shape (OpenAI-Compatible)

**Request:**
```json
POST /v1/embeddings
{
  "model": "text-embedding-nomic-embed-text-v1.5",
  "input": ["text to embed"]
}
```

**Response:**
```json
{
  "data": [
    { "embedding": [0.1, 0.2, ...] }
  ]
}
```

### Degradation Contract

- If embedding service is unavailable, lexical search MUST continue
- Response MUST include `degraded: true` and `degraded_reason` machine code
- Semantic failure codes: `EMBEDDING_UNAVAILABLE`, `EMBEDDING_TIMEOUT`, `EMBEDDING_INVALID`

---

## Backlinks

### Storage Schema

```sql
CREATE TABLE backlinks (
    source_note_id  UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    target_note_id  UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    created_at      TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (source_note_id, target_note_id)
);
```

### Update Rules

- Backlinks MUST update atomically with note write
- Parse `[[note title]]` and `[[note_id|alias]]` syntax
- Resolve by title first, then by ID if ambiguous
- Orphaned backlinks (target deleted) MUST be removed on target deletion

---

## Safety and Scope Rules

- Search results MUST be filtered by workspace and authorization scope
- Search MUST include `settings` and media-note metadata where relevant
- Search MUST include kjxlkj-agent-generated note updates
- Search MUST NOT leak inaccessible note existence through score counts

---

## Failure and Fallback Rules

| Failure Mode | Fallback Behavior |
|--------------|-------------------|
| Embedding service timeout | Return lexical-only with `degraded: true` |
| Vector index unavailable | Return lexical-only with diagnostic |
| Reindex job failure | Retry with exponential backoff (max 3) |
| pgvector extension missing | Disable semantic mode, return 422 for `mode=semantic` |

### Reindex Contract

- Reindex jobs MUST be resumable (checkpoint by note_id)
- Reindex jobs MUST be idempotent (upsert by note_id)
- Reindex batch size configurable (default: 200)

---

## Performance Targets

| Metric | Target (P95) |
|--------|--------------|
| Lexical search latency | < 50ms at 100k notes |
| Semantic search latency | < 150ms (embedding + HNSW) |
| Hybrid search latency | < 200ms (fusion + re-rank) |
| Backlink query latency | < 30ms |

---

## Related

- [Notes model](notes.md) — note lifecycle
- [API HTTP contract](/docs/spec/api/http.md) — search endpoint
- [Performance targets](/docs/spec/technical/performance.md) — latency budgets
- [Migration contract](/docs/spec/technical/migrations.md) — schema + indexes
