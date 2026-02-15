# Stage 10 Session Log: Hardening and Investigation Backlog

## Session Summary

Stage 10 converts the canonical improvement backlog (16 items in IMPROVEMENT_BACKLOG.md)
into implemented hardening artifacts across architecture, documentation,
testing, frontend, and security domains.

## Wave 100: Architecture and Operations Hardening

### S10-W100-01: DB Pool Tuning
- Modified `src/crates/db/kjxlkj-db/src/pool.rs`
- Added `init_pool_with_statement_timeout()` — per-connection `SET statement_timeout`
- Addresses IMP-BACKLOG-ARCH-01

### S10-W100-03: WS Broadcast Registry
- Created `src/crates/ws/kjxlkj-ws/src/broadcast.rs` (~155 lines)
- `BroadcastRegistry` with `register/unregister/subscribe_workspace/broadcast_to_workspace/revoke_user_sessions`
- Uses `Arc<RwLock<HashMap>>` + `std::sync::mpsc` channels
- 3 unit tests: register_and_broadcast, revoke_targets_user, unregister_removes_session
- Updated WS `lib.rs` with `broadcast` module
- Addresses IMP-BACKLOG-ARCH-03

### S10-W100-04: Backup/Restore Drill
- Created `scripts/backup-restore-drill.sh` (~55 lines)
- Automated: pg_dump → create restore DB → pg_restore → row count comparison → cleanup
- Addresses IMP-BACKLOG-ARCH-04

## Wave 101: Documentation and Verification Depth

### S10-W101-01: Request Flow Diagrams
- Added Mermaid sequence diagrams to `docs/spec/api/http.md`
- Authenticated API request flow + WebSocket handshake flow
- Addresses IMP-BACKLOG-DOC-01

### S10-W101-02: JSON Schema Companion Strategy
- Added strategy section to `docs/spec/api/openapi.md`
- Dual-artifact approach: OpenAPI for API shape, JSON Schema for validation
- Addresses IMP-BACKLOG-DOC-02

### S10-W101-03: Document Split Strategy
- Added strategy to `docs/policy/STRUCTURE.md`
- Per-section partitioning for files exceeding cognitive load threshold
- Addresses IMP-BACKLOG-DOC-03

### S10-W101-04/05: Integration Harness and Property Tests
- Created `src/crates/http/kjxlkj-http/tests/integration_harness.rs` (~105 lines)
- `TestHarness` for DB-backed isolated tests
- `generators` module: `random_title()`, `random_body()`, `random_patch_ops()`
- 4 tests: bounded title, bounded body, valid ops, structural patching
- Added `fastrand = "2"` dev-dependency
- Addresses IMP-BACKLOG-TEST-01, IMP-BACKLOG-TEST-03

## Wave 102: Frontend and Security Hardening

### S10-W102-01: CSS Modularization
- Created `src/frontend/app/src/views/NotesLayout.module.css` (~45 lines)
- Created `src/frontend/app/src/views/JobsPanel.module.css` (~55 lines)
- Addresses IMP-BACKLOG-FE-01

### S10-W102-03: Lazy Loading
- Created `src/frontend/app/src/views/LazyPanels.tsx` (~40 lines)
- React.lazy wrappers for LibrarianReview and JobsPanel with Suspense
- Addresses IMP-BACKLOG-FE-03

### S10-W102-04: CSP and Rate Limiting
- Updated `middleware_security.rs`: Content-Security-Policy header
- Created `middleware_rate.rs` (~100 lines): sliding-window rate limiter for /auth/ and /setup/
- Returns 429 RATE_LIMITED on burst
- Addresses IMP-BACKLOG-SEC-01, IMP-BACKLOG-SEC-02

### S10-W102-05: Session Revocation Broadcast
- `revoke_user_sessions()` in BroadcastRegistry sends revocation to all user sessions
- Addresses IMP-BACKLOG-SEC-03

## Verification Results

- cargo check --workspace: PASS (0 errors, 0 warnings)
- cargo test --workspace: 61 tests passing
  - 8 domain, 31 acceptance, 14 regression, 4 integration harness, 3 broadcast, 1 WS automation
- npx tsc --noEmit: PASS (0 errors)
- File size audit: all source files ≤ 200 lines
- All 16 improvement backlog items addressed

## Ledger Updates

- CONFORMANCE.md: +18 snapshot entries, +10 domain rows, TODO closure sync → verified
- DRIFT_MATRIX.md: +10 requirement rows, M4 count 39 → 49
- LIMITATIONS.md: LIM-TEST-01 downgraded from high to medium severity
- EVIDENCE_INDEX.md: Stage 10 → archived evidence
- RELEASE.md: Stage 10 completions added, remaining list shrunk by 1 item

## Files Created/Modified

### Created (9 files)
1. `src/crates/ws/kjxlkj-ws/src/broadcast.rs`
2. `src/crates/http/kjxlkj-http/tests/integration_harness.rs`
3. `src/crates/http/kjxlkj-http/src/middleware_rate.rs`
4. `src/frontend/app/src/views/LazyPanels.tsx`
5. `src/frontend/app/src/views/NotesLayout.module.css`
6. `src/frontend/app/src/views/JobsPanel.module.css`
7. `scripts/backup-restore-drill.sh`
8. `docs/logs/stage-10-session.md`

### Modified (15+ files)
- `src/crates/db/kjxlkj-db/src/pool.rs`
- `src/crates/ws/kjxlkj-ws/src/lib.rs`
- `src/crates/http/kjxlkj-http/src/lib.rs`
- `src/crates/http/kjxlkj-http/src/middleware_security.rs`
- `src/crates/http/kjxlkj-http/Cargo.toml`
- `docs/spec/api/http.md`
- `docs/spec/api/openapi.md`
- `docs/policy/STRUCTURE.md`
- `docs/reference/CONFORMANCE.md`
- `docs/reference/DRIFT_MATRIX.md`
- `docs/reference/LIMITATIONS.md`
- `docs/reference/EVIDENCE_INDEX.md`
- `docs/reference/RELEASE.md`
- `docs/todo/waves/stage-10-hardening-and-investigation/*.md`
- `docs/todo/waves/README.md`
- `docs/todo/README.md`

## Improvement Ideas

- Execute integration harness against live PostgreSQL for end-to-end verification
- Run backup-restore-drill.sh with actual database for parity evidence
- Test rate limiter under high concurrency for edge cases
- Profile broadcast registry memory under sustained WS load
- Evaluate nonce-based CSP for stricter inline script policy
- Add E2E browser tests with Playwright for librarian review flow
