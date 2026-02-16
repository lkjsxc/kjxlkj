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
10. **Structured tracing (IMP-OPS-01)**: tracing_mw.rs middleware with request_id span, method, path, workspace_id, latency logging. Error/warn/info log levels by status. x-request-id header propagation. Integrated with Metrics recording.
11. **Graceful shutdown (IMP-OPS-03)**: shutdown_signal() in main.rs awaits SIGTERM or Ctrl-C, triggers graceful shutdown with in-flight drain via axum::serve::with_graceful_shutdown.
12. **CSP nonce (IMP-SEC-01)**: csp.rs middleware adds Content-Security-Policy header with per-request nonce for script-src, frame-ancestors none, base-uri self, form-action self.
13. **WS broadcast registry (IMP-ARC-02)**: broadcast.rs in kjxlkj-ws — BroadcastRegistry with per-stream tokio::broadcast channels. Subscribe/broadcast for note and workspace streams. GC for stale channels. 5 async tests.
14. **Session revocation (IMP-SEC-03)**: revoke_user_sessions method on SessionRepo trait + InMemorySessionRepo. Removes all sessions for a user_id. Test verifies selective revocation.
15. **Metrics endpoint (IMP-OPS-02)**: metrics.rs — Metrics struct with atomic counters (total_requests, 4xx, 5xx, avg_latency_us). GET /api/metrics endpoint. Recording integrated into tracing middleware.
16. **Property-based tests (IMP-TEST-01)**: 11 tests verifying domain invariants — NoteKind/SearchMode round-trip, error status codes valid, SCREAMING_SNAKE_CASE codes, version conflict always 409, serde determinism, attachment constants.
17. **Snapshot tests (IMP-TEST-02)**: 9 tests verifying API response shapes — healthz, readyz, create note, list notes, search, session, register, metrics, error envelope.
18. **DB pool config (IMP-ARC-03)**: Validation test for DatabaseConfig — max≥min, positive timeouts, ≥10 connections for 100 concurrent sessions.
19. **Offline/PWA (IMP-FE-03)**: offline.ts — ConnectionState, PendingDraft queue, service worker registration, online/offline event watching, connection badge rendering. Integrated into AppState. 10 new test assertions.
20. **DB test harness (IMP-TEST-03)**: db_test_harness.rs — containerless integration harness with DATABASE_URL env, DbTestConfig, require_db() skip helper. 3 tests.
21. **Export domain + repo + routes (IMP-EXPORT-01)**: ExportJob lifecycle (queued→running→succeeded/failed), InMemoryExportRepo (5 tests), routes_export.rs (POST/GET /admin/export). ExportNotFound variant added to DomainError.
22. **E2E acceptance tests (IMP-E2E-01)**: 6 Rust E2E tests (E2E-06 autosave, E2E-17 conflict, E2E-23 create, API-SEARCH-03 degraded, API-AUTO-03 validation, export lifecycle). 5 TS E2E assertions (E2E-12 menu, E2E-19 320px, E2E-24 shortcuts, E2E-25 compact).
23. **SQLx offline cache (IMP-ARC-01)**: .sqlx/ directory with README, .env.example with SQLX_OFFLINE=true. Infrastructure ready for PG query cache generation.
24. **PG repos (IMP-PG-01)**: pg_note_repo.rs (full CRUD with optimistic locking), pg_user_repo.rs (user/session CRUD + revocation), pg_workspace_repo.rs (workspace CRUD). Shared pg_rows.rs with FromRow types and pg_err helper.
25. **Domain as_str methods**: AccessScope::as_str, NoteState::as_str, ActorType::as_str, NoteEventType::as_str for PG serialization.
26. **File structure compliance**: Split e2e_acceptance.rs (330 lines) → e2e_notes.rs + e2e_misc.rs. All source files verified ≤ 200 lines.

## Metrics

- Rust tests: 152 passing, 0 failures, 0 warnings
- Frontend TS tests: 9 files, 80+ assertions, all pass
- Open M2 (missing feature): 0
- Improvement backlog: 16 of 16 items done
- Files over 200 lines: 0
