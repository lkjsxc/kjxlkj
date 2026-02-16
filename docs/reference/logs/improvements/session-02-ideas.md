# Improvement Ideas — Session 02

## Completed

- [x] IMP-SQL-01: SQL migrations for all schema domains (8 files)
- [x] IMP-REPO-01: In-memory repository implementations for all traits
- [x] IMP-STATE-01: AppState DI with Arc-wrapped services
- [x] IMP-WIRE-01: All HTTP handlers wired to real repositories
- [x] IMP-LLM-01: LlmProvider trait + OpenRouter/LMStudio stubs
- [x] IMP-WS-01: WebSocket cursor replay and idempotency dedup
- [x] IMP-WKS-01: Workspace slug validation and state transitions
- [x] IMP-T1-01: T1 integration tests via AppState + tower oneshot
- [x] IMP-ROUTE-01: Fixed matchit 0.7 path param syntax (:id vs {id})
- [x] IMP-CI-01: GitHub Actions CI workflow
- [x] IMP-SCRIPT-01: scripts/check.sh and scripts/migrate.sh

## Backlog

- [x] IMP-CSRF-01: CSRF token validation middleware
- [x] IMP-SESSION-01: Session extraction middleware from cookie/header
- [ ] IMP-PG-01: PostgreSQL repository implementations via sqlx
- [ ] IMP-EMBED-01: Embedding provider integration for semantic search
- [ ] IMP-E2E-01: E2E acceptance tests (E2E-06/12/17/19/23/24/25)
- [x] IMP-ATTACH-01: Attachment upload/download streaming endpoints
- [ ] IMP-EXPORT-01: Export and backup job endpoints
- [x] IMP-RATE-01: Rate limiting middleware for auth endpoints
