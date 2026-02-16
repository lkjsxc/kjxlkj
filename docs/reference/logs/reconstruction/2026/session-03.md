# Session 03 — Reconstruction Log

Date: 2026-02-17

## Summary

Session cookie management, CSRF validation, YOLO scope guardrails, rich editor UI, rate limiting, and attachment system.

## Changes

1. **Session cookie**: `auth_login` now sets `Set-Cookie` header with HttpOnly, SameSite=Lax, Path=/, Max-Age=604800. `auth_logout` clears cookie with Max-Age=0.
2. **CSRF token**: `csrf_token` field added to `SessionRecord`, generated on login, returned to client. CSRF middleware validates `X-CSRF-Token` header against session-bound token for browser mutations. Bearer-token API requests are exempt.
3. **YOLO scope guardrails**: `AgentLoop` now carries `workspace_id` and `yolo_mode` fields. `check_workspace_scope()` rejects cross-workspace writes (returns `AgentYoloPolicyViolation` / 403).
4. **Integration test split**: Tests split into api_integration.rs, auth_integration.rs, csrf_integration.rs, auto_integration.rs, all under 200 lines. Shared helpers in test_helpers.rs.
5. **Automation repo fix**: `list_rules`/`list_runs` with nil workspace_id now returns all (no filter).
6. **Rich editor UI**: Added markdown.ts (wiki-link extraction, markdown-to-HTML, keyboard shortcuts, formatting), preview.ts (split/toggle preview, backlinks detection), note-list.ts (search/filter, create+select, title propagation), conflict.ts (conflict state, resolution, cursor preservation). Editor.ts enhanced with keyboard shortcut integration and cursor preservation.
7. **Frontend tests**: 4 new test files (markdown, preview, note-list, conflict) with 55+ assertions. All 7 frontend test files pass.
8. **Rate limiting (IMP-SEC-02)**: Sliding window per-IP rate limiter for auth endpoints (10 req/60s default). Returns 429 with Retry-After header. Integration test verifies rejection on 3rd request with max_requests=2.
9. **Attachment system**: AttachmentRepo trait with full CRUD. InMemoryAttachmentRepo with RwLock<HashMap> for metas/chunks, sorted chunk retrieval, 5 unit tests. HTTP routes: POST/GET /api/notes/:id/attachments, GET /api/attachments/:id/download, DELETE /api/attachments/:id. Base64 body upload with SHA-256 checksum, chunked 4 MiB storage, 413 for >500 MiB payloads.

## Metrics

- Rust tests: 100 passing, 0 failures, 0 warnings
- Frontend TS tests: 7 files, 55+ assertions, all pass
- Open M2 (missing feature): 0
- Files over 200 lines: 0
