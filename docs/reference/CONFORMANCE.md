# Conformance

**Back:** [Reference Root](/docs/reference/README.md)

---

## Status Vocabulary

| Status | Meaning |
|--------|---------|
| `verified` | Deterministic evidence exists |
| `partial` | Behavior exists but evidence incomplete |
| `spec-only` | Specified but not currently implemented |
| `blocked` | Contradicted or impossible in current state |

---

## Current Snapshot (Docs-Only Baseline)

**Repository state:** Docs-only baseline (source code deleted for clean rebuild)

- Docs governance and precedence are active
- Source code tree deleted — ready for reconstruction from specs
- TODO waves reset — all checkboxes unchecked
- Docker helpers removed (will be regenerated if needed)
- All specifications updated with latest requirements:
  - Redesigned search with modern vectorization (HNSW, RRF)
  - Obsidian-like markdown editor spec
  - 2/3 threshold menu toggle (1280px breakpoint)
  - kjxlkj-agent with JSON prompts + KV memory
  - Note ID/title separation enforced

---

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|--------|----------------|--------|----------|
| Docs governance | [Policy Root](/docs/policy/README.md) | `verified` | Policy tree present |
| Docs-only baseline | [Final Structure](/docs/spec/architecture/final-file-structure.md) | `verified` | State A (docs-only) active |
| TODO reset | [TODO Root](/docs/todo/README.md) | `verified` | All checkboxes reset to `[ ]` |
| Runtime crates | [Runtime Model](/docs/spec/architecture/runtime.md) | `spec-only` | Deleted — ready for S01 rebuild |
| HTTP API | [HTTP Contract](/docs/spec/api/http.md) | `spec-only` | To be implemented in S06 |
| WebSocket | [WS Contract](/docs/spec/api/websocket.md) | `spec-only` | To be implemented in S07 |
| Hybrid search | [Search Spec](/docs/spec/domain/search.md) | `spec-only` | Redesigned — to be implemented in S02 |
| Editor UX | [Editor Flow](/docs/spec/ui/editor-flow.md) | `spec-only` | Obsidian-like spec — to be implemented in S08 |
| kjxlkj-agent | [Agent Contract](/docs/spec/technical/librarian-agent.md) | `spec-only` | JSON prompts + KV — to be implemented in S04 |
| Auth/session | [Security Root](/docs/spec/security/README.md) | `spec-only` | To be implemented in S05 |
| Migrations | [Migrations](/docs/spec/technical/migrations.md) | `spec-only` | To be regenerated in S02 |

---

## Closure Rule

No row may move to `verified` without:

1. Deterministic test evidence
2. Runtime reachability from documented paths
3. Synchronized reference and TODO updates

---

## Related

- [Limitations](LIMITATIONS.md) — open gaps
- [Drift Matrix](DRIFT_MATRIX.md) — mismatch tracking
- [TODO Contract](/docs/todo/README.md) — execution order
