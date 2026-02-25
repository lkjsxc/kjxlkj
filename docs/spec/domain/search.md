# Search and Backlinks — Next-Generation Hybrid Pipeline

**Back:** [Domain Root](/docs/spec/domain/README.md)

---

## Search Model — State-of-the-Art Retrieval

Search MUST use a **four-stage neural retrieval pipeline** with modern vectorization and fusion techniques:

### Stage 1: Query Understanding and Expansion

#### 1.1 Query Normalization
- **Unicode NFC normalization** — canonical form for all text
- **Case folding** — lowercase for lexical matching
- **Diacritic stripping** — remove accents for broader matching
- **Punctuation normalization** — standardize special characters

#### 1.2 Intent Classification
- **Navigational** — user seeks specific note (high confidence in title match)
- **Informational** — user seeks content on topic (broad semantic match)
- **Transactional** — user seeks actionable content (filter by note-type)

#### 1.3 Query Expansion
- **Synonym injection** — configurable thesaurus (e.g., "meeting" → "sync,standup,review")
- **Hyponym expansion** — include specific instances (e.g., "database" → "postgres,mysql")
- **Acronym resolution** — expand common acronyms (e.g., "API" → "application programming interface")
- **Spelling correction** — Levenshtein-based fuzzy match for typos (max edit distance: 2)

---

### Stage 2: Parallel Retrieval

#### 2A. Lexical Retrieval (PostgreSQL BM25)

**Index Structure:**
```sql
-- Title + body tsvector with field weights
CREATE INDEX note_projections_tsv_idx ON note_projections USING GIN (tsv);

-- Title-only index for navigational queries
CREATE INDEX note_projections_title_tsv_idx ON note_projections USING GIN (title_tsv);
```

**Scoring Formula:**
```
lexical_score = (
  ts_rank(title_tsv, query) * 2.0 +      -- Title weight
  ts_rank(headings_tsv, query) * 1.5 +   -- Heading weight
  ts_rank(body_tsv, query) * 1.0         -- Body weight
)
```

**Features:**
- **Phrase matching** — exact phrase queries with `<->` operator
- **Prefix matching** — autocomplete with `:*` suffix
- **Proximity search** — `<N>` operator for word proximity
- **Boost recent** — time-decay factor for `updated_at`

#### 2B. Semantic Retrieval (Vector Index)

**Embedding Model:**
- **Default:** `nomic-embed-text-v1.5` (768-dim, 8192 context)
- **Alternative:** `mxbai-embed-large-v1` (1024-dim, 512 context)
- **Provider:** OpenAI-compatible API (LMStudio, OpenRouter, Ollama)

**Index Structure:**
```sql
-- HNSW index for approximate nearest neighbor search
CREATE INDEX note_embeddings_vector_idx ON note_embeddings
USING hnsw (vector vector_cosine_ops)
WITH (m = 16, ef_construction = 64);
```

**Distance Metric:**
- **Cosine similarity** — normalized dot product (default)
- **Inner product** — for unnormalized embeddings
- **L2 distance** — Euclidean distance (alternative)

**Advanced Techniques:**

1. **Late Interaction (ColBERT-style):**
   - Store token-level embeddings (max 128 tokens)
   - Compute MaxSim score at query time
   - Re-rank top-100 candidates with fine-grained similarity

2. **HyDE (Hypothetical Document Embeddings):**
   - Generate hypothetical answer embedding
   - Use as query vector for semantic search
   - Improves recall for question queries

3. **Query Multi-Vector:**
   - Embed query with multiple augmentations
   - Average or max-pool embeddings
   - Robust to query phrasing variations

#### 2C. Metadata Filtering

**Pre-Filter (before retrieval):**
- `workspace_id` — mandatory scope
- `project_id` — optional narrowing
- `note_kind` — filter by type (note, template, etc.)
- `access_scope` — permission-based filtering

**Post-Filter (after retrieval):**
- `updated_at` — recency threshold
- `backlink_count` — popularity threshold
- `word_count` — length constraints

---

### Stage 3: Fusion and Re-Ranking

#### 3.1 Deduplication
- Merge results by `note_id`
- Keep highest score per note
- Preserve rank from both lexical and semantic lists

#### 3.2 Reciprocal Rank Fusion (RRF)

**Formula:**
```
RRF_score(note) = Σ 1 / (k + rank_i)

where:
  k = 60 (standard constant, balances lexical vs semantic)
  rank_i = position in result list i (1-indexed)
```

**Example:**
```
Note A: lexical_rank=5, semantic_rank=10
RRF = 1/(60+5) + 1/(60+10) = 0.0154 + 0.0143 = 0.0297

Note B: lexical_rank=3, semantic_rank=50
RRF = 1/(60+3) + 1/(60+50) = 0.0159 + 0.0091 = 0.0250

Result: A ranks higher despite B's better lexical rank
```

#### 3.3 Neural Re-Ranking (Optional)

**Cross-Encoder Model:**
- **Model:** `ms-marco-MiniLM-L-6-v2` or similar
- **Input:** (query, document) pair
- **Output:** Relevance score (0-1)
- **Apply to:** Top-K candidates (K=20 default)

**Features for Re-Ranker:**
- Query-document token overlap
- Semantic similarity score
- Lexical BM25 score
- Recency (days since update)
- Backlink count
- Note length (penalize very short/long)

#### 3.4 Final Sorting

**Primary Sort:** `score_final DESC`
**Tie-Breaker 1:** `updated_at DESC` (recent first)
**Tie-Breaker 2:** `note_id ASC` (deterministic)

---

### Stage 4: Post-Processing and Presentation

#### 4.1 Permission Filtering
- Remove notes outside user's access scope
- Apply workspace and project filters
- Respect RBAC permissions

#### 4.2 Snippet Extraction

**Algorithm:**
1. Find query term positions in document
2. Extract context window (±50 characters)
3. Highlight query terms with `<mark>` tags
4. Truncate to max 200 characters
5. Prefer snippet with most query term matches

**Example:**
```json
{
  "snippet": "...discussed in the <mark>meeting</mark> about project timeline...",
  "highlight_positions": [23, 30]
}
```

#### 4.3 Backlink Injection
- Count incoming links to each result
- Boost score by `log(1 + backlink_count)`
- Display backlink count in UI

#### 4.4 Response Assembly

```json
{
  "results": [
    {
      "note_id": "uuid",
      "title": "Meeting Notes 2026-02-24",
      "snippet": "Discussed project timeline in the <mark>meeting</mark>...",
      "score_lexical": 8.5,
      "score_semantic": 0.87,
      "score_rrf": 0.0297,
      "score_rerank": 0.92,
      "score_final": 0.92,
      "backlink_count": 5,
      "updated_at": "2026-02-24T14:30:00Z",
      "note_kind": "note",
      "workspace_id": "uuid"
    }
  ],
  "total": 1,
  "mode": "hybrid",
  "degraded": false,
  "degraded_reason": null,
  "query_normalized": "meeting notes",
  "query_expanded": ["meeting notes", "sync notes", "standup notes"]
}
```

---

## Indexing Rules

### Lexical Index Maintenance

**Trigger-Based Update:**
```sql
CREATE OR REPLACE FUNCTION update_note_tsv() RETURNS trigger AS $$
BEGIN
  NEW.tsv :=
    setweight(to_tsvector('english', COALESCE(NEW.title, '')), 'A') ||
    setweight(to_tsvector('english', COALESCE(NEW.headings, '')), 'B') ||
    setweight(to_tsvector('english', COALESCE(NEW.markdown, '')), 'C');
  NEW.title_tsv := to_tsvector('english', COALESCE(NEW.title, ''));
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER note_projections_tsv_trigger
BEFORE INSERT OR UPDATE ON note_projections
FOR EACH ROW EXECUTE FUNCTION update_note_tsv();
```

**Transaction Guarantee:**
- TSV update MUST occur in same transaction as note write
- No orphaned or stale lexical indexes allowed

### Vector Index Maintenance

**Embedding Generation:**
```
Note Write → Queue Embedding Job → Call Embedding API → Update Vector Index
```

**Rules:**
- Embeddings MUST regenerate on title OR body change
- Batch reindexing MUST be resumable (checkpoint by `note_id`)
- Batch reindexing MUST be idempotent (upsert by `note_id`)
- Async queue prevents blocking writes

**Embedding Content:**
```
embedding_input = title + "\n\n" + first_2000_chars(markdown)
```

**Vector Storage:**
```sql
CREATE TABLE note_embeddings (
    note_id       UUID PRIMARY KEY REFERENCES note_streams(id) ON DELETE CASCADE,
    vector        vector(768),  -- or 1024 depending on model
    model_name    TEXT NOT NULL,
    model_version TEXT NOT NULL,
    indexed_at    TIMESTAMP NOT NULL DEFAULT now()
);
```

### Backlink Index

**Storage Schema:**
```sql
CREATE TABLE backlinks (
    source_note_id  UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    target_note_id  UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    created_at      TIMESTAMP NOT NULL DEFAULT now(),
    link_text       TEXT,  -- anchor text from wiki-link
    PRIMARY KEY (source_note_id, target_note_id)
);

CREATE INDEX backlinks_target_idx ON backlinks(target_note_id);
```

**Update Rules:**
- Parse `[[target]]` and `[[target|alias]]` syntax
- Resolve by title first, then by ID if ambiguous
- Update atomically with note write
- Remove orphaned backlinks on target deletion

---

## Query Contract

### Endpoint: `GET /api/search`

**Query Parameters:**

| Parameter | Required | Default | Validation | Description |
|-----------|----------|---------|------------|-------------|
| `q` | yes | — | 1-500 chars | Search query string |
| `workspace_id` | yes | — | UUID | Scope to workspace |
| `project_id` | no | — | UUID | Further scope to project |
| `limit` | no | 20 | 1-100 | Max results |
| `offset` | no | 0 | ≥0 | Pagination offset |
| `mode` | no | `hybrid` | enum | `hybrid`, `lexical`, `semantic` |
| `note_kind` | no | — | enum | Filter by note type |
| `sort` | no | `relevance` | enum | `relevance`, `updated_at` |

**Response Shape:**

```json
{
  "results": [],
  "total": 0,
  "mode": "hybrid",
  "degraded": false,
  "degraded_reason": null,
  "query_normalized": "",
  "query_expanded": [],
  "timing_ms": {
    "lexical": 12,
    "semantic": 85,
    "fusion": 3,
    "rerank": 45,
    "total": 145
  }
}
```

### Endpoint: `GET /api/notes/{id}/backlinks`

**Response:**
```json
{
  "note_id": "uuid",
  "backlinks": [
    {
      "source_note_id": "uuid",
      "source_title": "Linking Note",
      "link_text": "custom alias",
      "snippet": "...context with [[Linking Note|custom alias]] reference...",
      "updated_at": "2026-02-24T14:30:00Z"
    }
  ],
  "total": 1
}
```

---

## Embedding Provider Contract

### Supported Providers

| Provider | Kind | Base URL | Auth | Use Case |
|----------|------|----------|------|----------|
| **LMStudio** | `lmstudio` | `http://127.0.0.1:1234/v1` | None | Local models |
| **Ollama** | `ollama` | `http://127.0.0.1:11434/api` | None | Local models |
| **OpenRouter** | `openrouter` | `https://openrouter.ai/api/v1` | Bearer | Cloud models |
| **OpenAI** | `openai` | `https://api.openai.com/v1` | Bearer | Cloud models |
| **Stub** | `stub` | N/A | N/A | Testing (deterministic) |
| **Null** | `null` | N/A | N/A | Disabled semantic search |

### API Shape (OpenAI-Compatible)

**Request:**
```json
POST /v1/embeddings
{
  "model": "nomic-embed-text-v1.5",
  "input": ["text to embed"],
  "encoding_format": "float"
}
```

**Response:**
```json
{
  "data": [
    {
      "object": "embedding",
      "index": 0,
      "embedding": [0.1, 0.2, ...]
    }
  ],
  "model": "nomic-embed-text-v1.5",
  "usage": {
    "prompt_tokens": 10,
    "total_tokens": 10
  }
}
```

### Degradation Contract

**Failure Modes:**

| Failure | Detection | Fallback | Response Code |
|---------|-----------|----------|---------------|
| Embedding service timeout | 30s timeout | Lexical-only | 200 (degraded) |
| Embedding service unavailable | Connection refused | Lexical-only | 200 (degraded) |
| Invalid embedding response | Schema validation fail | Lexical-only | 200 (degraded) |
| pgvector extension missing | SQL error on vector query | Lexical-only | 200 (degraded) |
| Semantic mode requested + unavailable | Mode check | 422 error | 422 |

**Response on Degradation:**
```json
{
  "degraded": true,
  "degraded_reason": "EMBEDDING_UNAVAILABLE",
  "mode": "lexical",
  "message": "Semantic search unavailable, returning lexical results only"
}
```

---

## Performance Targets

| Metric | P50 | P95 | P99 |
|--------|-----|-----|-----|
| Lexical search latency | 20ms | 50ms | 100ms |
| Semantic search latency | 50ms | 150ms | 300ms |
| Hybrid search (no rerank) | 80ms | 200ms | 400ms |
| Hybrid search (with rerank) | 120ms | 300ms | 600ms |
| Backlink query latency | 10ms | 30ms | 60ms |
| Reindex batch (100 notes) | 5s | 15s | 30s |

**Scale Targets:**
- 100k notes: P95 < 500ms hybrid search
- 1M notes: P95 < 1s hybrid search (with sharding)

---

## Safety and Scope Rules

### Permission Enforcement
- Search results MUST be filtered by workspace scope
- Search MUST respect `access_scope` (private, workspace, public)
- Search MUST NOT leak inaccessible note existence through scores

### Content Inclusion
- Search MUST include note title and markdown body
- Search MAY include metadata fields (configurable)
- Search MUST include kjxlkj-agent-generated notes
- Search MUST exclude soft-deleted notes (unless `include_deleted=true`)

### Query Sanitization
- Max query length: 500 characters
- Strip potentially dangerous characters (SQL injection prevention)
- Rate limit: 10 queries/second per user

---

## Failure and Fallback Rules

### Reindex Job Contract

**Resumability:**
```json
{
  "job_id": "uuid",
  "status": "running",
  "checkpoint_note_id": "uuid",
  "processed_count": 5000,
  "total_count": 10000,
  "started_at": "2026-02-24T10:00:00Z",
  "updated_at": "2026-02-24T10:05:00Z"
}
```

**Retry Policy:**
- Max retries: 3
- Backoff: exponential (1s, 2s, 4s)
- On final failure: mark job as failed, alert operator

### Health Checks

**Embedding Provider Health:**
```
GET /health/embedding

Response:
{
  "status": "healthy",
  "provider": "lmstudio",
  "latency_ms": 45,
  "last_check": "2026-02-24T14:30:00Z"
}
```

**Vector Index Health:**
```
GET /health/vector-index

Response:
{
  "status": "healthy",
  "indexed_notes": 9850,
  "total_notes": 10000,
  "coverage": 0.985
}
```

---

## Related

- [Notes model](notes.md) — note lifecycle
- [API HTTP contract](/docs/spec/api/http.md) — search endpoint
- [Performance targets](/docs/spec/technical/performance.md) — latency budgets
- [Migration contract](/docs/spec/technical/migrations.md) — schema + indexes
- [Automation](automation.md) — kjxlkj-agent note creation
