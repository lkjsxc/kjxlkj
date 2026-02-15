# Stage 06 Session Log — REST and Librarian Provider Completion

Date: 2026-02-15

## Scope

Stage 06 closes REST contract completeness and librarian provider/protocol
integration per three waves:

- Wave 060: REST surface parity and OpenAPI sync
- Wave 061: Librarian provider adapters and JSON prompt pack loading
- Wave 062: xml_attrless parse, retry, and apply safety

## Deliverables

### Wave 060 — REST Surface Parity

| File | Lines | Purpose |
|---|---:|---|
| `repo_view.rs` | 98 | Saved views list/create/update/delete |
| `repo_dashboard.rs` | 62 | Dashboard widgets list/upsert |
| `dto_views.rs` | 84 | DTOs for views, dashboards, projects, media notes |
| `routes_views.rs` | 146 | GET/POST/PATCH/DELETE /views |
| `routes_dashboards.rs` | 85 | GET /dashboards, POST /dashboards/widgets |
| `routes_projects.rs` | 61 | PATCH/DELETE /projects/{id} |
| `routes_notes_media.rs` | 78 | POST /notes/media |
| `startup.rs` | 140 | All routes wired including new views, dashboards, projects, media |

New routes now cover full HTTP spec surface:
- Views: list, create, update, delete
- Dashboards: list, upsert widget (optional extension)
- Projects: update, delete (added to existing list/create)
- Notes: media note creation (POST /notes/media)

### Wave 061 — Provider Adapters and Prompt Loading

| File | Lines | Purpose |
|---|---:|---|
| `provider.rs` | 173 | Provider adapter: chat_completion, chat_with_fallback, ProviderConfig, ProviderFailure |
| `prompt_loader.rs` | 127 | Manifest loader, stage validation, template rendering, pack hash |

- OpenRouter: HTTPS bearer token auth, fallback model chain
- LMStudio: local endpoint (default http://127.0.0.1:1234/v1)
- Deterministic failure categories: auth_failed, rate_limited, timeout, unreachable, invalid_payload
- Pack hash: SHA-256 over included stage files per manifest.hash_policy

### Wave 062 — xml_attrless Parser and Safety

| File | Lines | Purpose |
|---|---:|---|
| `xml_types.rs` | 66 | LibrarianResponse, ParsedOperation, ParseError types |
| `xml_parser.rs` | 150 | Tag extraction, attribute rejection, operation validation |
| `safety_policy.rs` | 107 | Delete prevention, cross-workspace rejection, scope constraints |
| `pipeline_retry.rs` | 80 | Bounded repair retries (max 2), stage call helper |
| `run_pipeline.rs` | 160 | Full pipeline: ingest → plan → propose → validate → safety |

## Test Results

```
cargo test --workspace
  46 tests total (8 domain + 14 regression + 24 acceptance) — all passed
  0 errors, 0 warnings
```

## 200-Line Compliance

All files under 200 lines:
- Max: provider.rs at 173 lines
- Split: xml_parser.rs → xml_types.rs + xml_parser.rs
- Split: run_pipeline.rs → pipeline_retry.rs + run_pipeline.rs

## Design Decisions

1. **Unified provider adapter**: Both openrouter and lmstudio use OpenAI-compatible
   chat/completions endpoint. Single `chat_completion` function handles both with
   optional bearer auth for openrouter.

2. **Fallback chain**: Primary model failure triggers sequential fallback to
   configured model IDs per spec.

3. **Tag-only XML extraction**: Simple string-based extraction (no XML library)
   per spec's simplicity requirement. Rejects attributes by checking character
   after tag name.

4. **Bounded repair**: At most 2 retries using validate_repair stage prompt
   with parse diagnostics as input.

5. **Safety-first evaluation**: Accepted/rejected split occurs before any
   database writes. Default policy prevents deletion operations.

## Ledger Updates

- CONFORMANCE.md: added 8 domain rows (views, dashboards, media, provider, prompt,
  xml parser, pipeline)
- DRIFT_MATRIX.md: added 7 rows; M4 count 19→26
- LIMITATIONS.md: added 2 rows (LIM-PROVIDER-01, LIM-XML-PARSER-01); baseline updated
